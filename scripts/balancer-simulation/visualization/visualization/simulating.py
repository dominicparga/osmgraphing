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
