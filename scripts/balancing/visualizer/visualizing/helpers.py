import numpy as np
import matplotlib.colors as colors

from matplotlib.colors import Normalize


def _check_inputs(vmin, vcenter, vmax):
    if vmax < vmin:
        raise ValueError('Should not be: vmax < vmin')
    if vcenter is not None:
        if vcenter < vmin:
            raise ValueError('Should not be: vcenter < vmin')
        elif vmax < vcenter:
            raise ValueError('Should not be: vmax < vcenter')
        elif (not vcenter < vmax) or (not vmin < vcenter):
            # vcenter is equal to vmin or vmax
            vcenter = None
    return vmin, vcenter, vmax


class TwoSlopeLoggedNorm(Normalize):
    '''
    Maps values between `[vmin, vmax]` to `[1, base]` before using log_base to map the values to `[0, 1]`.
    The base can be set when initializing this norm, resulting in different contrasts.

    If a `vcenter != vmin, vmax` is provided, values are mapped

    - from `[vmin, vcenter]` to `[-base, -1]` and inverted to `[1, base]`.
    - from `[vcenter, vmax]` to `[1, base]`.

    Using the log_base on both groups results in values between `[0, 1]` for each group.
    Values from the group of values between `[vmin, vcenter]` are inverted back to `[-1, 0]` and merged with the positive values `[0, 1]` from the group `[vcenter, vmax]`.
    A linear interpolation from `[-1, 0, 1]` to `[0, 0.5, 1]` finalizes the normalization, where `0.5` equals `vcenter`.
    '''

    def __init__(self, vcenter=None, base=2.0, **kwargs):
        super().__init__(**kwargs)
        self.base = base
        self.vcenter = vcenter

    def __call__(self, value, clip=None):
        '''
        In the following explanation, vcenter is expected to be zero.
        This helps understanding the comments.

        All positive values are mapped from [0, max] to [1, base] before
        log is taken, resulting in a value in [0, 1].

        Negative values are negated before being treated as a positive
        value. After mapping to [0, 1], the resulting value is negated
        again, resulting in a value in [-1, 0].
        '''

        # set and check data-dependent values

        vmin = self.vmin
        vmax = self.vmax
        if vmin is None:
            vmin = np.min(value)
        if vmax is None:
            vmax = np.max(value)
        vcenter = self.vcenter
        vmin, vcenter, vmax = _check_inputs(
            vmin=vmin,
            vcenter=vcenter,
            vmax=vmax
        )

        base = self.base
        if not base > 1.0:
            raise ValueError('Base should be greater than 1.0')
        log_base = np.log(base)

        # if vcenter is not set, map from [0, 1] to [1, base]
        # to use log for mapping into [0, 1] again.
        if vcenter is None:
            xp, fp = [vmin, vmax], [1.0, base]
            mapped_value = np.interp(x=value, xp=xp, fp=fp)
            mapped_value = np.log(mapped_value) / log_base
        else:
            xp, fp = [vmin, vcenter, vmax], [-1.0, 0.0, 1.0]
            mapped_value = 2.0 * np.interp(x=value, xp=xp, fp=fp)

            for i in range(len(mapped_value)):
                v = mapped_value[i]

                # positive value in (0, 1]
                if v > 0.0:
                    # 1) map to (1, base]
                    # 2) use log to map back to (0, 1]
                    v = np.log((base-1.0)*v + 1.0) / log_base
                # negative value in [1, 0)
                elif v < 0.0:
                    # 1) mirror to (0, 1]
                    # 2) map to (1, base]
                    # 3) use log to map back to (0, 1]
                    # 4) mirror back to [-1, 0)
                    v = -np.log(-(base-1.0)*v + 1.0) / log_base

                # v is in [-1, 1] and should be shifted to [0, 1]
                # -> shift negative values into [0, 0.5)
                # -> shift positive values into (0.5, 1]
                # -> shift zero to 0.5
                mapped_value[i] = v / 2.0 + 0.5
        return np.ma.masked_array(mapped_value)


class SigmoidNorm(Normalize):
    '''
    Maps positive and negative values to[0.0, 1.0]:
    1) Map via linear interpolation
       from [vmin, vcenter, vmax] to[-5, 0, 5]
    1) Map via sigmoid-function
       from [-1, 0.5, 1.0] to[0.0 + eps, 0.5, 1.0 - eps]
    2) Map via linear interpolation to[0.0, 1.0] using TwoSlopeNorm
       from matplotlib

    https: // stackoverflow.com/a/42140710
    https: // de.wikipedia.org/wiki/Sigmoidfunktion
    '''

    @staticmethod
    def exp_sigmoid(x, x_scale):
        '''
        If x_scale < 1.0, the results are less nice.
        '''

        # Input-values are scaled and shifted from [0, 0.5, 1] to [-6, 0.0, 6]
        return 1.0 / (1.0 + np.exp(-12.0 * x_scale * (x-0.5)))

    @staticmethod
    def log_sigmoid(x, x_scale):
        '''
        If x_scale < 1.0, the results are less nice.
        '''

        # Input-values are scaled and shifted from [0, 0.5, 1] to [-6, 0.0, 6]
        return -np.log(1.0/x - 1.0) / (12.0 * x_scale) + 0.5

    def __init__(self, intensity=1.0, vmin=None, vmax=None, vcenter=0.0, **kwargs):
        super().__init__(vmin=vmin, vmax=vmax, **kwargs)
        self.intensity = intensity
        self.vmin = vmin
        self.vmax = vmax
        self.vcenter = vcenter

    def __call__(self, value, clip=None):
        '''
        The clip argument is unused.
        '''

        # set and check data-dependent values

        vmin = self.vmin
        vmax = self.vmax
        if vmin is None:
            vmin = np.min(value)
        if vmax is None:
            vmax = np.max(value)
        vcenter = self.vcenter
        vmin, vcenter, vmax = _check_inputs(
            vmin=vmin,
            vcenter=vcenter,
            vmax=vmax
        )
        intensity = self.intensity

        # normalize values

        if vcenter is None:
            xp, fp = [vmin, vmax], [0.0, 1.0]
        else:
            xp, fp = [vmin, vcenter, vmax], [0.0, 0.5, 1.0]
        values = np.interp(x=value, xp=xp, fp=fp)

        if intensity < 0.0:
            intensity = -intensity
            if intensity < 1.0:
                sigmoid = SigmoidNorm.log_sigmoid(x=values, x_scale=1.0)
                values = values - intensity * (values - sigmoid)
            else:
                sigmoid = SigmoidNorm.log_sigmoid(x=values, x_scale=intensity)
                values = values - 1.0 * (values - sigmoid)
        else:
            if intensity < 1.0:
                sigmoid = SigmoidNorm.exp_sigmoid(x=values, x_scale=1.0)
                values = values - intensity * (values - sigmoid)
            else:
                sigmoid = SigmoidNorm.exp_sigmoid(x=values, x_scale=intensity)
                values = values - 1.0 * (values - sigmoid)

        return np.ma.masked_array(values)
