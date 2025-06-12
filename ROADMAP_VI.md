# Lộ trình

Tài liệu này phác thảo các ưu tiên chính và mục tiêu sắp tới cho **NautilusTrader**,
vạch ra con đường của nó như một nền tảng tiên tiến cho giao dịch thuật toán hiệu năng cao.

Do tính chất năng động của dự án, các ưu tiên có thể phát triển để theo kịp chu kỳ phát triển thay đổi nhanh.
Để có cập nhật theo thời gian thực và theo dõi tác vụ chi tiết, hãy tham khảo [NautilusTrader Kanban board](https://github.com/orgs/nautechsystems/projects/3).

**Lưu ý**: Sửa lỗi và ưu tiên lộ trình được ưu tiên hơn yêu cầu tính năng để đảm bảo tính ổn định
và tiến độ. Tuy nhiên, pull requests (PRs) cho cải tiến và tính năng mới luôn được chào đón.
Để biết thêm chi tiết, xem [CONTRIBUTING.md](/CONTRIBUTING.md).

## Tầm nhìn

Thiết lập NautilusTrader như nền tảng tiêu chuẩn cho giao dịch thuật toán định lượng, kết hợp
hiệu năng, độ tin cậy, khả năng sử dụng và tài liệu toàn diện cho các nhà giao dịch và nhà phát triển.

## Ưu tiên

1. **Port core sang Rust**

   **Mục tiêu**: Tận dụng các tính năng hiệu năng và an toàn của Rust để cải thiện độ tin cậy, hiệu năng và khả năng mở rộng.
   - Viết lại các component quan trọng về hiệu năng bằng Rust (thay thế các module Cython hiện có).
   - Đảm bảo khả năng tương tác giữa các lớp Rust và Python sử dụng PyO3.
   - Benchmark cải tiến hiệu năng trong suốt quá trình chuyển đổi.

2. **Cải thiện Tài liệu và Hướng dẫn**

    **Mục tiêu**: Giảm đường cong học tập cho người dùng mới và trao quyền cho các nhà phát triển với các hướng dẫn rõ ràng, toàn diện:
   - Điền vào khoảng trống trong tài liệu người dùng và nhà phát triển bằng cách thêm các phần còn thiếu.
   - Thêm hướng dẫn và ví dụ bổ sung.

3. **Cải thiện Code Ergonomics**

    **Mục tiêu**: Đơn giản hóa trải nghiệm phát triển cho người dùng và người đóng góp:
   - Tăng cường type annotations và hỗ trợ cho Python import resolution.
   - Chuẩn hóa naming conventions và tinh chỉnh APIs để tăng tính trực quan.
   - Hợp lý hóa quy trình cấu hình và thiết lập để giảm thiểu ma sát.
   - Refactor modules và namespaces để cải thiện khả năng đọc và bảo trì.

## Cải tiến Bổ sung

Khi chúng tôi tiến triển trên các ưu tiên hàng đầu, chúng tôi cũng có kế hoạch tập trung vào các cải tiến sau:

- Mở rộng tích hợp với adapters để hỗ trợ các sàn giao dịch và nhà cung cấp dữ liệu.
- Tăng cường backtesting engine với các tính năng bổ sung.
- Tăng cường order book execution dynamics với các tính năng bổ sung, bao gồm tương tác lệnh người dùng, thay đổi book bền vững, và mở rộng mô phỏng microstructure.

## NautilusTrader v2.0 và Tương lai

- **Đạt được Trạng thái Ổn định**: Mặc dù NautilusTrader đã được sử dụng thành công trong sản xuất, v2.0 đại diện cho một cột mốc quan trọng hướng tới việc thiết lập API ổn định.
- **Lĩnh vực Tập trung**: Sáng kiến v2.0 sẽ ưu tiên tính nhất quán API, khả năng bảo trì dài hạn, và đáp ứng các tiêu chuẩn khắt khe của môi trường giao dịch thực.
- **Deprecations Chính thức**: v2.0 sẽ giới thiệu deprecations chính thức, giúp dễ dàng hơn trong việc áp dụng thay đổi và tính năng mới trong khi duy trì sự rõ ràng cho các nhà phát triển.
- **Cam kết Python API**: Bất chấp việc chuyển đổi core sang Rust, NautilusTrader sẽ tiếp tục cung cấp Python API hướng đến người dùng.

## Vạch ra Tương lai

Lộ trình này xây dựng trên nền tảng vững mạnh của NautilusTrader, thúc đẩy việc tinh chỉnh liên tục trong khi
mở rộng khả năng và tính năng của nó cho các nhà giao dịch thuật toán và nhà phát triển.
