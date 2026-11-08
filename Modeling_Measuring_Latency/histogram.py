#!/usr/bin/env python3
#
import matplotlib.pyplot as plt
import pandas as pd
import sys

def main():
    if not sys.argv[1]:
        print("Please provide a csv file containing latency values to plot")
        sys.exit(1)
    df = pd.read_csv(sys.argv[1])
    plt.hist(df["Latency_secs"] * 1000, bins='auto')
    plt.ylabel(f"Frequency (N = {len(df)})")
    plt.xlabel('Latency (in milliseconds)')
    plt.savefig('histogram.png', dpi=300)

if __name__=="__main__":
    main()
