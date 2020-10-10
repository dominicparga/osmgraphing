#!/usr/bin/env python

import argparse
import visualizing as vis
import os


def parse_cmdline():
    # define args and parse them

    parser = argparse.ArgumentParser(
        description='Visualize results from balancer-binary.')

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
        'style': args.style
    }


if __name__ == '__main__':
    params = parse_cmdline()

    sim = vis.Simulation(
        results_dir=params['results_dir'],
    )

    if params['style'] == 'dark':
        plotting_machine = vis.plotting.dark()
    if params['style'] == 'light':
        plotting_machine = vis.plotting.light()

    vis.run(sim=sim, vis=plotting_machine)
