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


class Style():
    '''
    Just a struct of values

    # available default-styles
    # https://matplotlib.org/3.1.0/gallery/style_sheets/style_sheets_reference.html

    # choose colormaps
    # https://matplotlib.org/3.1.0/tutorials/colors/colormaps.html
    '''

    def __init__(
        self,
        plt_style,
        positive_cmap,
        delta_cmap,
        hist_fc,
        hist_ec,
        scatter_s=2,
        scatter_alpha=1.0,
    ):
        self._plt = plt_style
        self._positive_cmap = positive_cmap
        self._delta_cmap = delta_cmap
        self._hist_fc = hist_fc
        self._hist_ec = hist_ec
        self._scatter_s = scatter_s
        self._scatter_alpha = scatter_alpha

    @property
    def plt(self):
        return self._plt

    @property
    def positive_cmap(self):
        return self._positive_cmap

    @property
    def delta_cmap(self):
        return self._delta_cmap

    @property
    def hist_fc(self):
        return self._hist_fc

    @property
    def hist_ec(self):
        return self._hist_ec

    @property
    def scatter_s(self):
        return self._scatter_s

    @property
    def scatter_alpha(self):
        return self._scatter_alpha

    @staticmethod
    def _grayscale():
        '''
        TODO
        '''
        return Style.light()  # TODO

    @staticmethod
    def light():
        '''
        TODO
        '''
        return Style(
            plt_style='default',
            # positive_cmap='cubehelix_r',
            positive_cmap='binary',
            # positive_cmap='PuRd',
            # delta_cmap='PRGn_r',
            delta_cmap='seismic',
            # delta_cmap='PiYG_r', # nice but too lighten
            hist_fc='k',
            hist_ec='k'
        )

    @staticmethod
    def dark():
        '''
        TODO
        '''
        return Style(
            plt_style='dark_background',
            positive_cmap='cubehelix',
            # delta_cmap='cividis',
            # delta_cmap='winter',
            delta_cmap='twilight',
            hist_fc='w',
            hist_ec='w'
        )


class Workload():
    '''
    Just a struct of values
    '''

    def __init__(self):
        self.raw = []

    @property
    def raw(self):
        return self._raw

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
        self._log_workloads = None
        self._delta_workloads = None
        self._log_delta_workloads = None

    def log_norm_pos(self, workloads: Workload):
        '''
        A value is mapped from [min, max] to [1.0, 2.0] before log2 is
        taken, resulting in a value in [0.0, 1.0].

        ATTENTION: 0.0 < min, max
        '''

        return [
            math.log2(1.0 + (w - workloads.min) /
                      (workloads.max - workloads.min))
            for w in workloads.raw
        ]

    def log_norm(self, workloads: Workload):
        '''
        All positive values are mapped from [0.0, max] to [1.0, 2.0]
        before log2 is taken, resulting in a value in [0.0, 1.0].

        Negative values are negated before being treated as a positive
        value. After mapping to [0.0, 1.0], the resulting value is
        negated again, resulting in a value in [-1.0, 0.0].

        ATTENTION: min < 0.0 < max
        '''

        new_list = deepcopy(workloads.raw)

        max_value = np.max(new_list)
        min_value = np.min(new_list)

        for i in range(len(new_list)):
            v = new_list[i]
            # positive values
            if v > 0.0:  # -> max > 0.0
                # get a value in [1.0, 2.0]
                v = 1.0 + v / max_value
                # get a value in [0.0, 1.0]
                new_list[i] = math.log2(v)
            # negative values
            elif v < 0.0:  # -> min < 0.0
                # get a value in [1.0, 2.0]
                v = 1.0 + v / min_value
                # get a value in [-1.0, 0.0]
                new_list[i] = -math.log2(v)

        return new_list

    def start_new_iteration(self, iteration: int):
        self._iteration = iteration

        tmp = self.old_workloads.raw
        self.old_workloads.raw = self.workloads.raw
        self.workloads.raw = tmp
        self.workloads.raw.clear()

        self._log_workloads = None
        self._delta_workloads = None
        self._log_delta_workloads = None

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
    def log_workloads(self):
        if self._log_workloads is None:
            self._log_workloads = Workload()
            self._log_workloads.raw = self.log_norm_pos(self.workloads)
        return self._log_workloads

    @ property
    def delta_workloads(self):
        if self._delta_workloads is None:
            self._delta_workloads = Workload()
            for new, old in zip(self.workloads.raw, self.old_workloads.raw):
                self._delta_workloads.raw.append(new - old)
        return self._delta_workloads

    @ property
    def log_delta_workloads(self):
        if self._log_delta_workloads is None:
            self._log_delta_workloads = Workload()
            self._log_delta_workloads.raw = self.log_norm(self.delta_workloads)
        return self._log_delta_workloads

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
        workloads_csv_path = f'{sim.results_dir}/{self._iteration}/stats/workloads.csv'
        with open(workloads_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for edge_idx, row in enumerate(csv_reader):
                value = float(row['num_routes']) / \
                    self.volume(edge_idx)  # TODO normalize
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
        s=style.scatter_s,
        alpha=style.scatter_alpha,
        edgecolors='none',
        cmap=style.delta_cmap,
        label='',
        # vmin=0,
        # vmax=10
    )
    plt.xlabel('longitude')
    plt.ylabel('latitude')
    plt.colorbar()
    plt.grid(False)
    plt.savefig(
        f'{sim.results_dir}/{data.iteration}/stats/delta_workloads.png')
    # plt.show()
    plt.close()


def plot_norm_delta_workloads(data: Data, sim: Simulation, style: Style):
    '''
    TODO function docstring
    '''

    plt.style.use(style.plt)
    plt.figure()
    plt.title(f'normalized delta-workloads {data.iteration}')
    plt.scatter(
        data.lons,
        data.lats,
        c=data.log_delta_workloads.raw,
        s=style.scatter_s,
        alpha=style.scatter_alpha,
        edgecolors='none',
        cmap=style.delta_cmap,
        label='',
        # vmin=0,
        # vmax=10
    )
    plt.xlabel('longitude')
    plt.ylabel('latitude')
    plt.colorbar()
    plt.grid(False)
    plt.savefig(
        f'{sim.results_dir}/{data.iteration}/stats/norm_delta_workloads.png')
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
        s=style.scatter_s,
        alpha=style.scatter_alpha,
        edgecolors='none',
        cmap=style.positive_cmap,
        label='',
        # vmin=0,
        # vmax=10
    )
    plt.xlabel('longitude')
    plt.ylabel('latitude')
    plt.colorbar()
    plt.grid(False)
    plt.savefig(f'{sim.results_dir}/{data.iteration}/stats/workloads.png')
    # plt.show()
    plt.close()


def plot_norm_workloads(data: Data, sim: Simulation, style: Style):
    '''
    TODO
    '''

    plt.style.use(style.plt)
    plt.figure()
    plt.title(f'normalized workloads {data.iteration}')
    plt.scatter(
        data.lons,
        data.lats,
        c=data.log_workloads.raw,
        s=style.scatter_s,
        alpha=style.scatter_alpha,
        edgecolors='none',
        cmap=style.positive_cmap,
        label='',
        # vmin=0,
        # vmax=10
    )
    plt.xlabel('longitude')
    plt.ylabel('latitude')
    plt.colorbar()
    plt.grid(False)
    plt.savefig(f'{sim.results_dir}/{data.iteration}/stats/norm_workloads.png')
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
    plt.title(f'workloads {data.iteration} greater than 0.0')  # TODO
    plt.hist(
        data.workloads.raw,
        bins=num_bins,
        fc=style.hist_fc,
        ec=style.hist_ec
    )
    plt.xlabel('workloads')
    plt.ylabel('amount')
    plt.grid(False)
    plt.savefig(
        f'{sim.results_dir}/{data.iteration}/stats/workloads_hist.png')
    # plt.show()
    plt.close()


def plot_norm_workloads_histogram(data: Data, sim: Simulation, style: Style):
    '''
    TODO
    '''

    # same number of bins than before
    num_bins = int(np.ceil(data.workloads.max)) - \
        int(np.floor(data.workloads.min))
    plt.style.use(style.plt)
    plt.figure()
    plt.title(f'normalized workloads {data.iteration} histogram')
    plt.hist(
        data.log_workloads.raw,
        bins=num_bins,
        fc=style.hist_fc,
        ec=style.hist_ec
    )
    plt.xlabel('log(workloads)')
    plt.ylabel('amount')
    plt.grid(False)
    plt.savefig(
        f'{sim.results_dir}/{data.iteration}/stats/norm_workloads_hist.png')
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
            print('plot_delta_workloads')
            plot_delta_workloads(sim=sim, data=data, style=style)
            print('plot_norm_delta_workloads')
            plot_norm_delta_workloads(sim=sim, data=data, style=style)
        print('plot_workloads')
        plot_workloads(sim=sim, data=data, style=style)
        print('plot_norm_workloads')
        plot_norm_workloads(sim=sim, data=data, style=style)
        print('plot_workload_histogram')
        plot_workload_histogram(sim=sim, data=data, style=style)
        print('plot_norm_workloads_histogram')
        plot_norm_workloads_histogram(sim=sim, data=data, style=style)
        print('plot_boxplots')
        plot_boxplots(sim=sim, data=data, style=style)
        print('plot_workloads_sorted')
        plot_workloads_sorted(sim=sim, data=data, style=style)

        # new line if next iteration

        if sim.is_last_iteration(data.iteration):
            print('')


def parse_cmdline():
    # define args and parse them

    parser = argparse.ArgumentParser(
        description='Visualize results from balancer-binary.')

    help_msg = 'Maximum number of iterations, starting with the provided index.'
    parser.add_argument(
        '--num-iter',
        metavar=('NUM_ITER'),
        required=True,
        type=int,
        help=help_msg
    )
    help_msg = 'Directory where results are laying.'
    parser.add_argument(
        '--results-dir',
        metavar=('RESULTS_DIR'),
        required=True,
        help=help_msg
    )
    help_msg = 'Dark or light style'
    parser.add_argument(
        '--style',
        metavar=('STYLE'),
        choices=['dark', 'light'],
        default='light',
        required=False,
        help=help_msg
    )

    # finalize and return

    args = parser.parse_args()

    cwd = os.path.join(os.getcwd(), os.path.dirname(__file__))
    results_dir = os.path.join(cwd, '..', '..')
    return {
        'results_dir': os.path.join(results_dir, args.results_dir),
        'num_iter': args.num_iter,
        'style': args.style
    }


if __name__ == '__main__':
    params = parse_cmdline()

    sim = Simulation(
        results_dir=params['results_dir'],
        num_iter=params['num_iter']
    )

    if params['style'] == 'dark':
        style = Style.dark()
    if params['style'] == 'light':
        style = Style.light()

    run(sim=sim, style=style)
