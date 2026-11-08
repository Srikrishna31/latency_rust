#!/usr/bin/env python3
#
from ping3 import ping
import pandas as pd
import argparse
import time


def build_args() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument('host')
    parser.add_argument('samples')

    return parser

def main() -> None:
    args = build_args().parse_args()
    interval = 1
    values = []
    for i in range(int(args.samples)):
        value = ping(args.host, timeout=interval)
        if not value:
            print("warning: ping timed out, latency outlier not measured")
            continue
        values.append(value)
        time.sleep(interval - value)

    df = pd.DataFrame({"Latency_secs": values})
    output = "ping-%s.csv" % (args.host)
    df.to_csv(output, index=False)


if __name__=="__main__":
    main()
