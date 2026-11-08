#!/usr/bin/env python3
#
from hdrh.histogram import HdrHistogram
import matplotlib.ticker as ticker
import matplotlib.pyplot as plt
import pandas as pd
import sys

MSECS_PER_SEC = 1000

def main() -> None:
    if not sys.argv[1]:
        print("Please provide a csv file containing latency values to plot")
        sys.exit(1)
    df = pd.read_csv(sys.argv[1])
    histogram = HdrHistogram(1, MSECS_PER_SEC, 4)
    PERCENTILES = [25.0, 50.0, 90.0, 99.9, 99.99]
    data = [
        [percentile / 100, histogram.get_value_at_percentile(percentile)]
        for percentile in PERCENTILES
    ]

    hist_df = pd.DataFrame(data, columns=['Percentile', 'Value'])
    fig, ax = plt.subplots()
    ax.plot(hist_df["Percentile"], hist_df["Value"])
    ax.grid()
    ax.set_title('Latency by Percentile Distribution')
    ax.set_xlabel('Percentile (%)')
    ax.set_ylabel('Latency (milliseconds)')
    ax.set_xscale('logit')
    plt.xticks([value / 100.0 for value in PERCENTILES])
    majors = [f"{value}%" for value in PERCENTILES]
    ax.xaxis.set_major_formatter(ticker.FixedFormatter(majors))
    ax.xaxis.set_minor_formatter(ticker.NullFormatter())
    fig.savefig('histogram-hdr.png', dpi=300)

if __name__=="__main__":
    main()
