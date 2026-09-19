# AstraFetch

**A modern Linux system information and realtime terminal monitor.**

Created by **Lưu Minh Quang - Astra**.

AstraFetch is a native Rust CLI/TUI intended as a modern replacement for Neofetch/Fastfetch. It keeps static information cached, reads Linux realtime counters directly, and does not use telemetry or background daemons.

## Features

- OS logo on the **left**, system information on the **right**.
- Linux distribution auto-detection with local Unicode/ANSI logos.
- Static mode: `astrafetch`.
- Realtime TUI: `astrafetch --watch`.
- Live Linux uptime from `/proc/uptime`.
- CPU, memory, disk, GPU, temperature and network counters.
- NVIDIA data via a cached `nvidia-smi` query between refreshes; it is never called per frame.
- Themes: `default`, `cyber`, `aurora`, `matrix`, `minimal`, `monochrome`.
- JSON output: `astrafetch --json`.
- Minimal output: `astrafetch --minimal`.
- `NO_COLOR` and `TERM=dumb` fallbacks.
- TTY/SSH friendly; no GUI, X11 or Wayland requirement.
- `asfetch` is a symlink to the single `astrafetch` binary.
- x86_64 and ARM64 build workflows.

## Installation

### Ubuntu / Debian package

After downloading a release package:

```bash
sudo apt install ./astrafetch_1.0.0_amd64.deb
```

Then:

```bash
astrafetch
asfetch
```

### From source

Install Rust stable using your distribution's package manager or the official Rust installation method, then:

```bash
cargo build --release
./target/release/astrafetch
./target/release/astrafetch --watch
./target/release/astrafetch --json | jq
```

## Usage

```text
astrafetch
asfetch
astrafetch --watch
astrafetch --minimal
astrafetch --json
astrafetch --theme cyber
astrafetch --logo ubuntu
astrafetch --logo auto
astrafetch --logo custom
astrafetch --logo none
astrafetch --no-animation
astrafetch --refresh-rate 2 --watch
```

Realtime keyboard controls:

- `q` / `Esc`: exit
- `p`: pause animation
- `r`: refresh immediately
- `h`: help

## Themes

```bash
asfetch --theme cyber
```

Available themes:

```text
default
cyber
aurora
matrix
minimal
monochrome
```

## OS Logos

Supported auto-detection includes Ubuntu, Debian, Arch Linux, Fedora, openSUSE, Alpine, Manjaro, Linux Mint, Kali Linux, Pop!_OS, Gentoo, Void Linux, Rocky Linux, AlmaLinux, CentOS and NixOS. Unknown systems fall back to Linux.

A custom logo is read from:

```text
~/.config/astrafetch/logo.txt
```

No logo is downloaded at runtime.

## Configuration

Optional file:

```text
~/.config/astrafetch/config.toml
```

Example:

```toml
theme = "cyber"
logo = "auto"
animation = true
refresh_rate = 1
show_os = true
show_kernel = true
show_cpu = true
show_gpu = true
show_memory = true
show_disk = true
show_network = true
show_temperature = true
show_uptime = true
author = true
```

The file is optional. Internal defaults are used when it does not exist.

## JSON

```bash
asfetch --json | jq
```

JSON is emitted without ANSI escape sequences and includes author/version metadata, OS, kernel, uptime, CPU, memory, GPU, disk, network, temperature, shell, terminal and desktop information.

## Performance and security

AstraFetch does not run as root, does not edit shell startup files, does not add APT repositories, does not send telemetry, does not make network requests, and does not execute shell commands from user input. Realtime information comes from Linux procfs/sysfs and a small set of explicitly invoked local tools when available (`ip`, `nvidia-smi`, `df`).

## Debian packaging

The repository contains Debian packaging metadata under `packaging/debian/` and a standalone package helper:

```bash
./packaging/build-deb.sh
```

Expected output on amd64:

```text
astrafetch_1.0.0_amd64.deb
```

Install it with:

```bash
sudo apt install ./astrafetch_1.0.0_amd64.deb
```

The package installs:

```text
/usr/bin/astrafetch
/usr/bin/asfetch -> astrafetch
/usr/share/doc/astrafetch/
```

## ARM64

Build natively or with a configured cross compiler:

```bash
cargo build --release --target aarch64-unknown-linux-gnu
```

The release workflow produces an ARM64 Debian artifact.

## APT repository readiness

The project is deliberately **not** hard-coded to a repository domain that may not exist yet. A future repository can publish `Packages`, `Packages.gz`, `Release`, `InRelease` and GPG signatures and then instruct users to use:

```bash
sudo apt update
sudo apt install astrafetch
```

Do not use `curl | bash` as an installation mechanism.

## Development

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## Project layout

```text
astrafetch/
├── Cargo.toml
├── README.md
├── LICENSE
├── Makefile
├── assets/
├── packaging/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── error.rs
│   ├── system/
│   ├── ui/
│   └── output/
├── tests/
└── .github/workflows/
```

## Screenshots

The TUI is intentionally designed around the requested concept:

```text
┌───────────────────────┬──────────────────────────────────────┐
│       OS LOGO         │            ASTRΛFETCH                │
│       Ubuntu          │  OS          Ubuntu 24.04 LTS       │
│                       │  Kernel      6.8.x                  │
│                       │  Host        Astra-PC               │
│                       │  Uptime      03d 14h 28m 51s        │
│                       │                                      │
│                       │  CPU         Ryzen 7 7800X3D        │
│                       │  Memory      12.4 / 32.0 GiB       │
│                       │  Disk        428 / 1000 GiB        │
└───────────────────────┴──────────────────────────────────────┘
```

## Author

**Lưu Minh Quang - Astra**

Created by Lưu Minh Quang - Astra.
