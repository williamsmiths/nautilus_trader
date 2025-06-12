# Ví dụ

Các ví dụ mã sau được tổ chức theo ngữ cảnh môi trường hệ thống:

- **Backtest**: Dữ liệu lịch sử với venues mô phỏng.
- **Sandbox**: Dữ liệu thời gian thực với venues mô phỏng.
- **Live**: Dữ liệu thời gian thực với venues thực (paper trading hoặc tài khoản thật).
- **Other**: Các ví dụ khác nhau ngoài strategies.

Scripts trong mỗi thư mục ngữ cảnh môi trường được tổ chức theo tích hợp.

Đảm bảo rằng package `nautilus_trader` được biên dịch từ source hoặc cài đặt qua pip trước khi
chạy các ví dụ. Xem [hướng dẫn cài đặt](https://nautilustrader.io/docs/latest/getting_started/installation)
để biết thêm thông tin.

Để thực thi một example script từ thư mục `examples`, sử dụng lệnh tương tự như sau:

```
python backtest/crypto_ema_cross_ethusdt_trade_ticks.py
```
