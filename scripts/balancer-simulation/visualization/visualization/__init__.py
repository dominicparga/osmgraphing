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

from .styling import Style, Norm, LogNorm
from .simulating import Simulation
from .model import Workload, Data


def plot_workloads(data: Data, sim: Simulation, style: Style):
    '''
    TODO
    '''

    plt.style.use(style.plt.sheet)
    fig, ax = plt.subplots()
    ax.set_title(f'workloads {data.iteration}')
    print(style.scatter.pos_integer.norm.vmin)
    print(style.scatter.pos_integer.norm.vmax)
    plot_collection = ax.scatter(
        x=data.lons,
        y=data.lats,
        c=data.workloads.raw,
        s=style.scatter.pos_integer.s,
        alpha=style.scatter.pos_integer.alpha,
        edgecolors=style.scatter.pos_integer.edgecolors,
        cmap=style.scatter.pos_integer.cmap,
        norm=deepcopy(style.scatter.pos_integer.norm),
        rasterized=None
    )
    print(style.scatter.pos_integer.norm.vmin)
    print(style.scatter.pos_integer.norm.vmax)
    ax.set_xlabel('longitude')
    ax.set_ylabel('latitude')
    ax.set_aspect(1.0 / np.cos(np.deg2rad(data.lats_mid)))
    fig.colorbar(
        mappable=plot_collection,
        label=style.fig.colorbar.label,
        shrink=style.fig.colorbar.shrink,
        extend=style.fig.colorbar.extend
    )
    plt.grid(False)
    plt.tight_layout()
    plt.savefig(
        os.path.join(
            sim.results_dir,
            f'{data.iteration}',
            'stats',
            'workloads.png'
        ),
        dpi=style.dpi
    )
    # plt.show()
    plt.close()


def plot_delta_workloads(data: Data, sim: Simulation, style: Style):
    '''
    TODO function docstring
    '''

    plt.style.use(style.plt.sheet)
    fig, ax = plt.subplots()
    ax.set_title(f'delta-workloads {data.iteration - 1} to {data.iteration}')
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
    ax.set_aspect(1 / np.cos(np.deg2rad(data.lats_mid)))
    fig.colorbar(
        mappable=plot_collection,
        label=style.fig.colorbar.label,
        shrink=style.fig.colorbar.shrink,
        extend=style.fig.colorbar.extend
    )
    plt.grid(False)
    plt.tight_layout()
    plt.savefig(
        os.path.join(
            sim.results_dir,
            f'{data.iteration}',
            'stats',
            'delta_workloads.png'
        ),
        dpi=style.dpi
    )
    # plt.show()
    plt.close()


def plot_workload_histogram(data: Data, sim: Simulation, style: Style):
    '''
    TODO
    '''

    num_bins = int(np.ceil(data.workloads.max)) - \
        int(np.floor(data.workloads.min))
    plt.style.use(style.plt.sheet)
    _fig, ax = plt.subplots()
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
    plt.tight_layout()
    plt.savefig(
        os.path.join(
            sim.results_dir,
            f'{data.iteration}',
            'stats',
            'workloads_hist.png'
        ),
        dpi=style.dpi
    )
    # plt.show()
    plt.close()


def plot_sorted_workloads(sim: Simulation, data: Data, style: Style):
    '''
    https://matplotlib.org/3.1.1/gallery/pyplots/boxplot_demo_pyplot.html#sphx-glr-gallery-pyplots-boxplot-demo-pyplot-py
    '''

    LAT_IDX = 0
    LON_IDX = 1
    WLD_IDX = 2
    sorted_workloads = np.array(sorted(
        list(map(list, zip(data.lats, data.lons, data.workloads.raw))),
        key=lambda x: x[WLD_IDX]
    ))
    n = len(sorted_workloads)
    (q_low, q_high) = (int(0.25 * n), int(0.95 * n))
    sorted_workloads = sorted_workloads[q_low: q_high]
    print(sorted_workloads)

    plt.style.use(style.plt.sheet)
    fig, ax = plt.subplots()
    ax.set_title(f'sorted workloads {data.iteration}')
    plot_collection = ax.scatter(
        x=sorted_workloads[:, LON_IDX],
        y=sorted_workloads[:, LAT_IDX],
        c=sorted_workloads[:, WLD_IDX],
        s=style.scatter.pos_integer.s,
        alpha=style.scatter.pos_integer.alpha,
        edgecolors=style.scatter.pos_integer.edgecolors,
        cmap=style.scatter.pos_integer.cmap,
        norm=Norm(),
        rasterized=None
    )
    ax.set_xlabel('longitude')
    ax.set_ylabel('latitude')
    ax.set_aspect(1.0 / np.cos(np.deg2rad(data.lats_mid)))
    fig.colorbar(
        mappable=plot_collection,
        label=style.fig.colorbar.label,
        shrink=style.fig.colorbar.shrink,
        extend=style.fig.colorbar.extend
    )
    plt.grid(False)
    plt.tight_layout()
    plt.savefig(
        os.path.join(
            sim.results_dir,
            f'{data.iteration}',
            'stats',
            'sorted_workloads.png'
        ),
        dpi=style.dpi
    )
    # plt.show()
    plt.close()


def _plot_new_metric_sorted(sim: Simulation, data: Data, style: Style):
    pass


def run(sim: Simulation, style: Style):
    '''
    TODO function docstring
    '''

    # vis workloads
    data = Data()
    for i in range(sim.num_iter):
        print(f'Preparing new ITERATION {i}, e.g. reading in new data')
        data.prepare_new_iteration(sim=sim)

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
        plot_sorted_workloads(sim=sim, data=data, style=style)

        # new line if next iteration

        if sim.is_last_iteration(data.iteration):
            print('')
