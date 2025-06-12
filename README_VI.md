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

> *nautilus - từ tiếng Hy Lạp cổ 'thủy thủ' và naus 'thuyền'.*
>
> *Vỏ nautilus bao gồm các khoang mô-đun với hệ số tăng trưởng gần với đường xoắn ốc logarit.
> Ý tưởng là điều này có thể được chuyển dịch thành tính thẩm mỹ của thiết kế và kiến trúc.*

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

## Tại sao Python?

Python được tạo ra từ nhiều thập kỷ trước như một ngôn ngữ scripting đơn giản với cú pháp rõ ràng và dễ hiểu.
Từ đó nó đã phát triển thành một ngôn ngữ lập trình hướng đối tượng đa năng hoàn chỉnh.
Dựa trên chỉ số TIOBE, Python hiện là ngôn ngữ lập trình phổ biến nhất thế giới.
Không chỉ vậy, Python đã trở thành *ngôn ngữ chung thực tế* của khoa học dữ liệu, học máy và trí tuệ nhân tạo.

Cộng đồng nhà phát triển/người dùng rất phong phú và đa dạng.
Tuy nhiên, Python có những hạn chế về hiệu năng và kiểu dữ liệu cho các hệ thống quy mô lớn, nhạy cảm với độ trễ. Cython giải quyết nhiều vấn đề này bằng cách đưa kiểu tĩnh vào hệ sinh thái thư viện và cộng đồng phong phú của Python.

## Tại sao Rust?

[Rust](https://www.rust-lang.org/) là một ngôn ngữ lập trình đa mô hình được thiết kế cho hiệu năng và an toàn, đặc biệt là
đồng thời an toàn. Rust "cực kỳ nhanh" và tiết kiệm bộ nhớ (so sánh với C và C++) mà không có garbage collector.
Nó có thể hỗ trợ các hệ thống có tính chất quan trọng, chạy trên thiết bị nhúng, và dễ dàng tích hợp với các ngôn ngữ khác.

Hệ thống kiểu phong phú và mô hình ownership của Rust đảm bảo an toàn bộ nhớ và an toàn luồng một cách xác định —
loại bỏ nhiều lớp bug tại thời điểm biên dịch.

Dự án ngày càng sử dụng Rust cho các component core quan trọng về hiệu năng. Python bindings được triển khai qua Cython và [PyO3](https://pyo3.rs)—không cần Rust toolchain tại thời điểm cài đặt.

Dự án này cam kết [Soundness Pledge](https://raphlinus.github.io/rust/2020/01/18/soundness-pledge.html):

> "Ý định của dự án này là không có soundness bugs.
> Các nhà phát triển sẽ cố gắng hết sức để tránh chúng, và hoan nghênh sự giúp đỡ trong việc phân tích và sửa chữa chúng."

> [!NOTE]
>
> **MSRV:** NautilusTrader phụ thuộc nhiều vào cải tiến trong ngôn ngữ và trình biên dịch Rust.
> Do đó, Minimum Supported Rust Version (MSRV) thường bằng với phiên bản ổn định mới nhất của Rust.

## Tích hợp

NautilusTrader được thiết kế mô-đun để làm việc với *adapters*, cho phép kết nối tới các sàn giao dịch
và nhà cung cấp dữ liệu bằng cách dịch các API thô của họ thành giao diện thống nhất và mô hình domain chuẩn hóa.

Các tích hợp sau hiện được hỗ trợ; xem [docs/integrations/](https://nautilustrader.io/docs/latest/integrations/) để biết chi tiết:

| Tên                                                                          | ID                    | Loại                   | Trạng thái                                              | Tài liệu                                    |
| :--------------------------------------------------------------------------- | :-------------------- | :---------------------- | :------------------------------------------------------ | :------------------------------------------ |
| [Betfair](https://betfair.com)                                               | `BETFAIR`             | Sàn cá cược thể thao   | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/betfair.md)       |
| [Binance](https://binance.com)                                               | `BINANCE`             | Sàn Crypto (CEX)       | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/binance.md)       |
| [Binance US](https://binance.us)                                             | `BINANCE`             | Sàn Crypto (CEX)       | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/binance.md)       |
| [Binance Futures](https://www.binance.com/en/futures)                        | `BINANCE`             | Sàn Crypto (CEX)       | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/binance.md)       |
| [Bybit](https://www.bybit.com)                                               | `BYBIT`               | Sàn Crypto (CEX)       | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/bybit.md)         |
| [Coinbase International](https://www.coinbase.com/en/international-exchange) | `COINBASE_INTX`       | Sàn Crypto (CEX)       | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/coinbase_intx.md) |
| [Databento](https://databento.com)                                           | `DATABENTO`           | Nhà cung cấp dữ liệu   | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/databento.md)     |
| [dYdX](https://dydx.exchange/)                                               | `DYDX`                | Sàn Crypto (DEX)       | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/dydx.md)          |
| [Interactive Brokers](https://www.interactivebrokers.com)                    | `INTERACTIVE_BROKERS` | Môi giới (đa sàn)      | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/ib.md)            |
| [OKX](https://okx.com)                                                       | `OKX`                 | Sàn Crypto (CEX)       | ![status](https://img.shields.io/badge/building-orange) | [Hướng dẫn](docs/integrations/okx.md)           |
| [Polymarket](https://polymarket.com)                                         | `POLYMARKET`          | Thị trường dự đoán (DEX) | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/polymarket.md)    |
| [Tardis](https://tardis.dev)                                                 | `TARDIS`              | Nhà cung cấp dữ liệu Crypto | ![status](https://img.shields.io/badge/stable-green)    | [Hướng dẫn](docs/integrations/tardis.md)        |

- **ID**: Client ID mặc định cho các adapter client tích hợp.
- **Loại**: Loại tích hợp (thường là loại sàn).

### Trạng thái

- `building`: Đang xây dựng và có thể chưa sử dụng được.
- `beta`: Hoàn thành ở trạng thái hoạt động tối thiểu và đang trong giai đoạn beta testing.
- `stable`: Tập tính năng và API đã ổn định, tích hợp đã được test bởi cả nhà phát triển và người dùng ở mức hợp lý (vẫn có thể còn một số bug).

Xem tài liệu [Integrations](https://nautilustrader.io/docs/latest/integrations/index.html) để biết thêm chi tiết.

## Versioning và releases

**NautilusTrader vẫn đang trong giai đoạn phát triển tích cực**. Một số tính năng có thể chưa hoàn thiện, và mặc dù
API đang trở nên ổn định hơn, các thay đổi phá vỡ có thể xảy ra giữa các phiên bản.
Chúng tôi cố gắng ghi chép những thay đổi này trong release notes trên cơ sở **cố gắng hết sức**.

Chúng tôi nhắm tới lịch trình phát hành **hai tuần một lần**, mặc dù các tính năng thử nghiệm hoặc lớn hơn có thể gây ra độ trễ.

### Nhánh

Chúng tôi nhắm duy trì build ổn định, vượt qua kiểm tra trên tất cả các nhánh.

- `master`: Phản ánh mã nguồn cho phiên bản phát hành mới nhất; được khuyến nghị cho việc sử dụng sản xuất.
- `nightly`: Snapshot hàng ngày của nhánh `develop` để kiểm tra sớm; được merge lúc **14:00 UTC** hoặc theo yêu cầu.
- `develop`: Nhánh phát triển tích cực cho contributors và feature work.

> [!NOTE]
>
> [Roadmap](/ROADMAP.md) của chúng tôi nhắm đạt được **API ổn định cho phiên bản 2.x** (có thể sau khi port Rust).
> Khi cột mốc này đạt được, chúng tôi có kế hoạch triển khai quy trình deprecation chính thức cho bất kỳ thay đổi API nào.
> Cách tiếp cận này cho phép chúng tôi duy trì tốc độ phát triển nhanh hiện tại.

## Chế độ precision

NautilusTrader hỗ trợ hai chế độ precision cho các core value types (`Price`, `Quantity`, `Money`),
khác nhau về bit-width nội bộ và precision thập phân tối đa.

- **High-precision**: Số nguyên 128-bit với tới 16 chữ số thập phân precision, và phạm vi giá trị lớn hơn.
- **Standard-precision**: Số nguyên 64-bit với tới 9 chữ số thập phân precision, và phạm vi giá trị nhỏ hơn.

> [!NOTE]
>
> Mặc định, các Python wheels chính thức **ship** ở chế độ high-precision (128-bit) trên Linux và macOS.
> Trên Windows, chỉ có standard-precision (64-bit) do thiếu hỗ trợ số nguyên 128-bit gốc.
> Đối với Rust crates, mặc định là standard-precision trừ khi bạn explicitly enable `high-precision` feature flag.

Xem [Installation Guide](https://nautilustrader.io/docs/latest/getting_started/installation) để biết thêm chi tiết.

**Rust feature flag**: Để enable chế độ high-precision trong Rust, thêm feature `high-precision` vào Cargo.toml:

```toml
[dependencies]
nautilus_model = { version = "*", features = ["high-precision"] }
```

## Cài đặt

Chúng tôi khuyến nghị sử dụng phiên bản Python mới nhất được hỗ trợ và cài đặt [nautilus_trader](https://pypi.org/project/nautilus_trader/) trong virtual environment để cô lập dependencies.

**Có hai cách cài đặt được hỗ trợ**:

1. Pre-built binary wheel từ PyPI *hoặc* Nautech Systems package index.
2. Build từ source.

> [!TIP]
>
> Chúng tôi khuyến nghị sử dụng package manager [uv](https://docs.astral.sh/uv) với CPython "vanilla".
>
> Conda và các bản phân phối Python khác *có thể* hoạt động nhưng không được hỗ trợ chính thức.

### Từ PyPI

Để cài đặt binary wheel mới nhất (hoặc sdist package) từ PyPI sử dụng pip package manager của Python:

```bash
pip install -U nautilus_trader
```

### Từ Nautech Systems package index

Nautech Systems package index (`packages.nautechsystems.io`) tuân thủ [PEP-503](https://peps.python.org/pep-0503/) và host cả stable và development binary wheels cho `nautilus_trader`.
Điều này cho phép người dùng cài đặt phiên bản stable mới nhất hoặc phiên bản pre-release để testing.

#### Stable wheels

Stable wheels tương ứng với các phiên bản chính thức của `nautilus_trader` trên PyPI, và sử dụng versioning tiêu chuẩn.

Để cài đặt phiên bản stable mới nhất:

```bash
pip install -U nautilus_trader --index-url=https://packages.nautechsystems.io/simple
```

#### Development wheels

Development wheels được publish từ cả nhánh `nightly` và `develop`,
cho phép người dùng test các tính năng và fixes trước các phiên bản stable.

**Chú ý**: Wheels từ nhánh `develop` chỉ được build cho nền tảng Linux x86_64 để tiết kiệm thời gian
và tài nguyên tính toán, trong khi `nightly` wheels hỗ trợ thêm các nền tảng như shown bên dưới.

| Nền tảng           | Nightly | Develop |
| :----------------- | :------ | :------ |
| `Linux (x86_64)`   | ✓       | ✓       |
| `Linux (ARM64)`    | ✓       | -       |
| `macOS (ARM64)`    | ✓       | -       |
| `Windows (x86_64)` | ✓       | -       |

Quy trình này cũng giúp bảo tồn tài nguyên tính toán và đảm bảo truy cập dễ dàng tới các binary chính xác được test trong CI pipelines,
tuân thủ tiêu chuẩn versioning [PEP-440](https://peps.python.org/pep-0440/):

- `develop` wheels sử dụng format version `dev{date}+{build_number}` (e.g., `1.208.0.dev20241212+7001`).
- `nightly` wheels sử dụng format version `a{date}` (alpha) (e.g., `1.208.0a20241212`).

> [!WARNING]
>
> Chúng tôi không khuyến nghị sử dụng development wheels trong môi trường sản xuất, như giao dịch thực kiểm soát vốn thật.

#### Lệnh cài đặt

Mặc định, pip cài đặt phiên bản stable mới nhất. Thêm flag `--pre` đảm bảo rằng các phiên bản pre-release, bao gồm development wheels, được xem xét.

Để cài đặt pre-release mới nhất có sẵn (bao gồm development wheels):

```bash
pip install -U nautilus_trader --pre --index-url=https://packages.nautechsystems.io/simple
```

Để cài đặt development wheel cụ thể (e.g., `1.208.0a20241212` cho 12 tháng 12, 2024):

```bash
pip install nautilus_trader==1.208.0a20241212 --index-url=https://packages.nautechsystems.io/simple
```

#### Phiên bản có sẵn

Bạn có thể xem tất cả phiên bản có sẵn của `nautilus_trader` trên [package index](https://packages.nautechsystems.io/simple/nautilus-trader/index.html).

Để fetch và list các phiên bản có sẵn bằng code:

```bash
curl -s https://packages.nautechsystems.io/simple/nautilus-trader/index.html | grep -oP '(?<=<a href=")[^"]+(?=")' | awk -F'#' '{print $1}' | sort
```

#### Cập nhật nhánh

- `develop` branch wheels (`.dev`): Được build và publish liên tục với mỗi commit được merge.
- `nightly` branch wheels (`a`): Được build và publish hàng ngày khi nhánh `develop` tự động merge lúc **14:00 UTC** (nếu có thay đổi).

#### Chính sách lưu trữ

- `develop` branch wheels (`.dev`): Chỉ wheel build gần nhất được giữ lại.
- `nightly` branch wheels (`a`): Chỉ 10 wheel builds gần nhất được giữ lại.

### Từ Source

Có thể cài đặt từ source sử dụng pip nếu bạn cài đặt build dependencies trước như được chỉ định trong `pyproject.toml`.

1. Cài đặt [rustup](https://rustup.rs/) (Rust toolchain installer):
   - Linux và macOS:

       ```bash
       curl https://sh.rustup.rs -sSf | sh
       ```

   - Windows:
       - Tải và cài đặt [`rustup-init.exe`](https://win.rustup.rs/x86_64)
       - Cài đặt "Desktop development with C++" với [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
   - Xác minh (bất kỳ hệ thống nào):
       từ terminal session chạy: `rustc --version`

2. Enable `cargo` trong shell hiện tại:
   - Linux và macOS:

       ```bash
       source $HOME/.cargo/env
       ```

   - Windows:
     - Khởi động PowerShell mới

3. Cài đặt [clang](https://clang.llvm.org/) (C language frontend cho LLVM):
   - Linux:

       ```bash
       sudo apt-get install clang
       ```

   - Windows:
       1. Thêm Clang vào [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16):
          - Start | Visual Studio Installer | Modify | C++ Clang tools for Windows (12.0.0 - x64…) = checked | Modify
       2. Enable `clang` trong shell hiện tại:

          ```powershell
          [System.Environment]::SetEnvironmentVariable('path', "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\Llvm\x64\bin\;" + $env:Path,"User")
          ```

   - Xác minh (bất kỳ hệ thống nào):
       từ terminal session chạy: `clang --version`

4. Cài đặt uv (xem [uv installation guide](https://docs.astral.sh/uv/getting-started/installation) để biết thêm chi tiết):

    ```bash
    curl -LsSf https://astral.sh/uv/install.sh | sh
    ```

5. Clone source với `git`, và cài đặt từ thư mục root của dự án:

    ```bash
    git clone --branch develop --depth 1 https://github.com/nautechsystems/nautilus_trader
    cd nautilus_trader
    uv sync --all-extras
    ```

> [!NOTE]
>
> Flag `--depth 1` chỉ fetch commit mới nhất để clone nhanh và nhẹ hơn.

Xem [Installation Guide](https://nautilustrader.io/docs/latest/getting_started/installation) cho các tùy chọn khác và chi tiết thêm.

## Redis

Sử dụng [Redis](https://redis.io) với NautilusTrader là **tùy chọn** và chỉ cần thiết nếu được cấu hình như backend cho
[cache](https://nautilustrader.io/docs/latest/concepts/cache) database hoặc [message bus](https://nautilustrader.io/docs/latest/concepts/message_bus).
Xem phần **Redis** của [Installation Guide](https://nautilustrader.io/docs/latest/getting_started/installation#redis) để biết thêm chi tiết.

## Makefile

Một `Makefile` được cung cấp để tự động hóa hầu hết các tác vụ cài đặt và build cho development. Nó cung cấp các target sau:

- `make install`: Cài đặt ở chế độ build `release` với tất cả dependency groups và extras.
- `make install-debug`: Giống như `make install` nhưng với chế độ build `debug`.
- `make install-just-deps`: Chỉ cài đặt dependencies `main`, `dev` và `test` (không cài đặt package).
- `make build`: Chạy build script ở chế độ build `release` (mặc định).
- `make build-debug`: Chạy build script ở chế độ build `debug`.
- `make build-wheel`: Chạy uv build với wheel format ở chế độ `release`.
- `make build-wheel-debug`: Chạy uv build với wheel format ở chế độ `debug`.
- `make clean`: Xóa tất cả kết quả build, như files `.so` hoặc `.dll`.
- `make distclean`: **THẬN TRỌNG** Loại bỏ tất cả artifacts không có trong git index từ repository. Bao gồm source files chưa được `git add`.
- `make docs`: Build documentation HTML sử dụng Sphinx.
- `make pre-commit`: Chạy pre-commit checks trên tất cả files.
- `make ruff`: Chạy ruff trên tất cả files sử dụng config `pyproject.toml` (với autofix).
- `make pytest`: Chạy tất cả tests với `pytest`.
- `make test-performance`: Chạy performance tests với [codspeed](https://codspeed.io).

> [!TIP]
>
> Chạy `make build-debug` để compile sau các thay đổi tới Rust hoặc Cython code cho workflow development hiệu quả nhất.

## Ví dụ

Indicators và strategies có thể được phát triển bằng cả Python và Cython. Cho các ứng dụng hiệu năng và
nhạy cảm độ trễ, chúng tôi khuyến nghị sử dụng Cython. Dưới đây là một số ví dụ:

- Ví dụ [indicator](/nautilus_trader/examples/indicators/ema_python.py) viết bằng Python.
- Ví dụ [indicator](/nautilus_trader/indicators/) viết bằng Cython.
- Ví dụ [strategy](/nautilus_trader/examples/strategies/) viết bằng Python.
- Ví dụ [backtest](/examples/backtest/) sử dụng `BacktestEngine` trực tiếp.

## Docker

Docker containers được build sử dụng base image `python:3.12-slim` với các variant tags sau:

- `nautilus_trader:latest` có phiên bản release mới nhất được cài đặt.
- `nautilus_trader:nightly` có head của nhánh `nightly` được cài đặt.
- `jupyterlab:latest` có phiên bản release mới nhất được cài đặt cùng với `jupyterlab` và một
  example backtest notebook với dữ liệu đi kèm.
- `jupyterlab:nightly` có head của nhánh `nightly` được cài đặt cùng với `jupyterlab` và một
  example backtest notebook với dữ liệu đi kèm.

Bạn có thể pull container images như sau:

```bash
docker pull ghcr.io/nautechsystems/<image_variant_tag> --platform linux/amd64
```

Bạn có thể launch backtest example container bằng cách chạy:

```bash
docker pull ghcr.io/nautechsystems/jupyterlab:nightly --platform linux/amd64
docker run -p 8888:8888 ghcr.io/nautechsystems/jupyterlab:nightly
```

Sau đó mở browser tại địa chỉ sau:

```bash
http://127.0.0.1:8888/lab
```

> [!WARNING]
>
> NautilusTrader hiện vượt quá rate limit cho Jupyter notebook logging (stdout output).
> Do đó, `log_level` trong examples được đặt thành `ERROR`. Hạ thấp level này để xem thêm
> logging sẽ khiến notebook bị treo trong quá trình thực thi cell. Chúng tôi đang điều tra một fix, có thể
> bao gồm việc tăng rate limits được cấu hình cho Jupyter hoặc throttling log flushing
> từ Nautilus.
>
> - <https://github.com/jupyterlab/jupyterlab/issues/12845>
> - <https://github.com/deshaw/jupyterlab-limit-output>

## Phát triển

Chúng tôi nhắm cung cấp trải nghiệm developer thoải mái nhất có thể cho codebase hybrid này của Python, Cython và Rust.
Xem [Developer Guide](https://nautilustrader.io/docs/latest/developer_guide/index.html) để có thông tin hữu ích.

### Testing với Rust

[cargo-nextest](https://nexte.st) là Rust test runner tiêu chuẩn cho NautilusTrader.
Lợi ích chính của nó là cô lập mỗi test trong process riêng, đảm bảo độ tin cậy của test
bằng cách tránh interference.

Bạn có thể cài đặt cargo-nextest bằng cách chạy:

```bash
cargo install cargo-nextest
```

> [!TIP]
>
> Chạy Rust tests với `make cargo-test`, sử dụng **cargo-nextest** với profile hiệu quả.

## Đóng góp

Cảm ơn bạn đã xem xét đóng góp cho NautilusTrader! Chúng tôi hoan nghênh mọi sự giúp đỡ để cải thiện
dự án. Nếu bạn có ý tưởng cho enhancement hoặc bug fix, bước đầu tiên là mở một [issue](https://github.com/nautechsystems/nautilus_trader/issues)
trên GitHub để thảo luận với team. Điều này giúp đảm bảo rằng đóng góp của bạn sẽ
phù hợp với mục tiêu của dự án và tránh trùng lặp công sức.

Khi bạn sẵn sàng bắt đầu làm việc trên đóng góp của mình, hãy đảm bảo tuân theo hướng dẫn
được nêu trong file [CONTRIBUTING.md](https://github.com/nautechsystems/nautilus_trader/blob/develop/CONTRIBUTING.md). Bao gồm việc ký Contributor License Agreement (CLA)
để đảm bảo rằng đóng góp của bạn có thể được bao gồm trong dự án.

> [!NOTE]
>
> Pull requests nên target nhánh `develop` (nhánh mặc định). Đây là nơi các tính năng và cải tiến mới được tích hợp trước khi release.

Cảm ơn một lần nữa vì sự quan tâm đến NautilusTrader! Chúng tôi mong được xem xét các đóng góp của bạn và làm việc cùng bạn để cải thiện dự án.

## Cộng đồng

Tham gia cộng đồng người dùng và contributors trên [Discord](https://discord.gg/NautilusTrader) để chat
và cập nhật những thông báo và tính năng mới nhất của NautilusTrader. Dù bạn là
developer muốn đóng góp hay chỉ muốn tìm hiểu thêm về nền tảng, tất cả đều được chào đón trên Discord server của chúng tôi.

> [!WARNING]
>
> NautilusTrader không phát hành, quảng bá hoặc ủng hộ bất kỳ cryptocurrency tokens nào. Bất kỳ tuyên bố hoặc giao tiếp nào gợi ý ngược lại đều không được ủy quyền và sai.
>
> Tất cả updates và giao tiếp chính thức từ NautilusTrader sẽ được chia sẻ độc quyền qua <https://nautilustrader.io>, [Discord server](https://discord.gg/NautilusTrader) của chúng tôi,
> hoặc tài khoản X (Twitter) của chúng tôi: [@NautilusTrader](https://x.com/NautilusTrader).
>
> Nếu bạn gặp phải hoạt động đáng ngờ nào, vui lòng báo cáo cho nền tảng thích hợp và liên hệ chúng tôi tại <info@nautechsystems.io>.

## Giấy phép

Mã nguồn của NautilusTrader có sẵn trên GitHub theo [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
Đóng góp cho dự án được hoan nghênh và yêu cầu hoàn thành [Contributor License Agreement (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md) tiêu chuẩn.

---

NautilusTrader™ được phát triển và duy trì bởi Nautech Systems, một công ty công nghệ
chuyên về phát triển hệ thống giao dịch hiệu năng cao.
Để biết thêm thông tin, hãy truy cập <https://nautilustrader.io>.

© 2015-2025 Nautech Systems Pty Ltd. Tất cả quyền được bảo lưu.

![nautechsystems](https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/ns-logo.png "nautechsystems")
<img src="https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/ferris.png" width="128">
