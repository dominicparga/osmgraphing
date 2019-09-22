#!/usr/bin/env python3

import csv
import math
import matplotlib as mpl
import matplotlib.pyplot as plt
import sys

#-----------------------------------------------------------------------------#
# utilization

def utilization(csv_path, label, out_path):
    with open(csv_path, mode='r') as csv_file:
        csv_reader = csv.DictReader(csv_file, delimiter=',')
        # csv_reader = csv.reader(csv_file, delimiter=',')

        print("Visualizing utilization")

        # get list of relevant data
        lons = []
        lats = []
        vals = []
        for row in csv_reader:
            lons.append(10e-7 * int(row['decimicro_lon']))
            lats.append(10e-7 * int(row['decimicro_lat']))
            length_m = float(row['length_m'])
            lane_count = float(row['lane_count'])
            route_count = float(row['route_count'])

            # Nagel-Schreckenberg-model
            vehicles_per_lane = length_m / 7.5
            vehicles_per_edge = lane_count * vehicles_per_lane
            utilization = route_count / vehicles_per_edge
            vals.append(utilization)

    # plot data
    plt.figure()
    plt.title("Edge-utilization")

    cmap = mpl.cm.get_cmap('copper')
    plt.scatter(lons, lats, s=20, c=vals, alpha=0.3, edgecolors='none', cmap=cmap, label=label)

    plt.xlabel('longitude')
    plt.ylabel('latitude')
    plt.colorbar()
    plt.grid(True)
    plt.savefig(out_path)

#-----------------------------------------------------------------------------#
# lat, lon, src, dst

def vis_lat_lon(csv_path, label, out_path):
    with open(csv_path, mode='r') as csv_file:
        csv_reader = csv.DictReader(csv_file, delimiter=',')
        # csv_reader = csv.reader(csv_file, delimiter=',')

        print("Visualizing lat-lon-src-dst")

        # get list of relevant data
        # lons, lats, styles
        data = {
            'vani': {'lons': [], 'lats': [], 'c': 0.0, 'label': 'vani', 'plot': False},
            'dsts': {'lons': [], 'lats': [], 'c': 0.3, 'label': 'dst', 'plot': True},
            'both': {'lons': [], 'lats': [], 'c': 0.6, 'label': 'src & dst', 'plot': True},
            'srcs': {'lons': [], 'lats': [], 'c': 1.0, 'label': 'src', 'plot': True},
        }
        for row in csv_reader:
            # parse data
            is_src = row['is_src'] == 'true'
            is_dst = row['is_dst'] == 'true'
            lon = 10e-7 * int(row['decimicro_lon'])
            lat = 10e-7 * int(row['decimicro_lat'])

            # append to data
            if (not is_dst) and (not is_src) and data['vani']['plot']:
                data['vani']['lons'].append(lon)
                data['vani']['lats'].append(lat)
            elif is_dst and (not is_src) and data['dsts']['plot']:
                data['dsts']['lons'].append(lon)
                data['dsts']['lats'].append(lat)
            elif is_dst and is_src and data['both']['plot']:
                data['both']['lons'].append(lon)
                data['both']['lats'].append(lat)
            elif (not is_dst) and is_src and data['srcs']['plot']:
                data['srcs']['lons'].append(lon)
                data['srcs']['lats'].append(lat)

    # plot data
    plt.figure()
    plt.title('src-dst')

    cmap = mpl.cm.get_cmap('copper')
    for _, d in data.items():
        if not d['plot']:
            continue
        lons = d['lons']
        lats = d['lats']
        label = d['label']
        c = cmap(d['c'])
        plt.scatter(lons, lats, s=20, c=[c], alpha=1.0, edgecolors='none', cmap=cmap, label=label)

    plt.xlabel('longitude')
    plt.ylabel('latitude')
    plt.legend()
    plt.grid(True)
    plt.savefig(out_path)

#-----------------------------------------------------------------------------#
# route-counts

def vis_route_counts(cfg):
    out_path = cfg[-1]

    csv_path = None
    plt.figure()
    plt.title('route-count -> log10(frequency)')
    for cfg_i in range(len(cfg)):
        if cfg_i % 2 == 0:
            csv_path = cfg[cfg_i]
            continue
        label = cfg[cfg_i]
        with open(csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=',')
            # csv_reader = csv.reader(csv_file, delimiter=',')

            print("Visualizing route-counts")

            # get sorted list of route-counts
            route_counts = []
            for row in csv_reader:
                route_counts.append(int(row['route_count']))
            route_counts.sort()

            # reduce to count duplicates
            x = [route_counts[0]]
            y = [0]
            for rc in route_counts:
                if rc == x[-1]:
                    y[-1] = y[-1] + 1
                else:
                    x.append(rc)
                    y.append(1)

            # smooth
            for i in range(len(y)):
                y[i] = math.log(y[i]) / math.log(10)

            plt.plot(x, y, '-', label=label)
    plt.legend()
    plt.xlabel('route-count of some edge')
    plt.ylabel('log10(frequency)')
    plt.savefig(out_path)

#-----------------------------------------------------------------------------#
# main

if __name__ == '__main__':
    if sys.argv[1] == 'route-counts':
        csv_paths = sys.argv[2:]
        vis_route_counts(csv_paths)
    elif sys.argv[1] == 'src-dst':
        vis_lat_lon(sys.argv[2], sys.argv[3], sys.argv[4])
    elif sys.argv[1] == 'utilization':
        utilization(sys.argv[2], sys.argv[3], sys.argv[4])
    else:
        print("Please provide identically-structured csv-paths as cmdline-arg.")
        print("They will be printed together.")
        print("Format: visualization.py (src-dst|route-counts) [path_0 label_0 path_1 label_1 ...] out_path")
        exit(0)
