#! /usr/bin/env python3

import json
import subprocess
import sys


def run_inner(args):
    print("Running `{}`...".format(" ".join(args)))
    ret = subprocess.call(args) == 0
    print("")
    return ret


def run(mcu, cargo_cmd):
    if mcu == "":
        return run_inner(cargo_cmd)
    else:
        return run_inner(cargo_cmd + ["--features={}".format(mcu)])


def main():
    cargo_meta = json.loads(
        subprocess.check_output("cargo metadata --no-deps --format-version=1",
                       shell=True,
                       universal_newlines=True)
        )

    crate_info = cargo_meta["packages"][0]

    features = ["{},usb_fs,can,i2s,fsmc_lcd".format(x)
            for x in crate_info["features"].keys()
            if x.startswith("stm32f4")]

    if 'size_check' in sys.argv:
        cargo_cmd = ['cargo', 'build', '--release']
    else:
        cargo_cmd = ['cargo', 'check']

    if not all(map(lambda f: run(f, cargo_cmd),
                   features)):
        sys.exit(-1)


if __name__ == "__main__":
    main()

