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
from .simulating import Simulation
from .model import Workload, Data


def plot_workloads(data: Data, sim: Simulation, style: Style):
    '''
    TODO
    '''

    plt.style.use(style.plt.sheet)
    fig, ax = plt.subplots(constrained_layout=style.plt.is_layout_constrained)
    ax.set_title(f'workloads {data.iteration}')
    plot_collection = ax.scatter(
        x=data.lons,
        y=data.lats,
        c=data.workloads.raw,
        s=style.scatter.pos_integer.s,
        alpha=style.scatter.pos_integer.alpha,
        edgecolors=style.scatter.pos_integer.edgecolors,
        cmap=style.scatter.pos_integer.cmap,
        norm=style.scatter.pos_integer.norm
    )
    ax.set_xlabel('longitude')
    ax.set_ylabel('latitude')
    # ax.set_aspect(1 / np.cos(np.deg2rad(49))) # TODO set aspect
    fig.colorbar(
        mappable=plot_collection,
        shrink=1.0,
        extend=style.fig.colorbar.extend
    )
    plt.grid(False)
    plt.savefig(f'{sim.results_dir}/{data.iteration}/stats/workloads.png')
    # plt.show()
    plt.close()


def plot_delta_workloads(data: Data, sim: Simulation, style: Style):
    '''
    TODO function docstring
    '''

    plt.style.use(style.plt.sheet)
    fig, ax = plt.subplots(constrained_layout=style.plt.is_layout_constrained)
    ax.set_title(f'delta-workloads {data.iteration}')
    plot_collection = ax.scatter(
        x=data.lons,
        y=data.lats,
        c=data.delta_workloads.raw,
        s=style.scatter.integer.s,
        alpha=style.scatter.integer.alpha,
        edgecolors=style.scatter.integer.edgecolors,
        cmap=style.scatter.integer.cmap,
        norm=style.scatter.integer.norm
    )
    ax.set_xlabel('longitude')
    ax.set_ylabel('latitude')
    # ax.set_aspect(1 / np.cos(np.deg2rad(49))) # TODO set aspect
    fig.colorbar(
        mappable=plot_collection,
        shrink=style.fig.colorbar.shrink,
        extend=style.fig.colorbar.extend
    )
    plt.grid(False)
    plt.savefig(
        f'{sim.results_dir}/{data.iteration}/stats/delta_workloads.png')
    # plt.show()
    plt.close()


def plot_workload_histogram(data: Data, sim: Simulation, style: Style):
    '''
    TODO
    '''

    num_bins = int(np.ceil(data.workloads.max)) - \
        int(np.floor(data.workloads.min))
    plt.style.use(style.plt.sheet)
    _fig, ax = plt.subplots(constrained_layout=style.plt.is_layout_constrained)
    ax.set_title(f'workloads {data.iteration}')  # TODO
    _plot_collection = ax.hist(
        data.workloads.raw,
        bins=num_bins,
        fc=style.hist.fc,
        ec=style.hist.ec
    )
    ax.set_xlabel('workloads')
    ax.set_ylabel('amount')
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
