# latency_rust

Learn about latency concepts and ways to reduce delays in software systems, following the book _Latency_ by Pekka Enberg.

## Python scripts

The root `pyproject.toml` manages dependencies for `Modeling_Measuring_Latency/ping.py` and future Python scripts. It requires Python 3.11 or newer and installs `ping3` and `pandas`. `argparse` and `time` are part of Python's standard library and need no additional dependencies.

### Using pip

Run these commands from the repository root with a Python 3.11+ interpreter:

```sh
python3 -m venv .venv
source .venv/bin/activate
python -m pip install -e .
python Modeling_Measuring_Latency/ping.py example.com 10
```

On Windows PowerShell, create the environment with `py -3.11 -m venv .venv` and activate it with `.\.venv\Scripts\Activate.ps1` instead. The installation and script commands are then the same.

The example requests 10 ping samples and writes successful measurements to `ping-example.com.csv` in the current directory. ICMP permissions and firewall rules may affect whether ping works on your system.

### Using uv instead

If you have [uv](https://docs.astral.sh/uv/) installed, it can manage the environment without manual activation:

```sh
uv sync
uv run python Modeling_Measuring_Latency/ping.py example.com 10
```

`uv sync` creates `.venv` and generates `uv.lock`. Commit `uv.lock` if you use uv and want reproducible Python dependency versions; subsequent checkouts can use `uv sync --locked`.

Add dependencies for future scripts to `[project].dependencies` in `pyproject.toml`, then rerun the installation command. With uv, `uv add <package>` updates the manifest and lockfile for you. The setuptools configuration deliberately disables Python module discovery: this project installs dependency metadata, not the Rust source or standalone scripts as Python packages.

## Cargo and Python in the same repository

`Cargo.toml` and `pyproject.toml` can sit side by side in the repository root. They serve independent toolchains:

- **Rust:** Cargo reads `Cargo.toml`, resolves Rust crates with `Cargo.lock`, and builds into `target/`. Continue using commands such as `cargo test` and `cargo bench --bench periodic_gate`.
- **Python:** pip or uv reads `pyproject.toml` and installs Python dependencies into `.venv/`. uv additionally uses `uv.lock`; pip alone does not generate that lockfile.

Neither toolchain reads or modifies the other's manifest. Activating the Python environment does not prevent you from running Cargo commands. No Rust/Python build integration (such as maturin or PyO3) is needed because these scripts do not import the Rust library. `gnuplot` remains a separate system dependency, installed as described below.

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
