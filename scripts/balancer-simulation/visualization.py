#!/usr/bin/env python3

'''
TODO module docstring
'''

import os
import csv
import math
import matplotlib as mpl
import matplotlib.pyplot as plt
import numpy as np

def run(cfg):
    '''
    TODO function docstring
    '''
    results_dir = cfg['results_dir']
    num_iter = cfg['num_iter']

    for i in range(num_iter):
        print(f'workloads {i}')

        print('Read coordinates')

        lats = []
        lons = []
        coords_csv_path = f'{results_dir}/{i}/stats/src-dst-coords.csv'
        with open(coords_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for row in csv_reader:
                src_lat = float(row['src-lat'])
                src_lon = float(row['src-lon'])
                dst_lat = float(row['dst-lat'])
                dst_lon = float(row['dst-lon'])
                # take mid-point of an edge as reference
                lats.append((src_lat+ dst_lat) / 2.0)
                lons.append((src_lon + dst_lon) / 2.0)

        print('Read workloads')

        workloads = []
        workloads_csv_path = f'{results_dir}/{i}/stats/workloads.csv'
        with open(workloads_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for row in csv_reader:
                value = float(row['workloads'])
                if value != 0:
                    value = math.log(value) / math.log(10)

                workloads.append(value)

        print(f'mean={np.mean(workloads)}')
        print(f' std={np.std(workloads)}')

        # plot

        plt.figure()
        plt.title(f'workloads {i}')

        cmap = mpl.cm.get_cmap(cfg['cmap'])
        plt.scatter(
            lons,
            lats,
            c=workloads,
            s=20,
            alpha=0.4,
            edgecolors='none',
            cmap=cmap,
            label='',
            # vmin=0,
            # vmax=10
        )

        plt.xlabel('longitude')
        plt.ylabel('latitude')
        plt.colorbar()
        plt.grid(False)
        plt.savefig(f'{results_dir}/{i}/stats/workloads.png')
        # plt.show()


if __name__ == '__main__':
    cwd = os.path.join(os.getcwd(), os.path.dirname(__file__))

    # vis workloads
    run({
        'results_dir': os.path.join(
            cwd,
            '../../custom/results/isle_of_man_2020-03-14/2020-05-28_10-20-20'
        ),
        'cmap': 'copper',
        'num_iter': 10
    })
