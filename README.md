# <img src="https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/nautilus-trader-logo.png" width="500">

[![codecov](https://codecov.io/gh/nautechsystems/nautilus_trader/branch/master/graph/badge.svg?token=DXO9QQI40H)](https://codecov.io/gh/nautechsystems/nautilus_trader)
[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/nautechsystems/nautilus_trader)
![pythons](https://img.shields.io/pypi/pyversions/nautilus_trader)
![pypi-version](https://img.shields.io/pypi/v/nautilus_trader)
![pypi-format](https://img.shields.io/pypi/format/nautilus_trader?color=blue)
[![Downloads](https://pepy.tech/badge/nautilus-trader)](https://pepy.tech/project/nautilus-trader)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

| Nhánh     | Phiên bản                                                                                                                                                                                                                   | Trạng thái                                                                                                                                                                                        |
| :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `master`  | [![version](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnautechsystems%2Fnautilus_trader%2Fmaster%2Fversion.json)](https://packages.nautechsystems.io/simple/nautilus-trader/index.html)  | [![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=nightly)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml) |
| `nightly` | [![version](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnautechsystems%2Fnautilus_trader%2Fnightly%2Fversion.json)](https://packages.nautechsystems.io/simple/nautilus-trader/index.html) | [![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=nightly)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml) |
| `develop` | [![version](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnautechsystems%2Fnautilus_trader%2Fdevelop%2Fversion.json)](https://packages.nautechsystems.io/simple/nautilus-trader/index.html) | [![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=develop)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml) |

| Nền tảng           | Rust    | Python     |
| :----------------- | :------ | :--------- |
| `Linux (x86_64)`   | 1.87.0+ | 3.11-3.13  |
| `Linux (ARM64)`    | 1.87.0+ | 3.11-3.13  |
| `macOS (ARM64)`    | 1.87.0+ | 3.11-3.13  |
| `Windows (x86_64)` | 1.87.0+ | 3.11-3.13  |

- **Tài liệu**: <https://nautilustrader.io/docs/>
- **Website**: <https://nautilustrader.io>
- **Hỗ trợ**: [support@nautilustrader.io](mailto:support@nautilustrader.io)

## Giới thiệu

NautilusTrader là một nền tảng giao dịch thuật toán mã nguồn mở, hiệu năng cao và cấp độ sản xuất,
cung cấp cho các nhà giao dịch định lượng khả năng kiểm tra lại danh mục chiến lược giao dịch tự động
trên dữ liệu lịch sử với engine điều khiển bằng sự kiện, và cũng triển khai những chiến lược tương tự trong môi trường thực tế mà không cần thay đổi mã.

Nền tảng này *ưu tiên AI*, được thiết kế để phát triển và triển khai các chiến lược giao dịch thuật toán trong một
môi trường Python gốc hiệu năng cao và mạnh mẽ. Điều này giúp giải quyết thách thức đồng nhất của việc giữ môi trường
nghiên cứu/kiểm tra lại Python nhất quán với môi trường giao dịch thực tế trong sản xuất.

Thiết kế, kiến trúc và triết lý triển khai của NautilusTrader ưu tiên tính đúng đắn và an toàn của phần mềm ở
mức cao nhất, với mục tiêu hỗ trợ các workload kiểm tra lại hệ thống giao dịch và triển khai thực tế
có tính chất quan trọng trong Python gốc.

Nền tảng này cũng có tính universal và bất khả tri về lớp tài sản — bất kỳ REST API hoặc WebSocket feed nào đều có thể được tích hợp thông qua các
adapter mô-đun. Nó hỗ trợ giao dịch tần số cao trên nhiều loại tài sản và loại công cụ khác nhau
bao gồm FX, Cổ phiếu, Hợp đồng tương lai, Quyền chọn, Crypto và Cá cược, cho phép hoạt động liền mạch trên nhiều sàn giao dịch cùng lúc.

![nautilus-trader](https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/nautilus-trader.png "nautilus-trader")

## Tính năng

- **Nhanh**: Core được viết bằng Rust với mạng bất đồng bộ sử dụng [tokio](https://crates.io/crates/tokio).
- **Đáng tin cậy**: An toàn về kiểu và luồng được hỗ trợ bởi Rust, với tùy chọn lưu trữ trạng thái Redis.
- **Di động**: Độc lập với hệ điều hành, chạy trên Linux, macOS và Windows. Triển khai sử dụng Docker.
- **Linh hoạt**: Adapter mô-đun có nghĩa là bất kỳ REST API hoặc WebSocket feed nào đều có thể được tích hợp.
- **Nâng cao**: Time in force `IOC`, `FOK`, `GTC`, `GTD`, `DAY`, `AT_THE_OPEN`, `AT_THE_CLOSE`, các loại lệnh nâng cao và trigger điều kiện. Chỉ thị thực thi `post-only`, `reduce-only`, và icebergs. Lệnh ngẫu nhiên bao gồm `OCO`, `OUO`, `OTO`.
- **Có thể tùy chỉnh**: Thêm các component tùy chỉnh do người dùng định nghĩa, hoặc lắp ráp toàn bộ hệ thống từ đầu tận dụng [cache](https://nautilustrader.io/docs/latest/concepts/cache) và [message bus](https://nautilustrader.io/docs/latest/concepts/message_bus).
- **Kiểm tra lại**: Chạy với nhiều sàn giao dịch, công cụ và chiến lược đồng thời sử dụng dữ liệu lịch sử về quote tick, trade tick, bar, order book và dữ liệu tùy chỉnh với độ phân giải nanosecond.
- **Thực tế**: Sử dụng cùng implementation chiến lược giữa kiểm tra lại và triển khai thực tế.
- **Đa sàn giao dịch**: Khả năng đa sàn giao dịch tạo điều kiện cho chiến lược tạo thị trường và arbitrage thống kê.
- **Đào tạo AI**: Engine kiểm tra lại đủ nhanh để được sử dụng để đào tạo các agent giao dịch AI (RL/ES).

![Alt text](https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/nautilus-art.png "nautilus")

> *nautilus - from ancient Greek 'sailor' and naus 'ship'.*
>
> *The nautilus shell consists of modular chambers with a growth factor which approximates a logarithmic spiral.
> The idea is that this can be translated to the aesthetics of design and architecture.*

## Tại sao chọn NautilusTrader?

- **Python điều khiển sự kiện hiệu năng cao**: Các component core native binary.
- **Tính nhất quán giữa kiểm tra lại và giao dịch thực tế**: Mã chiến lược giống hệt nhau.
- **Giảm rủi ro vận hành**: Chức năng quản lý rủi ro nâng cao, độ chính xác logic và an toàn về kiểu.
- **Khả năng mở rộng cao**: Message bus, component và actor tùy chỉnh, dữ liệu tùy chỉnh, adapter tùy chỉnh.

Theo truyền thống, nghiên cứu chiến lược giao dịch và kiểm tra lại có thể được tiến hành bằng Python
sử dụng các phương pháp vector hóa, với chiến lược sau đó cần phải được triển khai lại theo cách điều khiển sự kiện hơn
sử dụng C++, C#, Java hoặc các ngôn ngữ tĩnh khác. Lý do ở đây là mã kiểm tra lại vector hóa không thể
biểu hiện sự phức tạp chi tiết phụ thuộc vào thời gian và sự kiện của giao dịch thời gian thực, nơi các ngôn ngữ biên dịch đã
được chứng minh là phù hợp hơn do hiệu năng vốn có cao hơn và an toàn về kiểu.

Một trong những lợi thế chính của NautilusTrader ở đây, là bước triển khai lại này hiện được tránh khỏi - vì các component core quan trọng của nền tảng
đã được viết hoàn toàn bằng [Rust](https://www.rust-lang.org/) hoặc [Cython](https://cython.org/).
Điều này có nghĩa là chúng ta đang sử dụng đúng công cụ cho công việc, nơi các ngôn ngữ lập trình hệ thống biên dịch binary hiệu năng,
với các module mở rộng CPython C sau đó có thể cung cấp môi trường Python gốc, phù hợp cho các nhà giao dịch định lượng chuyên nghiệp và công ty giao dịch.

## Why Python?

Python was originally created decades ago as a simple scripting language with a clean straightforward syntax.
It has since evolved into a fully fledged general purpose object-oriented programming language.
Based on the TIOBE index, Python is currently the most popular programming language in the world.
Not only that, Python has become the *de facto lingua franca* of data science, machine learning, and artificial intelligence.

developer/user communities.
However, Python has performance and typing limitations for large-scale, latency-sensitive systems. Cython addresses many of these issues by introducing static typing into Python's rich ecosystem of libraries and communities.

## Why Rust?

[Rust](https://www.rust-lang.org/) is a multi-paradigm programming language designed for performance and safety, especially safe
concurrency. Rust is "blazingly fast" and memory-efficient (comparable to C and C++) with no garbage collector.
It can power mission-critical systems, run on embedded devices, and easily integrates with other languages.

Rust’s rich type system and ownership model guarantees memory-safety and thread-safety deterministically —
eliminating many classes of bugs at compile-time.

The project increasingly utilizes Rust for core performance-critical components. Python bindings are implemented via Cython and [PyO3](https://pyo3.rs)—no Rust toolchain is required at install time.

This project makes the [Soundness Pledge](https://raphlinus.github.io/rust/2020/01/18/soundness-pledge.html):

> “The intent of this project is to be free of soundness bugs.
> The developers will do their best to avoid them, and welcome help in analyzing and fixing them.”

> [!NOTE]
>
> **MSRV:** NautilusTrader relies heavily on improvements in the Rust language and compiler.
> As a result, the Minimum Supported Rust Version (MSRV) is generally equal to the latest stable release of Rust.

## Integrations

NautilusTrader is modularly designed to work with *adapters*, enabling connectivity to trading venues
and data providers by translating their raw APIs into a unified interface and normalized domain model.

The following integrations are currently supported; see [docs/integrations/](https://nautilustrader.io/docs/latest/integrations/) for details:

| Name                                                                         | ID                    | Type                    | Status                                                  | Docs                                        |
| :--------------------------------------------------------------------------- | :-------------------- | :---------------------- | :------------------------------------------------------ | :------------------------------------------ |
| [Betfair](https://betfair.com)                                               | `BETFAIR`             | Sports Betting Exchange | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/betfair.md)       |
| [Binance](https://binance.com)                                               | `BINANCE`             | Crypto Exchange (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/binance.md)       |
| [Binance US](https://binance.us)                                             | `BINANCE`             | Crypto Exchange (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/binance.md)       |
| [Binance Futures](https://www.binance.com/en/futures)                        | `BINANCE`             | Crypto Exchange (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/binance.md)       |
| [Bybit](https://www.bybit.com)                                               | `BYBIT`               | Crypto Exchange (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/bybit.md)         |
| [Coinbase International](https://www.coinbase.com/en/international-exchange) | `COINBASE_INTX`       | Crypto Exchange (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/coinbase_intx.md) |
| [Databento](https://databento.com)                                           | `DATABENTO`           | Data Provider           | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/databento.md)     |
| [dYdX](https://dydx.exchange/)                                               | `DYDX`                | Crypto Exchange (DEX)   | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/dydx.md)          |
| [Interactive Brokers](https://www.interactivebrokers.com)                    | `INTERACTIVE_BROKERS` | Brokerage (multi-venue) | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/ib.md)            |
| [OKX](https://okx.com)                                                       | `OKX`                 | Crypto Exchange (CEX)   | ![status](https://img.shields.io/badge/building-orange) | [Guide](docs/integrations/okx.md)           |
| [Polymarket](https://polymarket.com)                                         | `POLYMARKET`          | Prediction Market (DEX) | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/polymarket.md)    |
| [Tardis](https://tardis.dev)                                                 | `TARDIS`              | Crypto Data Provider    | ![status](https://img.shields.io/badge/stable-green)    | [Guide](docs/integrations/tardis.md)        |

- **ID**: The default client ID for the integrations adapter clients.
- **Type**: The type of integration (often the venue type).

### Status

- `building`: Under construction and likely not in a usable state.
- `beta`: Completed to a minimally working state and in a beta testing phase.
- `stable`: Stabilized feature set and API, the integration has been tested by both developers and users to a reasonable level (some bugs may still remain).

See the [Integrations](https://nautilustrader.io/docs/latest/integrations/index.html) documentation for further details.

## Versioning and releases

**NautilusTrader is still under active development**. Some features may be incomplete, and while
the API is becoming more stable, breaking changes can occur between releases.
We strive to document these changes in the release notes on a **best-effort basis**.

We aim to follow a **bi-weekly release schedule**, though experimental or larger features may cause delays.

### Branches

We aim to maintain a stable, passing build across all branches.

- `master`: Reflects the source code for the latest released version; recommended for production use.
- `nightly`: Daily snapshots of the `develop` branch for early testing; merged at **14:00 UTC** or on demand.
- `develop`: Active development branch for contributors and feature work.

> [!NOTE]
>
> Our [roadmap](/ROADMAP.md) aims to achieve a **stable API for version 2.x** (likely after the Rust port).
> Once this milestone is reached, we plan to implement a formal deprecation process for any API changes.
> This approach allows us to maintain a rapid development pace for now.

## Precision mode

NautilusTrader supports two precision modes for its core value types (`Price`, `Quantity`, `Money`),
which differ in their internal bit-width and maximum decimal precision.

- **High-precision**: 128-bit integers with up to 16 decimals of precision, and a larger value range.
- **Standard-precision**: 64-bit integers with up to 9 decimals of precision, and a smaller value range.

> [!NOTE]
>
> By default, the official Python wheels **ship** in high-precision (128-bit) mode on Linux and macOS.
> On Windows, only standard-precision (64-bit) is available due to the lack of native 128-bit integer support.
> For the Rust crates, the default is standard-precision unless you explicitly enable the `high-precision` feature flag.

See the [Installation Guide](https://nautilustrader.io/docs/latest/getting_started/installation) for further details.

**Rust feature flag**: To enable high-precision mode in Rust, add the `high-precision` feature to your Cargo.toml:

```toml
[dependencies]
nautilus_model = { version = "*", features = ["high-precision"] }
```

## Installation

We recommend using the latest supported version of Python and installing [nautilus_trader](https://pypi.org/project/nautilus_trader/) inside a virtual environment to isolate dependencies.

**There are two supported ways to install**:

1. Pre-built binary wheel from PyPI *or* the Nautech Systems package index.
2. Build from source.

> [!TIP]
>
> We highly recommend installing using the [uv](https://docs.astral.sh/uv) package manager with a "vanilla" CPython.
>
> Conda and other Python distributions *may* work but aren’t officially supported.

### From PyPI

To install the latest binary wheel (or sdist package) from PyPI using Python's pip package manager:

```bash
pip install -U nautilus_trader
```

### From the Nautech Systems package index

The Nautech Systems package index (`packages.nautechsystems.io`) is [PEP-503](https://peps.python.org/pep-0503/) compliant and hosts both stable and development binary wheels for `nautilus_trader`.
This enables users to install either the latest stable release or pre-release versions for testing.

#### Stable wheels

Stable wheels correspond to official releases of `nautilus_trader` on PyPI, and use standard versioning.

To install the latest stable release:

```bash
pip install -U nautilus_trader --index-url=https://packages.nautechsystems.io/simple
```

#### Development wheels

Development wheels are published from both the `nightly` and `develop` branches,
allowing users to test features and fixes ahead of stable releases.

**Note**: Wheels from the `develop` branch are only built for the Linux x86_64 platform to save time
and compute resources, while `nightly` wheels support additional platforms as shown below.

| Platform           | Nightly | Develop |
| :----------------- | :------ | :------ |
| `Linux (x86_64)`   | ✓       | ✓       |
| `Linux (ARM64)`    | ✓       | -       |
| `macOS (ARM64)`    | ✓       | -       |
| `Windows (x86_64)` | ✓       | -       |

This process also helps preserve compute resources and ensures easy access to the exact binaries tested in CI pipelines,
while adhering to [PEP-440](https://peps.python.org/pep-0440/) versioning standards:

- `develop` wheels use the version format `dev{date}+{build_number}` (e.g., `1.208.0.dev20241212+7001`).
- `nightly` wheels use the version format `a{date}` (alpha) (e.g., `1.208.0a20241212`).

> [!WARNING]
>
> We don't recommend using development wheels in production environments, such as live trading controlling real capital.

#### Installation commands

By default, pip installs the latest stable release. Adding the `--pre` flag ensures that pre-release versions, including development wheels, are considered.

To install the latest available pre-release (including development wheels):

```bash
pip install -U nautilus_trader --pre --index-url=https://packages.nautechsystems.io/simple
```

To install a specific development wheel (e.g., `1.208.0a20241212` for December 12, 2024):

```bash
pip install nautilus_trader==1.208.0a20241212 --index-url=https://packages.nautechsystems.io/simple
```

#### Available versions

You can view all available versions of `nautilus_trader` on the [package index](https://packages.nautechsystems.io/simple/nautilus-trader/index.html).

To programmatically fetch and list available versions:

```bash
curl -s https://packages.nautechsystems.io/simple/nautilus-trader/index.html | grep -oP '(?<=<a href=")[^"]+(?=")' | awk -F'#' '{print $1}' | sort
```

#### Branch updates

- `develop` branch wheels (`.dev`): Are built and published continuously with every merged commit.
- `nightly` branch wheels (`a`): Are built and published daily when `develop` branch is automatically merged at **14:00 UTC** (if there are changes).

#### Retention policies

- `develop` branch wheels (`.dev`): Only the most recent wheel build is retained.
- `nightly` branch wheels (`a`): Only the 10 most recent wheel builds are retained.

### From Source

It's possible to install from source using pip if you first install the build dependencies as specified in the `pyproject.toml`.

1. Install [rustup](https://rustup.rs/) (the Rust toolchain installer):
   - Linux and macOS:

       ```bash
       curl https://sh.rustup.rs -sSf | sh
       ```

   - Windows:
       - Download and install [`rustup-init.exe`](https://win.rustup.rs/x86_64)
       - Install "Desktop development with C++" with [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
   - Verify (any system):
       from a terminal session run: `rustc --version`

2. Enable `cargo` in the current shell:
   - Linux and macOS:

       ```bash
       source $HOME/.cargo/env
       ```

   - Windows:
     - Start a new PowerShell

3. Install [clang](https://clang.llvm.org/) (a C language frontend for LLVM):
   - Linux:

       ```bash
       sudo apt-get install clang
       ```

   - Windows:
       1. Add Clang to your [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16):
          - Start | Visual Studio Installer | Modify | C++ Clang tools for Windows (12.0.0 - x64…) = checked | Modify
       2. Enable `clang` in the current shell:

          ```powershell
          [System.Environment]::SetEnvironmentVariable('path', "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\Llvm\x64\bin\;" + $env:Path,"User")
          ```

   - Verify (any system):
       from a terminal session run: `clang --version`

4. Install uv (see the [uv installation guide](https://docs.astral.sh/uv/getting-started/installation) for more details):

    ```bash
    curl -LsSf https://astral.sh/uv/install.sh | sh
    ```

5. Clone the source with `git`, and install from the project's root directory:

    ```bash
    git clone --branch develop --depth 1 https://github.com/nautechsystems/nautilus_trader
    cd nautilus_trader
    uv sync --all-extras
    ```

> [!NOTE]
>
> The `--depth 1` flag fetches just the latest commit for a faster, lightweight clone.

See the [Installation Guide](https://nautilustrader.io/docs/latest/getting_started/installation) for other options and further details.

## Redis

Using [Redis](https://redis.io) with NautilusTrader is **optional** and only required if configured as the backend for a
[cache](https://nautilustrader.io/docs/latest/concepts/cache) database or [message bus](https://nautilustrader.io/docs/latest/concepts/message_bus).
See the **Redis** section of the [Installation Guide](https://nautilustrader.io/docs/latest/getting_started/installation#redis) for further details.

## Makefile

A `Makefile` is provided to automate most installation and build tasks for development. It provides the following targets:

- `make install`: Installs in `release` build mode with all dependency groups and extras.
- `make install-debug`: Same as `make install` but with `debug` build mode.
- `make install-just-deps`: Installs just the `main`, `dev` and `test` dependencies (does not install package).
- `make build`: Runs the build script in `release` build mode (default).
- `make build-debug`: Runs the build script in `debug` build mode.
- `make build-wheel`: Runs uv build with a wheel format in `release` mode.
- `make build-wheel-debug`: Runs uv build with a wheel format in `debug` mode.
- `make clean`: Deletes all build results, such as `.so` or `.dll` files.
- `make distclean`: **CAUTION** Removes all artifacts not in the git index from the repository. This includes source files which have not been `git add`ed.
- `make docs`: Builds the documentation HTML using Sphinx.
- `make pre-commit`: Runs the pre-commit checks over all files.
- `make ruff`: Runs ruff over all files using the `pyproject.toml` config (with autofix).
- `make pytest`: Runs all tests with `pytest`.
- `make test-performance`: Runs performance tests with [codspeed](https://codspeed.io).

> [!TIP]
>
> Run `make build-debug` to compile after changes to Rust or Cython code for the most efficient development workflow.

## Examples

Indicators and strategies can be developed in both Python and Cython. For performance and
latency-sensitive applications, we recommend using Cython. Below are some examples:

- [indicator](/nautilus_trader/examples/indicators/ema_python.py) example written in Python.
- [indicator](/nautilus_trader/indicators/) examples written in Cython.
- [strategy](/nautilus_trader/examples/strategies/) examples written in Python.
- [backtest](/examples/backtest/) examples using a `BacktestEngine` directly.

## Docker

Docker containers are built using the base image `python:3.12-slim` with the following variant tags:

- `nautilus_trader:latest` has the latest release version installed.
- `nautilus_trader:nightly` has the head of the `nightly` branch installed.
- `jupyterlab:latest` has the latest release version installed along with `jupyterlab` and an
  example backtest notebook with accompanying data.
- `jupyterlab:nightly` has the head of the `nightly` branch installed along with `jupyterlab` and an
  example backtest notebook with accompanying data.

You can pull the container images as follows:

```bash
docker pull ghcr.io/nautechsystems/<image_variant_tag> --platform linux/amd64
```

You can launch the backtest example container by running:

```bash
docker pull ghcr.io/nautechsystems/jupyterlab:nightly --platform linux/amd64
docker run -p 8888:8888 ghcr.io/nautechsystems/jupyterlab:nightly
```

Then open your browser at the following address:

```bash
http://127.0.0.1:8888/lab
```

> [!WARNING]
>
> NautilusTrader currently exceeds the rate limit for Jupyter notebook logging (stdout output).
> As a result, the `log_level` in the examples is set to `ERROR`. Lowering this level to see more
> logging will cause the notebook to hang during cell execution. We are investigating a fix, which
> may involve either raising the configured rate limits for Jupyter or throttling the log flushing
> from Nautilus.
>
> - <https://github.com/jupyterlab/jupyterlab/issues/12845>
> - <https://github.com/deshaw/jupyterlab-limit-output>

## Development

We aim to provide the most pleasant developer experience possible for this hybrid codebase of Python, Cython and Rust.
See the [Developer Guide](https://nautilustrader.io/docs/latest/developer_guide/index.html) for helpful information.

### Testing with Rust

[cargo-nextest](https://nexte.st) is the standard Rust test runner for NautilusTrader.
Its key benefit is isolating each test in its own process, ensuring test reliability
by avoiding interference.

You can install cargo-nextest by running:

```bash
cargo install cargo-nextest
```

> [!TIP]
>
> Run Rust tests with `make cargo-test`, which uses **cargo-nextest** with an efficient profile.

## Contributing

Thank you for considering contributing to NautilusTrader! We welcome any and all help to improve
the project. If you have an idea for an enhancement or a bug fix, the first step is to open an [issue](https://github.com/nautechsystems/nautilus_trader/issues)
on GitHub to discuss it with the team. This helps to ensure that your contribution will be
well-aligned with the goals of the project and avoids duplication of effort.

Once you're ready to start working on your contribution, make sure to follow the guidelines
outlined in the [CONTRIBUTING.md](https://github.com/nautechsystems/nautilus_trader/blob/develop/CONTRIBUTING.md) file. This includes signing a Contributor License Agreement (CLA)
to ensure that your contributions can be included in the project.

> [!NOTE]
>
> Pull requests should target the `develop` branch (the default branch). This is where new features and improvements are integrated before release.

Thank you again for your interest in NautilusTrader! We look forward to reviewing your contributions and working with you to improve the project.

## Community

Join our community of users and contributors on [Discord](https://discord.gg/NautilusTrader) to chat
and stay up-to-date with the latest announcements and features of NautilusTrader. Whether you're a
developer looking to contribute or just want to learn more about the platform, all are welcome on our Discord server.

> [!WARNING]
>
> NautilusTrader does not issue, promote, or endorse any cryptocurrency tokens. Any claims or communications suggesting otherwise are unauthorized and false.
>
> All official updates and communications from NautilusTrader will be shared exclusively through <https://nautilustrader.io>, our [Discord server](https://discord.gg/NautilusTrader),
> or our X (Twitter) account: [@NautilusTrader](https://x.com/NautilusTrader).
>
> If you encounter any suspicious activity, please report it to the appropriate platform and contact us at <info@nautechsystems.io>.

## License

The source code for NautilusTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
Contributions to the project are welcome and require the completion of a standard [Contributor License Agreement (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md).

---

NautilusTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nautilustrader.io>.

© 2015-2025 Nautech Systems Pty Ltd. All rights reserved.

![nautechsystems](https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/ns-logo.png "nautechsystems")
<img src="https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/ferris.png" width="128">
