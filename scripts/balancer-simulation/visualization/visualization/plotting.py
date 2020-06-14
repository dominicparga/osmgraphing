import math
import os
from copy import deepcopy
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.colors as colors

from visualization.simulating import Simulation
from visualization.model import Data
from visualization.helpers import Norm, LogNorm


class Figure():
    class Colorbar():
        def __init__(self, label=None, shrink=0.9, extend='neither'):
            self._label = label
            self._shrink = shrink
            self._extend = extend

        @property
        def label(self):
            return self._label

        @property
        def shrink(self):
            return self._shrink

        @property
        def extend(self):
            return self._extend

    def __init__(self, colorbar: Colorbar):
        self._colorbar = colorbar

    @property
    def colorbar(self):
        return self._colorbar


class Machine():
    '''
    Just a struct of values

    # available default-styles
    # https://matplotlib.org/3.1.0/gallery/style_sheets/style_sheets_reference.html

    # choose colormaps
    # https://matplotlib.org/3.1.0/tutorials/colors/colormaps.html
    '''

    def __init__(
        self,
        *,
        dpi=1024,
        is_light: bool,
        fig_style=Figure(colorbar=Figure.Colorbar()),
    ):
        self._dpi = dpi
        self._is_light = is_light

        self._fig_style = fig_style

    @property
    def dpi(self) -> int:
        return self._dpi

    @property
    def is_light(self) -> bool:
        return self._is_light

    @property
    def plt_theme(self) -> str:
        if self.is_light:
            return 'default'
        else:
            return 'dark_background'

    @property
    def fig(self) -> Figure:
        return deepcopy(self._fig_style)

    def plot_workloads(self, data: Data, sim: Simulation):
        # setup figure

        plt.style.use(self.plt_theme)
        fig, ax = plt.subplots()
        ax.set_title(f'workloads {data.iteration}')

        # set norm and cmap

        if self.is_light:
            norm, cmap = [
                [LogNorm(base=50.0), 'PuRd'],
                [LogNorm(base=40.0), 'binary'],
                [LogNorm(base=100.0), 'Purples'],
                [LogNorm(base=20.0), 'gist_heat_r'],
                [LogNorm(base=20.0), 'cubehelix_r']
            ][1]
        else:
            norm, cmap = [
                [LogNorm(base=50.0), 'copper']
            ][0]

        # plot data

        plot_collection = ax.scatter(
            x=data.lons,
            y=data.lats,
            c=data.workloads.raw,
            s=0.3,
            alpha=1.0,
            edgecolors='none',
            norm=norm,
            cmap=cmap,
        )

        # finalize plot

        ax.set_xlabel('longitude')
        ax.set_ylabel('latitude')
        ax.set_aspect(1.0 / np.cos(np.deg2rad(data.lats_mid)))
        fig.colorbar(
            mappable=plot_collection,
            label=self.fig.colorbar.label,
            shrink=self.fig.colorbar.shrink,
            extend=self.fig.colorbar.extend
        )
        plt.grid(False)
        plt.tight_layout()

        # save plot

        plt.savefig(
            os.path.join(
                sim.results_dir,
                f'{data.iteration}',
                'stats',
                'workloads.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_delta_workloads(self, data: Data, sim: Simulation):
        # setup figure

        plt.style.use(self.plt_theme)
        fig, ax = plt.subplots()
        ax.set_title(
            f'delta-workloads {data.iteration - 1} to {data.iteration}')

        # set norm and cmap

        if self.is_light:
            norm, cmap = [
                [LogNorm(vcenter=0.0, base=20.0), 'seismic'],
                [LogNorm(vcenter=0.0, base=100.0), 'PRGn_r'],
                [LogNorm(vcenter=0.0, base=100.0), 'RdGy']
            ][0]
        else:
            norm, cmap = [
                [LogNorm(vcenter=0.0, base=1000.0), 'twilight']
            ][0]

        # plot data

        plot_collection = ax.scatter(
            x=data.lons,
            y=data.lats,
            c=data.delta_workloads.raw,
            s=0.3,
            alpha=1.0,
            edgecolors='none',
            norm=norm,
            cmap=cmap,
        )

        # finalize plot

        ax.set_xlabel('longitude')
        ax.set_ylabel('latitude')
        ax.set_aspect(1 / np.cos(np.deg2rad(data.lats_mid)))
        fig.colorbar(
            mappable=plot_collection,
            label=self.fig.colorbar.label,
            shrink=self.fig.colorbar.shrink,
            extend=self.fig.colorbar.extend
        )
        plt.grid(False)
        plt.tight_layout()

        # save plot

        plt.savefig(
            os.path.join(
                sim.results_dir,
                f'{data.iteration}',
                'stats',
                'delta_workloads.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_workload_histogram(self, data: Data, sim: Simulation):
        # setup figure

        plt.style.use(self.plt_theme)
        _fig, ax = plt.subplots()
        ax.set_title(f'workloads-density-function {data.iteration}')

        #  set cmap

        if self.is_light:
            fc = 'w'
            ec = 'w'
        else:
            fc = 'k'
            ec = 'k'

        # plot data

        num_bins = int(np.ceil(data.workloads.max)) - \
            int(np.floor(data.workloads.min))
        _plot_collection = ax.hist(
            data.workloads.raw,
            bins=num_bins,
            fc=fc,
            ec=ec
        )

        # finalize plot

        ax.set_xlabel('workloads')
        ax.set_ylabel('amount')
        plt.grid(False)
        plt.tight_layout()

        # save plot

        plt.savefig(
            os.path.join(
                sim.results_dir,
                f'{data.iteration}',
                'stats',
                'workloads_hist.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_sorted_workloads(self, sim: Simulation, data: Data):
        '''
        https://matplotlib.org/3.1.1/gallery/pyplots/boxplot_demo_pyplot.html#sphx-glr-gallery-pyplots-boxplot-demo-pyplot-py
        '''

        # set norm and cmap

        if self.is_light:
            norm, cmap = [
                [LogNorm(base=50.0), 'PuRd'],
                [LogNorm(base=40.0), 'binary'],
                [LogNorm(base=100.0), 'Purples'],
                [LogNorm(base=20.0), 'gist_heat_r'],
                [LogNorm(base=20.0), 'cubehelix_r']
            ][1]
        else:
            norm, cmap = [
                [LogNorm(base=50.0), 'copper']
            ][0]

        LAT_IDX = 0
        LON_IDX = 1
        WLD_IDX = 2
        sorted_workloads = np.array(sorted(
            list(map(list, zip(data.lats, data.lons, data.workloads.raw))),
            key=lambda x: x[WLD_IDX]
        ))
        n = len(sorted_workloads)
        (q_low, q_high) = (int(0.25 * n), int(0.95 * n))
        print(sorted_workloads)

        norm.vmin = sorted_workloads[q_low][WLD_IDX]
        norm.vmax = sorted_workloads[q_high][WLD_IDX]

        # setup figure

        plt.style.use(self.plt_theme)
        fig, ax = plt.subplots()
        ax.set_title(f'sorted workloads {data.iteration}')

        # plot data

        plot_collection = ax.scatter(
            x=sorted_workloads[:, LON_IDX],
            y=sorted_workloads[:, LAT_IDX],
            c=sorted_workloads[:, WLD_IDX],
            s=0.3,
            alpha=1.0,
            edgecolors='none',
            norm=norm,
            cmap=cmap,
        )

        # finalize plot

        ax.set_xlabel('longitude')
        ax.set_ylabel('latitude')
        ax.set_aspect(1.0 / np.cos(np.deg2rad(data.lats_mid)))
        fig.colorbar(
            mappable=plot_collection,
            label=self.fig.colorbar.label,
            shrink=self.fig.colorbar.shrink,
            extend=self.fig.colorbar.extend
        )
        plt.grid(False)
        plt.tight_layout()

        # save plot

        plt.savefig(
            os.path.join(
                sim.results_dir,
                f'{data.iteration}',
                'stats',
                'sorted_workloads.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def _plot_new_metric_sorted(self, sim: Simulation, data: Data):
        pass


def light():
    return Machine(is_light=True)


def dark():
    return Machine(is_light=False)
