#!/usr/bin/env python3

'''
TODO module docstring
'''

import argparse
import csv
from copy import deepcopy
import math
import os
import matplotlib as mpl
import matplotlib.pyplot as plt
import numpy as np

from .styling import Style


class Simulation():
    '''
    Just a struct of values
    '''

    def __init__(self, results_dir, num_iter: int):
        self._results_dir = results_dir
        self._num_iter = num_iter

    def is_last_iteration(self, iteration):
        return iteration < self._num_iter - 1

    @property
    def results_dir(self):
        return self._results_dir

    @property
    def num_iter(self):
        return self._num_iter


class Workload():
    '''
    Just a struct of values
    '''

    def __init__(self):
        self.raw = []

    @property
    def raw(self):
        return self._raw

    @property
    def _raw_nz(self):
        return list(filter(lambda w: w > 0.0, self._raw))

    @raw.setter
    def raw(self, new_raw):
        self._raw = new_raw
        self._min = None
        self._max = None
        self._mean = None
        self._std = None

    @property
    def min(self):
        if self._min is None:
            self._min = np.min(self._raw)
        return self._min

    @property
    def max(self):
        if self._max is None:
            self._max = np.max(self._raw)
        return self._max

    @property
    def mean(self):
        if self._mean is None:
            self._mean = np.mean(self._raw)
        return self._mean

    @property
    def std(self):
        if self._std is None:
            self._std = np.std(self._raw)
        return self._std


class Data():
    '''
    Just a struct of values
    '''

    def __init__(self):
        self._iteration = None
        self._lats = []
        self._lons = []
        self._kilometers = []
        self._lane_counts = []
        self._old_workloads = Workload()
        self._workloads = Workload()
        self._delta_workloads = None

    def start_new_iteration(self, iteration: int):
        self._iteration = iteration

        tmp = self.old_workloads.raw
        self.old_workloads.raw = self.workloads.raw
        self.workloads.raw = tmp
        self.workloads.raw.clear()

        self._delta_workloads = None

    @ property
    def iteration(self):
        return self._iteration

    @ property
    def lats(self):
        return self._lats

    @ property
    def lons(self):
        return self._lons

    @ property
    def kilometers(self):
        return self._kilometers

    @ property
    def lane_counts(self):
        return self._lane_counts

    def volume(self, edge_idx: int) -> float:
        '''
        It's used for hopefully greater numbers

        Nagel-Schreckenberg-Model: 7.5 m per vehicle
        '''
        num_vehicles = max(1.0, self._kilometers[edge_idx] / 0.0075)
        return num_vehicles * self._lane_counts[edge_idx]

    @ property
    def old_workloads(self):
        return self._old_workloads

    @ property
    def workloads(self):
        return self._workloads

    @ property
    def delta_workloads(self):
        if self._delta_workloads is None:
            self._delta_workloads = Workload()
            for new, old in zip(self.workloads.raw, self.old_workloads.raw):
                self._delta_workloads.raw.append(new - old)
        return self._delta_workloads

    def read_in_edge_info(self, sim: Simulation):
        coords_csv_path = f'{sim.results_dir}/{self.iteration}/stats/edge-info.csv'
        with open(coords_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for row in csv_reader:
                src_lat = float(row['src-lat'])
                src_lon = float(row['src-lon'])
                dst_lat = float(row['dst-lat'])
                dst_lon = float(row['dst-lon'])
                kilometers = float(row['kilometers'])
                lane_count = float(row['lane_count'])
                # take mid-point of an edge as reference
                self.lats.append((src_lat + dst_lat) / 2.0)
                self.lons.append((src_lon + dst_lon) / 2.0)
                self.kilometers.append(kilometers)
                self.lane_counts.append(lane_count)

    def read_in_workloads(self, sim: Simulation):
        workloads_csv_path = f'{sim.results_dir}/{self._iteration}/stats/abs_workloads.csv'
        with open(workloads_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for edge_idx, row in enumerate(csv_reader):
                value = float(row['num_routes']) / self.volume(edge_idx)
                self.workloads.raw.append(value)

    def _read_in_new_metrics(self, sim: Simulation):
        workloads_csv_path = f'{sim.results_dir}/{self._iteration}/stats/new_metrics.csv'
        with open(workloads_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for row in csv_reader:
                value = float(row['new_metrics'])
                self.workloads.raw.append(value)


def plot_delta_workloads(data: Data, sim: Simulation, style: Style):
    '''
    TODO function docstring
    '''

    plt.style.use(style.plt)
    plt.figure()
    plt.title(f'delta-workloads {data.iteration}')
    plt.scatter(
        data.lons,
        data.lats,
        c=data.delta_workloads.raw,
        s=style.integer.s,
        alpha=style.integer.alpha,
        edgecolors=style.integer.edgecolors,
        cmap=style.integer.cmap,
        norm=style.integer.norm
    )
    plt.xlabel('longitude')
    plt.ylabel('latitude')
    plt.colorbar()
    plt.grid(False)
    plt.savefig(
        f'{sim.results_dir}/{data.iteration}/stats/delta_workloads.png')
    # plt.show()
    plt.close()


def plot_workloads(data: Data, sim: Simulation, style: Style):
    '''
    TODO
    '''

    plt.style.use(style.plt)
    plt.figure()
    plt.title(f'workloads {data.iteration}')
    plt.scatter(
        data.lons,
        data.lats,
        c=data.workloads.raw,
        s=style.pos_integer.s,
        alpha=style.pos_integer.alpha,
        edgecolors=style.pos_integer.edgecolors,
        cmap=style.pos_integer.cmap,
        norm=style.pos_integer.norm
    )
    plt.xlabel('longitude')
    plt.ylabel('latitude')
    plt.colorbar()
    plt.grid(False)
    plt.savefig(f'{sim.results_dir}/{data.iteration}/stats/workloads.png')
    # plt.show()
    plt.close()


def plot_workload_histogram(data: Data, sim: Simulation, style: Style):
    '''
    TODO
    '''

    num_bins = int(np.ceil(data.workloads.max)) - \
        int(np.floor(data.workloads.min))
    plt.style.use(style.plt)
    plt.figure()
    plt.title(f'workloads {data.iteration}')  # TODO
    plt.hist(
        data.workloads.raw,
        bins=num_bins,
        fc=style.hist.fc,
        ec=style.hist.ec
    )
    plt.xlabel('workloads')
    plt.ylabel('amount')
    plt.grid(False)
    plt.savefig(
        f'{sim.results_dir}/{data.iteration}/stats/workloads_hist.png')
    # plt.show()
    plt.close()


def plot_boxplots(sim: Simulation, data: Data, style: Style):
    '''
    https://matplotlib.org/3.1.1/gallery/pyplots/boxplot_demo_pyplot.html#sphx-glr-gallery-pyplots-boxplot-demo-pyplot-py
    '''
    pass


def plot_workloads_sorted(sim: Simulation, data: Data, style: Style):
    pass


def _plot_new_metric_sorted(sim: Simulation, data: Data, style: Style):
    pass


def run(sim: Simulation, style: Style):
    '''
    TODO function docstring
    '''

    # vis workloads
    data = Data()
    for i in range(sim.num_iter):
        data.start_new_iteration(iteration=i)

        print(f'iteration {data.iteration}')

        if i == 0:
            print('Read in edge-info')
            data.read_in_edge_info(sim=sim)

        print('Read in new metrics')
        data.read_in_workloads(sim=sim)

        print(f'mean={data.workloads.mean}')
        print(f' std={data.workloads.std}')
        print(f' min={data.workloads.min}')
        print(f' max={data.workloads.max}')

        if data.iteration > 0:
            print('plot delta-workloads')
            plot_delta_workloads(sim=sim, data=data, style=style)
        print('plot workloads')
        plot_workloads(sim=sim, data=data, style=style)
        print('plot workloads as histogram')
        plot_workload_histogram(sim=sim, data=data, style=style)
        print('plot boxplots')
        plot_boxplots(sim=sim, data=data, style=style)
        print('plot workloads sorted')
        plot_workloads_sorted(sim=sim, data=data, style=style)

        # new line if next iteration

        if sim.is_last_iteration(data.iteration):
            print('')
