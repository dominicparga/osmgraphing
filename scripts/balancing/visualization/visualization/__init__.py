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

from visualization.simulating import Simulation
from visualization.model import Data, GlobalData
import visualization.plotting as plotting


def run(sim: Simulation, vis: plotting.Machine):
    print('get global data, e.g. maximum workload')
    global_data = GlobalData.fill(sim=sim)
    print('plot all sorted workloads')
    vis.plot_all_sorted_workloads(global_data=global_data, sim=sim)
    print('plot all boxplots')
    vis.plot_all_boxplot_workloads(global_data=global_data, sim=sim)
    print('plot all max workloads')
    vis.plot_all_max_workloads(global_data=global_data, sim=sim)
    print('plot number of unique edges')
    vis.plot_all_unique_edges(global_data=global_data, sim=sim)
    print('')

    data = Data(global_data=global_data)
    for i in range(sim.num_iter):
        print(f'Preparing new ITERATION {i}, e.g. reading in new data')
        data.prepare_new_iteration(sim=sim)

        print(f'mean={data.workloads.mean}')
        print(f' std={data.workloads.std}')
        print(f' min={data.workloads.min}')
        print(f' max={data.workloads.max}')

        print('plot workloads')
        vis.plot_workloads(sim=sim, data=data)
        print('plot workload-quantiles')
        vis.plot_workload_quantiles(data=data, sim=sim)
        if data.iteration > 0:
            print('plot delta-workloads')
            vis.plot_delta_workloads(sim=sim, data=data)
            print('plot delta-workload-quantiles')
            vis.plot_delta_workload_quantiles(sim=sim, data=data)
        print('plot workloads as histogram')
        vis.plot_workload_histogram(sim=sim, data=data)

        # new line if next iteration

        if sim.is_last_iteration(data.iteration):
            print('')
