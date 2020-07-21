import math
import os
from copy import deepcopy
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.colors as colors
import matplotlib.tri as tri

from visualizing.simulating import Simulation
from visualizing.model import Data, GlobalData
from visualizing.helpers import TwoSlopeLoggedNorm


class Figure():
    class Colorbar():
        def __init__(self, shrink=0.9, extend='neither'):
            self._shrink = shrink
            self._extend = extend

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

    def plot_all_sorted_workloads(
        self,
        global_data: GlobalData,
        sim: Simulation
    ):
        # setup simulation

        data = Data(global_data=global_data)

        # setup figure

        plt.style.use(self.plt_theme)
        _fig, ax = plt.subplots()
        ax.set_title(f'all occured workloads')

        #  set cmap

        if self.is_light:
            cmap = plt.get_cmap('gist_heat_r')
        else:
            cmap = plt.get_cmap('copper')

        for iteration in range(sim.num_iter):
            data.prepare_new_iteration(sim=sim)
            sorted_lon_lat_workloads = data.sorted_lon_lat_workloads()[:, 2]

            mapped_values = []
            for i in range(len(sorted_lon_lat_workloads)):
                value = sorted_lon_lat_workloads[i]
                if value > 0.0:
                    mapped_values.append(value)

            # plot data

            # alpha should vary
            # dependent on iteration (first iteration should have min alpha)
            xp, fp = [0, sim.num_iter - 1], [0.2, 1.0]
            alpha = np.interp(x=iteration, xp=xp, fp=fp)
            color = cmap(alpha)

            ax.plot(
                range(len(mapped_values)),
                mapped_values,
                color=color,
                alpha=alpha
            )
            # plot maximum
            ax.plot(
                len(mapped_values)-1,
                mapped_values[-1],
                'x',
                color=color,
                alpha=alpha
            )

        # finalize plot

        ax.set_xlabel('')
        ax.set_ylabel('workload')
        plt.grid(True)
        plt.tight_layout()

        # save plot

        plt.savefig(
            os.path.join(
                sim.results_dir,
                'sorted_workloads.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_all_boxplot_workloads(
        self,
        global_data: GlobalData,
        sim: Simulation
    ):
        # setup simulation

        data = Data(global_data=global_data)

        q_low, q_high = 5, 95

        # setup figure

        plt.style.use(self.plt_theme)
        _fig, ax = plt.subplots()
        ax.set_title(f'all workloads in [{q_low} %, {q_high} %]')

        #  set cmap

        for iteration in range(sim.iteration_max + 1):
            data.prepare_new_iteration(sim=sim)
            mapped_values = np.array(list(filter(
                lambda x: x > 0.0,
                data.workloads.raw
            )))

            # plot data

            ax.boxplot(
                mapped_values,
                positions=[iteration],
                showfliers=False,
                whis=[q_low, q_high]
            )

        # finalize plot

        ax.set_xlabel('iteration')
        ax.set_ylabel('workload')
        plt.grid(False)
        plt.tight_layout()

        # save plot

        plt.savefig(
            os.path.join(
                sim.results_dir,
                'workload-boxplots.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_all_max_workloads(self, global_data: GlobalData, sim: Simulation):
        # setup simulation

        data = Data(global_data=global_data)

        # setup figure

        plt.style.use(self.plt_theme)
        _fig, ax = plt.subplots()
        ax.set_title(f'all max (delta-) workloads')

        #  set cmap

        if self.is_light:
            c_max = 'k'
            cmap = plt.get_cmap('seismic')
            c_delta_max = cmap(0.75)
            c_delta_min = cmap(0.25)
        else:
            c_max = 'w'
            cmap = plt.get_cmap('twilight')
            c_delta_max = cmap(0.75)
            c_delta_min = cmap(0.25)

        max_workloads = []
        max_delta_workloads = []
        min_delta_workloads = []
        for _ in range(sim.num_iter):
            data.prepare_new_iteration(sim=sim)
            max_workloads.append(data.workloads.max)
            if data.iteration > 0:
                max_delta_workloads.append(data.delta_workloads.max)
                min_delta_workloads.append(data.delta_workloads.min)

        # plot data

        # plot max workloads
        ax.plot(
            range(len(max_workloads)),
            max_workloads,
            'x-',
            color=c_max,
            label='max workloads'
        )
        # plot max delta-workloads
        ax.plot(
            np.array(range(len(max_delta_workloads))) + 0.5,
            max_delta_workloads,
            'x-',
            color=c_delta_max,
            label='max delta-workloads'
        )
        # plot min delta-workloads
        ax.plot(
            np.array(range(len(min_delta_workloads))) + 0.5,
            min_delta_workloads,
            'x-',
            color=c_delta_min,
            label='min delta-workloads'
        )

        # finalize plot

        ax.set_xlabel('iteration')
        ax.set_ylabel('(delta-) workload')
        plt.grid(True)
        plt.legend()
        plt.tight_layout()

        # save plot

        plt.savefig(
            os.path.join(
                sim.results_dir,
                'max_workloads.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_all_unique_edges(self, global_data: GlobalData, sim: Simulation):
        # setup simulation

        data = Data(global_data=global_data)

        # setup figure

        plt.style.use(self.plt_theme)
        _fig, ax = plt.subplots()
        ax.set_title(f'number of unique edges')

        #  set cmap

        if self.is_light:
            color = 'k'
        else:
            color = 'w'

        num_unique_edges = []
        for _ in range(sim.num_iter):
            data.prepare_new_iteration(sim=sim)
            num_unique_edges.append(
                np.array(data.workloads.raw).astype(bool).sum()
            )

        # plot data

        # plot max workloads
        ax.plot(
            range(len(num_unique_edges)),
            num_unique_edges,
            'x-',
            color=color,
        )

        # finalize plot

        ax.set_xlabel('iteration')
        ax.set_ylabel('amount of unique edges')
        plt.grid(True)
        plt.tight_layout()

        # save plot

        plt.savefig(
            os.path.join(
                sim.results_dir,
                'num_unique_edges.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_workloads(self, data: Data, sim: Simulation):
        '''
        https://matplotlib.org/3.1.1/gallery/pyplots/boxplot_demo_pyplot.html#sphx-glr-gallery-pyplots-boxplot-demo-pyplot-py
        '''

        # setup data

        sorted_lon_lat_workloads = data.sorted_lon_lat_workloads()

        # setup figure

        plt.style.use(self.plt_theme)
        fig, ax = plt.subplots()
        ax.set_title(
            'workloads$_{'
            + f'{data.iteration}'
            + '}$'
            + f' in [{data.workloads.min}, {data.workloads.max}]'
        )

        # set norm and cmap

        if self.is_light:
            cmap = 'binary'
        else:
            cmap = 'copper'
        norm = {
            # light
            'PuRd': colors.Normalize(),
            'binary': TwoSlopeLoggedNorm(base=5),
            'Purples': TwoSlopeLoggedNorm(base=10),
            'gist_heat_r': TwoSlopeLoggedNorm(base=20),
            'cubehelix_r': TwoSlopeLoggedNorm(),
            # dark
            'copper': TwoSlopeLoggedNorm(base=50),
        }.get(cmap, colors.Normalize())
        norm.vmax = data.global_data.max_workload

        # plot data

        plot_collection = ax.scatter(
            x=sorted_lon_lat_workloads[:, 0],
            y=sorted_lon_lat_workloads[:, 1],
            c=sorted_lon_lat_workloads[:, 2],
            s=2,  # 0.1
            alpha=1.0,
            edgecolors='none',
            cmap=cmap,
            norm=norm,
        )

        # finalize plot

        ax.set_xlabel('longitude')
        ax.set_ylabel('latitude')
        ax.set_aspect(1.0 / np.cos(np.deg2rad(data.lats.center)))
        fig.colorbar(
            mappable=plot_collection,
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

    def plot_workload_quantiles(self, data: Data, sim: Simulation):
        '''
        https://matplotlib.org/3.1.1/gallery/pyplots/boxplot_demo_pyplot.html#sphx-glr-gallery-pyplots-boxplot-demo-pyplot-py
        '''

        # setup data

        sorted_lon_lat_workloads = data.sorted_lon_lat_workloads()
        n = len(sorted_lon_lat_workloads)
        q_low, q_mid, q_high = 0.0, 0.5, 0.95
        q_low_idx, q_mid_idx, q_high_idx = (
            int(q_low * n),
            int(q_mid * n),
            int(q_high * n)
        )
        q_low_val, _q_mid_val, q_high_val = (
            sorted_lon_lat_workloads[q_low_idx, 2],
            sorted_lon_lat_workloads[q_mid_idx, 2],
            sorted_lon_lat_workloads[q_high_idx, 2]
        )

        # setup figure

        plt.style.use(self.plt_theme)
        fig, ax = plt.subplots()
        ax.set_title(
            'workloads$_{'
            + f'{data.iteration}'
            + '}$'
            + f' in [{data.workloads.min}, {data.workloads.max}]'
        )

        # set norm and cmap

        if self.is_light:
            cmap = 'cubehelix_r'
        else:
            cmap = 'copper'
        cmap = plt.get_cmap(cmap)

        # set boundaries

        boundaries = []
        if q_low_val > 0.0:
            boundaries.append(0.0)
        boundaries.extend([q_low_val, q_high_val])
        if q_high_val < data.global_data.max_workload:
            boundaries.append(data.global_data.max_workload)
        norm = colors.BoundaryNorm(
            boundaries=boundaries,
            ncolors=cmap.N
        )

        # plot data

        plot_collection = ax.scatter(
            x=sorted_lon_lat_workloads[:, 0],
            y=sorted_lon_lat_workloads[:, 1],
            c=sorted_lon_lat_workloads[:, 2],
            s=2,  # 0.1
            alpha=1.0,
            edgecolors='none',
            cmap=cmap,
            norm=norm,
        )

        # finalize plot

        ax.set_xlabel('longitude')
        ax.set_ylabel('latitude')
        ax.set_aspect(1.0 / np.cos(np.deg2rad(data.lats.center)))
        fig.colorbar(
            label=f'upper {100 * (1.0 - q_high):3.1f} % of workloads'
            + '$_{'
            + f'{data.iteration}'
            + '}$',
            mappable=plot_collection,
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
                'workload-quantiles.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_delta_workloads(self, data: Data, sim: Simulation):
        # setup data

        sorted_lon_lat_deltas = data.sorted_lon_lat_deltas()
        n = len(sorted_lon_lat_deltas)
        (q_low, q_high) = (int(0.25 * n), int(0.95 * n))
        (q_low, q_high) = (
            sorted_lon_lat_deltas[q_low][2],
            sorted_lon_lat_deltas[q_high][2]
        )

        # setup figure

        plt.style.use(self.plt_theme)
        fig, ax = plt.subplots()
        ax.set_title(
            'delta-workloads$_{' +
            f'{data.iteration-1}, {data.iteration}'
            + '}$'
            + f' in [{data.delta_workloads.min}, {data.delta_workloads.max}]'
        )

        # set norm and cmap

        if self.is_light:
            cmap = 'seismic'
        else:
            cmap = 'twilight'
        norm = {
            # light
            'seismic': colors.TwoSlopeNorm(vcenter=0.0),
            'PRGn_r': TwoSlopeLoggedNorm(vcenter=0.0),
            'RdGy': TwoSlopeLoggedNorm(vcenter=0.0),
            # dark
            'twilight': TwoSlopeLoggedNorm(vcenter=0.0),
        }.get(cmap, colors.TwoSlopeNorm(vcenter=0.0))

        # plot data

        plot_collection = ax.scatter(
            x=data.lons.raw,
            y=data.lats.raw,
            c=data.delta_workloads.raw,
            s=2,  # 0.3
            alpha=1.0,
            edgecolors='none',
            norm=norm,
            cmap=cmap,
        )

        # finalize plot

        ax.set_xlabel('longitude')
        ax.set_ylabel('latitude')
        ax.set_aspect(1 / np.cos(np.deg2rad(data.lats.center)))
        fig.colorbar(
            mappable=plot_collection,
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

    def plot_delta_workload_quantiles(self, data: Data, sim: Simulation):
        # setup data

        sorted_lon_lat_deltas = data.sorted_lon_lat_deltas()
        n = len(sorted_lon_lat_deltas)
        q_low, q_high = 0.05, 0.95
        q_low_idx, q_high_idx = (
            int(q_low * n),
            int(q_high * n)
        )
        q_low_val, q_high_val = (
            sorted_lon_lat_deltas[q_low_idx, 2],
            sorted_lon_lat_deltas[q_high_idx, 2]
        )

        # setup figure

        plt.style.use(self.plt_theme)
        fig, ax = plt.subplots()
        ax.set_title(
            'delta-workloads$_{'
            + f'{data.iteration-1}, {data.iteration}'
            + '}$'
            + f' in [{data.delta_workloads.min}, {data.delta_workloads.max}]'
        )

        # set norm and cmap

        if self.is_light:
            cmap = 'seismic'
        else:
            cmap = 'twilight'
        cmap = plt.get_cmap(cmap)

        # set boundaries

        boundaries = [
            data.delta_workloads.min,
            q_low_val,
            q_high_val,
            data.delta_workloads.max,
        ]
        norm = colors.BoundaryNorm(
            boundaries=boundaries,
            ncolors=cmap.N
        )

        # plot data

        plot_collection = ax.scatter(
            x=data.lons.raw,
            y=data.lats.raw,
            c=data.delta_workloads.raw,
            s=2,  # 0.3
            alpha=1.0,
            edgecolors='none',
            norm=norm,
            cmap=cmap,
        )

        # finalize plot

        ax.set_xlabel('longitude')
        ax.set_ylabel('latitude')
        ax.set_aspect(1 / np.cos(np.deg2rad(data.lats.center)))
        fig.colorbar(
            label=f'lower {100 * (q_low):3.1f} %'
            + f' and upper {100 * (1.0 - q_high):3.1f} % \n'
            + 'of delta-workloads$_{'
            + f'{data.iteration - 1}, {data.iteration}'
            + '}$',
            mappable=plot_collection,
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
                'delta_workloads-quantiles.png'
            ),
            dpi=self.dpi
        )
        # plt.show()
        plt.close()

    def plot_workload_histogram(self, data: Data, sim: Simulation):
        # setup figure

        plt.style.use(self.plt_theme)
        _fig, ax = plt.subplots()
        ax.set_title(
            f'density-function of workloads'
            + '$_{'
            + f'{data.iteration}'
            + '}$ > 0'
        )

        #  set cmap

        if self.is_light:
            fc = 'k'
            ec = 'k'
        else:
            fc = 'w'
            ec = 'w'

        # plot data

        num_bins = int(np.ceil(data.workloads.max)) - \
            int(np.floor(data.workloads.min))
        _n, _bins, _patches = ax.hist(
            data.workloads.raw_nz,
            bins=num_bins,
            fc=fc,
            ec=ec
        )

        # finalize plot

        ax.set_xlabel('workloads')
        ax.set_ylabel('amount of occurence')
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


def light():
    return Machine(is_light=True)


def dark():
    return Machine(is_light=False)
