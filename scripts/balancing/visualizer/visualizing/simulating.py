import os


class Simulation():
    '''
    Just a struct of values
    '''

    def __init__(self, results_dir):
        self._results_dir = results_dir

        i = 0
        while os.path.exists(os.path.join(self.results_dir, f"{i}")):
            i += 1
        self._num_iter = i

    def is_last_iteration(self, iteration):
        return iteration < self._num_iter - 1

    @property
    def iteration_max(self):
        return self.num_iter - 1

    @property
    def results_dir(self):
        return self._results_dir

    @property
    def num_iter(self):
        return self._num_iter
