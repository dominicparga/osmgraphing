import math
import numpy as np
import matplotlib as plt
import matplotlib.colors as colors


class Norm(colors.Normalize):
    '''
    Maps positive and negative values to [0.0, 1.0]
    '''

    def __init__(self, vmin=None, vmax=None, vcenter=None, clip=False):
        self.vcenter = vcenter
        super().__init__(vmin=vmin, vmax=vmax, clip=clip)

    def __call__(self, value, clip=None):
        if self.vmin is None:
            self.vmin = np.min(value)
        if self.vmax is None:
            self.vmax = np.max(value)
        self.check_inputs()

        new_value = np.array(value)

        # clip values

        if clip is None:
            clip = self.clip
        if clip:
            new_value = np.array(np.clip(
                a=new_value, a_min=self.vmin, a_max=self.vmax, out=new_value
            ))

        # map values to [0.0, 1.0]
        if self.vmin == self.vmax:
            new_value.fill(0.5)
        else:
            new_value = self.do_mapping(new_value)

        return np.ma.masked_array(new_value)

    def check_inputs(self):
        if self.vmax < self.vmin:
            raise ValueError('Should not be: vmax < vmin')
        if self.vcenter is not None:
            if self.vcenter < self.vmin:
                raise ValueError('Should not be: vcenter < vmin')
            elif self.vmax < self.vcenter:
                raise ValueError('Should not be: vmax < vcenter')
            elif (not self.vcenter < self.vmax) or (not self.vmin < self.vcenter):
                # vcenter is equal to vmin or vmax
                self.vcenter = None

    def do_mapping(self, value):
        '''
        Per default, this is a linear mapping via linear interpolation.
        '''

        # in __init__, vcenter is compared with vmin and vmax
        if self.vcenter is None:
            xp, fp = [self.vmin, self.vmax], [0.0, 1.0]
        else:
            xp, fp = [self.vmin, self.vcenter, self.vmax], [0.0, 0.5, 1.0]

        return np.interp(x=value, xp=xp, fp=fp)


class LogNorm(Norm):
    '''
    Maps positive and negative values to [0.0, 1.0] using log2,
    where 0.5 equals vcenter.
    '''

    def __init__(self, base=2.0, **kwargs):
        self.base = base
        super().__init__(**kwargs)

    def check_inputs(self):
        super().check_inputs()
        if not 1.0 < self.base:
            raise ValueError('Should not be: base <= 1.0')

    def do_mapping(self, value):
        '''
        In the following explanation, vcenter is expected to be zero.
        This helps understanding the comments.

        All positive values are mapped from [0.0, max] to [1.0, base]
        before log is taken, resulting in a value in [0.0, 1.0].

        Negative values are negated before being treated as a positive
        value. After mapping to [0.0, 1.0], the resulting value is
        negated again, resulting in a value in [-1.0, 0.0].
        '''

        self.check_inputs()

        log_base = np.log(self.base)

        # if vcenter is not set, map from [0.0, 1.0] to [1.0, base]
        # to use log for mapping into [0.0, 1.0] again.
        if self.vcenter is None:
            mapped_value = 1.0 + (self.base - 1.0) * super().do_mapping(value)
            return np.log(mapped_value) / log_base

        # values in [0.0, 1.0] via linear interpolation
        # -> mapped to [-1.0, 1.0] where 0.0 equals vcenter
        mapped_value = 2.0 * super().do_mapping(value) - 1.0

        for i in range(len(mapped_value)):
            v = mapped_value[i]

            # positive value in (0.0, 1.0]
            if v > 0.0:
                # 1) map to (1.0, base]
                # 2) use log to map back to (0.0, 1.0]
                v = np.log((self.base-1.0)*v + 1.0) / log_base
            # negative value in [-1.0, 0.0)
            elif v < 0.0:
                # 1) mirror to (0.0, 1.0]
                # 2) map to (1.0, base]
                # 3) use log to map back to (0.0, 1.0]
                # 4) mirror back to [-1.0, 0.0)
                v = -np.log(-(self.base-1.0)*v + 1.0) / log_base

            # v is in [-1.0, 1.0] and should be shifted to [0.0, 1.0]
            # -> shift negative values into [0.0, 0.5)
            # -> shift positive values into (0.5, 1.0]
            # -> shift zero to 0.5
            mapped_value[i] = v / 2.0 + 0.5

        return mapped_value


class Scatter():
    def __init__(
        self,
        *,
        norm,
        cmap,
        s=2,
        alpha=1.0,
        edgecolors='none'
    ):
        self._norm = norm
        self._cmap = cmap
        self._s = s
        self._alpha = alpha
        self._edgecolors = edgecolors

    @ property
    def norm(self):
        return self._norm

    @ property
    def cmap(self):
        return self._cmap

    @ property
    def s(self):
        return self._s

    @ property
    def alpha(self):
        return self._alpha

    @ property
    def edgecolors(self):
        return self._edgecolors

    def as_dict(self):
        return {
            'norm': self._norm,
            'cmap': self._cmap,
            's': self._s,
            'alpha': self._alpha,
            'edgecolors': self._edgecolors
        }


class Hist():
    def __init__(self, fc, ec):
        self._fc = fc
        self._ec = ec

    @ property
    def fc(self):
        return self._fc

    @ property
    def ec(self):
        return self._ec


class Style():
    '''
    Just a struct of values

    # available default-styles
    # https://matplotlib.org/3.1.0/gallery/style_sheets/style_sheets_reference.html

    # choose colormaps
    # https://matplotlib.org/3.1.0/tutorials/colors/colormaps.html
    '''

    def __init__(
        self, *, plt_style, pos_integer: Scatter, integer: Scatter, hist: Hist
    ):
        self._plt_style = plt_style
        self._pos_integer = pos_integer
        self._integer = integer
        self._hist = hist

    @ property
    def plt(self):
        return self._plt_style

    @ property
    def integer(self) -> Scatter:
        return self._integer

    @ property
    def pos_integer(self) -> Scatter:
        return self._pos_integer

    @ property
    def hist(self) -> Hist:
        return self._hist

    @ staticmethod
    def light():
        return Style(
            plt_style='default',
            pos_integer=Scatter(
                norm=LogNorm(vcenter=0.0, base=2.0),
                cmap='binary'
                # cmap='cubehelix_r',
                # cmap='PuRd',
            ),
            integer=Scatter(
                norm=LogNorm(vcenter=0.0, base=2.0),
                # cmap='PRGn_r',
                cmap='seismic',
                # cmap='PiYG_r', # nice but too lighten
            ),
            hist=Hist(
                fc='k',
                ec='k'
            )
        )

    @ staticmethod
    def dark():
        return Style(
            plt_style='dark_background',
            pos_integer=Scatter(
                norm=Norm,
                cmap='cubehelix',
            ),
            integer=Scatter(
                norm=Norm,
                # cmap='cividis',
                # cmap='winter',
                cmap='twilight',
            ),
            hist=Hist(
                fc='w',
                ec='w'
            )
        )
