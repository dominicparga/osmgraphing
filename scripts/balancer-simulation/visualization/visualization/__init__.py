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
from visualization.model import Workload, Data
import visualization.plotting as plotting


def run(sim: Simulation, vis: plotting.Machine):
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

        print('plot workloads')
        vis.plot_workloads(sim=sim, data=data)
        if data.iteration > 0:
            print('plot delta-workloads')
            vis.plot_delta_workloads(sim=sim, data=data)
        print('plot workloads as histogram')
        vis.plot_workload_histogram(sim=sim, data=data)
        print('plot sorted workloads')
        vis.plot_sorted_workloads(sim=sim, data=data)
        print('plot boxplots')
        # TODO

        # new line if next iteration

        if sim.is_last_iteration(data.iteration):
            print('')
