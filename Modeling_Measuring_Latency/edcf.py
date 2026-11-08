#!/usr/bin/env python3

import matplotlib.ticker as tck
import matplotlib.pyplot as plt
import statsmodels.api as sm
import pandas as pd
import numpy as np
import sys

"""
The empirical cumulative distribution function (eCDF) is a distribution function that
represents the proportion of empirically measured samples falling below each value.
Unlike histograms, which show frequency in bins, the eCDF displays the cumulative percentage
of samples at or below each latency value, creating a smooth curve that rises from 0% to 100%.
This visualization is particularly valuable for latency analysis because it directly answers questions
like "What percentage of requests are complete within 100ms?" and "What latency do 95% of users experience?"
For latency measurements, this means you can quickly identify whether your system meets service level agreement(SLA)
requirements and spot the tail behaviour that affects user experience.
"""
def main()->None:
    if not sys.argv[1]:
        print("Please provide a csv file containing latency values to plot")
        sys.exit(1)
    df = pd.read_csv(sys.argv[1])
    values = df["Latency_secs"] * 1000
    ecdf = sm.distributions.ECDF(values)
    x = np.linspace(min(values), max(values))
    y = ecdf(x)
    fig, ax = plt.subplots()
    plt.plot(x,y)
    ax.xaxis.set_minor_locator(tck.AutoMinorLocator())
    plt.grid()
    plt.xlim((min(x), max(x)))
    plt.xlabel('Latency (in milliseonds)')
    plt.ylabel('Cumulative density')
    plt.yticks(np.arange(0, 1.1, 0.1))
    plt.savefig('ecdf.png', dpi=300)

if __name__=="__main__":
    main()
