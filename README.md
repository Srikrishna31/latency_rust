# latency_rust

Learn about latency concepts and ways to reduce delays in software systems, following the book _Latency_ by Pekka Enberg.

## Why gnuplot?

The `periodic_gate` benchmark uses Criterion to measure how the gate behaves under repeated polling. Installing `gnuplot` allows Criterion to generate visual reports of timing measurements and compare them with previous runs. These plots make it easier to spot performance regressions and distinguish meaningful changes from normal benchmark variance.

## Installing gnuplot on Windows

`gnuplot` enables Criterion to generate benchmark plots. Install it with Windows Package Manager:

```powershell
winget install --id gnuplot.gnuplot --exact
```

Restart your terminal and verify the installation:

```powershell
gnuplot --version
```

If the package ID is unavailable, search for the current one:

```powershell
winget search gnuplot
```

## Installing gnuplot on Linux

Install `gnuplot` with your distribution's package manager:

```sh
# Debian or Ubuntu
sudo apt update && sudo apt install gnuplot

# Fedora or RHEL
sudo dnf install gnuplot

# Arch Linux
sudo pacman -S gnuplot
```

Verify the installation:

```sh
gnuplot --version
```

With `gnuplot` on your `PATH`, run the periodic-gate benchmark with:

```powershell
cargo bench --bench periodic_gate
```
