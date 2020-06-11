import numpy as np
import csv

from . import Simulation


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
