#!/usr/bin/env python

import argparse
import visualization as vis
import os


def parse_cmdline():
    # define args and parse them

    parser = argparse.ArgumentParser(
        description='Visualize results from balancer-binary.')

    help_msg = 'Maximum number of iterations, starting with the provided index.'
    parser.add_argument(
        '--num-iter',
        metavar=('NUM_ITER'),
        required=True,
        type=int,
        help=help_msg
    )
    help_msg = 'Directory where results are laying.'
    parser.add_argument(
        '--results-dir',
        metavar=('RESULTS_DIR'),
        required=True,
        help=help_msg
    )
    help_msg = 'Dark or light style'
    parser.add_argument(
        '--style',
        metavar=('STYLE'),
        choices=['dark', 'light'],
        default='light',
        required=False,
        help=help_msg
    )

    # finalize and return

    args = parser.parse_args()

    cwd = os.path.join(os.getcwd(), os.path.dirname(__file__))
    results_dir = os.path.join(cwd, '..', '..', '..')
    return {
        'results_dir': os.path.join(results_dir, args.results_dir),
        'num_iter': args.num_iter,
        'style': args.style
    }


if __name__ == '__main__':
    params = parse_cmdline()

    sim = vis.Simulation(
        results_dir=params['results_dir'],
        num_iter=params['num_iter']
    )

    if params['style'] == 'dark':
        style = vis.styling.Style.dark()
    if params['style'] == 'light':
        style = vis.styling.Style.light()

    vis.run(sim=sim, style=style)
