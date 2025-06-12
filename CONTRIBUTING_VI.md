# Đóng góp cho NautilusTrader

Chúng tôi rất trân trọng sự tham gia từ cộng đồng giao dịch, và tất cả các đóng góp đều được đánh giá cao vì chúng giúp chúng tôi liên tục cải thiện NautilusTrader!

## Các bước

Để đóng góp, hãy làm theo các bước sau:

1. Mở một issue trên GitHub để thảo luận về những thay đổi hoặc cải tiến mà bạn đề xuất.

2. Khi mọi người đã thống nhất, fork nhánh `develop` và đảm bảo fork của bạn được cập nhật bằng cách thường xuyên merge các thay đổi upstream.

3. Cài đặt và cấu hình [pre-commit](https://pre-commit.com/) trên máy local của bạn để tự động chạy code checks, formatters và linters trước mỗi commit.
   Bạn có thể cài đặt pre-commit với:
    ```bash
    pip install pre-commit
    pre-commit install
    ```

4. Mở một pull request (PR) trên nhánh `develop` với comment tóm tắt và tham chiếu tới các GitHub issue liên quan.

5. Hệ thống CI sẽ chạy toàn bộ test suite trên code của bạn bao gồm tất cả unit và integration tests, vì vậy hãy bao gồm các tests thích hợp với PR.

6. [Deepsource](https://deepsource.io) sẽ thực hiện code review tự động. Sửa bất kỳ vấn đề nào gây ra failed check, và thêm commit vào PR của bạn.

7. Đọc và hiểu Contributor License Agreement (CLA), có sẵn tại https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md.

8. Bạn cũng sẽ được yêu cầu ký CLA, được quản lý tự động thông qua [CLA Assistant](https://cla-assistant.io/).

9. Chúng tôi sẽ review code của bạn càng nhanh càng tốt và cung cấp feedback nếu cần bất kỳ thay đổi nào trước khi merge.

## Mẹo

- Tuân theo các thực hành coding đã được thiết lập trong [Developer Guide](https://nautilustrader.io/docs/developer_guide/index.html).
- Giữ PRs nhỏ và tập trung để dễ review hơn.
- Tham chiếu các GitHub issue liên quan trong comment PR của bạn.
