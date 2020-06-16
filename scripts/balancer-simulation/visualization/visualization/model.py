import os
import filecmp
import numpy as np
import csv

from visualization.simulating import Simulation


class GlobalData():
    def __init__(self):
        self._max_workload = None

    @property
    def max_workload(self):
        return self._max_workload

    @staticmethod
    def fill(sim: Simulation):
        global_data = GlobalData()

        data = Data(sim.iteration_0)
        for i in range(sim.iteration_0, sim.iteration_max + 1):
            data.prepare_new_iteration(sim=sim)
            if global_data._max_workload is None:
                global_data._max_workload = data.workloads.max
            else:
                if data.workloads.max > global_data._max_workload:
                    global_data._max_workload = data.workloads.max

        return global_data


class Values():
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
        self._center = None
        self._mean = None
        self._std = None

    @property
    def min(self):
        if self._min is None:
            self._min = np.min(self._raw)
        return self._min

    @property
    def center(self):
        if self._center is None:
            self._center = (self.min + self.max) / 2.0
        return self._center

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

    def __init__(self, global_data, iteration_0=0):
        self._iteration = iteration_0 - 1
        self._lats = Values()
        self._lons = Values()
        self._kilometers = Values()
        self._lane_counts = Values()
        self._old_workloads = Values()
        self._workloads = Values()
        self._delta_workloads = None

        self._global_data = global_data

    def prepare_new_iteration(self, sim: Simulation):
        self._iteration += 1

        # reset all current data

        tmp = self.old_workloads.raw
        self.old_workloads.raw = self.workloads.raw
        self.workloads.raw = tmp
        self.workloads.raw.clear()

        self._delta_workloads = None

        # continue TODO

        if self.iteration == sim.iteration_0:
            self.check_for_equal_edge_files(sim=sim)
            self.read_in_edge_info(sim=sim)

        self.read_in_workloads(sim=sim)

    def path_to_edge_info(self, iteration=None):
        if iteration is None:
            iteration = self.iteration
        return os.path.join(f'{iteration}', 'stats', 'edge-info.csv')

    def path_to_abs_workloads(self, iteration=None):
        if iteration is None:
            iteration = self.iteration
        return os.path.join(f'{self.iteration}', 'stats', 'abs_workloads.csv')

    def path_to_new_metrics(self, iteration=None):
        if iteration is None:
            iteration = self.iteration
        return os.path.join(f'{self._iteration}', 'stats', 'new_metrics.csv')

    @property
    def global_data(self):
        return self._global_data

    @property
    def iteration(self):
        return self._iteration

    @property
    def lats(self):
        return self._lats

    @property
    def lons(self):
        return self._lons

    @property
    def kilometers(self):
        return self._kilometers

    @property
    def lane_counts(self):
        return self._lane_counts

    def volume(self, edge_idx: int) -> float:
        '''
        It's used for hopefully greater numbers

        Nagel-Schreckenberg-Model: 7.5 m per vehicle
        '''
        num_vehicles = max(1.0, self._kilometers.raw[edge_idx] / 0.0075)
        return num_vehicles * self._lane_counts.raw[edge_idx]

    @property
    def old_workloads(self):
        return self._old_workloads

    @property
    def workloads(self):
        return self._workloads

    def sorted_lon_lat_workloads(self):
        return np.array(sorted(
            list(map(list, zip(
                self.lons.raw,
                self.lats.raw,
                self.workloads.raw
            ))),
            key=lambda x: x[2]
        ))

    def sorted_lon_lat_deltas(self):
        return np.array(sorted(
            list(map(list, zip(
                self.lons.raw,
                self.lats.raw,
                self.delta_workloads.raw
            ))),
            key=lambda x: x[2]
        ))

    @property
    def delta_workloads(self):
        if self._delta_workloads is None:
            self._delta_workloads = Values()
            for new, old in zip(self.workloads.raw, self.old_workloads.raw):
                self._delta_workloads.raw.append(new - old)
        return self._delta_workloads

    def check_for_equal_edge_files(self, sim: Simulation):
        '''
        If this is not successful, the rows of edges from iteration `i`
        don't fit to the rows of edges from iteration `i+1`.
        '''
        last_file = os.path.join(
            sim.results_dir,
            self.path_to_edge_info(sim.iteration_0)
        )
        for i in range(
            sim.iteration_0 + 1,
            sim.iteration_0 + sim.num_iter
        ):
            next_file = os.path.join(
                sim.results_dir,
                self.path_to_edge_info(i)
            )

            if not filecmp.cmp(last_file, next_file, shallow=False):
                raise RuntimeError(
                    f'The edge-info {i} isn\'t equal to edge-info {i-1}.'
                )

            last_file = next_file

    def read_in_edge_info(self, sim: Simulation):
        coords_csv_path = os.path.join(
            f'{sim.results_dir}',
            self.path_to_edge_info()
        )
        with open(coords_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for row in csv_reader:
                src_lat = float(row['src_lat'])
                src_lon = float(row['src_lon'])
                dst_lat = float(row['dst_lat'])
                dst_lon = float(row['dst_lon'])
                kilometers = float(row['kilometers'])
                lane_count = float(row['lane_count'])

                self.lats.raw.append((src_lat + dst_lat) / 2.0)
                self.lons.raw.append((src_lon + dst_lon) / 2.0)
                self.kilometers.raw.append(kilometers)
                self.lane_counts.raw.append(lane_count)

    def read_in_workloads(self, sim: Simulation):
        workloads_csv_path = os.path.join(
            f'{sim.results_dir}',
            self.path_to_abs_workloads()
        )
        with open(workloads_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for edge_idx, row in enumerate(csv_reader):
                value = float(row['num_routes']) / self.volume(edge_idx)
                self.workloads.raw.append(value)

    def _read_in_new_metrics(self, sim: Simulation):
        workloads_csv_path = os.path.join(
            sim.results_dir,
            self.path_to_new_metrics()
        )
        with open(workloads_csv_path, mode='r') as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=' ')
            for row in csv_reader:
                value = float(row['new_metrics'])
                self.workloads.raw.append(value)
