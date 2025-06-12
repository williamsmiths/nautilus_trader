# NautilusTrader 1.218.0 Beta

Phát hành vào ngày 31 tháng 5, 2025 (UTC).

### Cải tiến
- Thêm re-exports tiện lợi cho Betfair adapter (constants, configs, factories, types)
- Thêm re-exports tiện lợi cho Binance adapter (constants, configs, factories, loaders, types)
- Thêm re-exports tiện lợi cho Bybit adapter (constants, configs, factories, loaders, types)
- Thêm re-exports tiện lợi cho Coinbase International adapter (constants, configs, factories)
- Thêm re-exports tiện lợi cho Databento adapter (constants, configs, factories, loaders, types)
- Thêm re-exports tiện lợi cho dYdX adapter (constants, configs, factories)
- Thêm re-exports tiện lợi cho Polymarket adapter (constants, configs, factories)
- Thêm re-exports tiện lợi cho Tardis adapter (constants, configs, factories, loaders)
- Thêm hỗ trợ cho `FillModel`, `LatencyModel` và `FeeModel` trong BacktestNode (#2601), cảm ơn @faysou
- Thêm bars caching từ `request_aggregated_bars` (#2649), cảm ơn @faysou
- Thêm `BacktestDataIterator` vào backtest engine để cung cấp data loading on-the-fly (#2545), cảm ơn @faysou
- Thêm hỗ trợ cho `MarkPriceUpdate` streaming từ catalog (#2582), cảm ơn @bartolootrit
- Thêm hỗ trợ cho Binance Futures margin type (#2660), cảm ơn @bartolootrit
- Thêm hỗ trợ cho Binances mark price stream trên tất cả markets (#2670), cảm ơn @sunlei
- Thêm tùy chọn config `bars_timestamp_on_close` cho Databento mặc định là `True` để nhất quán với conventions của Nautilus
- Thêm hỗ trợ `activation_price` cho trailing stop orders (#2610), cảm ơn @hope2see
- Thêm trailing stops cho OrderFactory bracket orders (#2654), cảm ơn @hope2see
- Thêm tùy chọn config `raise_exception` cho `BacktestRunConfig` (mặc định `False` để giữ hành vi hiện tại) sẽ raise exceptions để ngắt quy trình chạy của nodes
- Thêm phương thức tiện lợi `UnixNanos::is_zero()` để kiểm tra giá trị zero/epoch
- Thêm SQL schema, model, và query cho `OrderCancelRejected`
- Thêm SQL schema, model, và query cho `OrderModifyRejected`
- Thêm HyperSync client vào blockchain adapter (#2606), cảm ơn @filipmacek
- Thêm hỗ trợ cho DEXs, pools, và tokens vào blockchain adapter (#2638), cảm ơn @filipmacek

### Thay đổi Phá vỡ
- Thay đổi trailing stops để sử dụng `activation_price` thay vì `trigger_price` cho Binance để phù hợp hơn với conventions của Binance API

### Cải tiến Nội bộ
- Thêm `activation_price` str và repr tests cho trailing stop orders (#2620), cảm ơn @hope2see
- Thêm condition check cho order `contingency_type` và `linked_order_ids` nơi contingency nên có associated linked order IDs
- Cải thiện tính robust của socket client reconnects và disconnects để tránh state race conditions
- Cải thiện error handling cho socket clients, giờ sẽ raise Python exceptions trên send errors thay vì chỉ logging với `tracing`
- Cải thiện error handling cho Databento adapter bằng cách thay đổi nhiều unwraps thành log hoặc raise Python exceptions (nếu applicable)
- Cải thiện error handling cho Tardis adapter bằng cách thay đổi nhiều unwraps thành log hoặc raise Python exceptions (nếu applicable)
- Cải thiện fill behavior cho limit orders trong `L1_MBP` books, giờ sẽ fill toàn bộ size khi marketable như `TAKER` hoặc market di chuyển qua limit như `MAKER`
- Cải thiện account state event generation cho margin accounts, tránh generation của redundant intermediate account states cho cùng execution event
- Cải thiện ergonomics của messaging topics, patterns, và endpoints trong Rust (#2658), cảm ơn @twitu
- Cải thiện development debug builds với cranelift backend cho Rust (#2640), cảm ơn @twitu
- Cải thiện validations cho `LimitOrder` trong Rust (#2613), cảm ơn @nicolad
- Cải thiện validations cho `LimitIfTouchedOrder` trong Rust (#2533), cảm ơn @nicolad
- Cải thiện validations cho `MarketIfTouchedOrder` trong Rust (#2577), cảm ơn @nicolad
- Cải thiện validations cho `MarketToLimitOrder` trong Rust (#2584), cảm ơn @nicolad
- Cải thiện validations cho `StopLimitOrder` trong Rust (#2593), cảm ơn @nicolad
- Cải thiện validations cho `StopMarketOrder` trong Rust (#2596), cảm ơn @nicolad
- Cải thiện validations cho `TrailingStopMarketOrder` trong Rust (#2607), cảm ơn @nicolad
- Cải thiện orders initialize và display tests trong Rust (#2617), cảm ơn @nicolad
- Cải thiện testing cho Rust orders module (#2578), cảm ơn @dakshbtc
- Cải thiện Cython-Rust indicator parity cho `AdaptiveMovingAverage` (AMA) (#2626), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `DoubleExponentialMovingAverage` (DEMA) (#2633), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `ExponentialMovingAverage` (EMA) (#2642), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `HullMovingAverage` (HMA) (#2648), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `LinearRegression` (#2651), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `WilderMovingAverage` (RMA) (#2653), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `VariableIndexDynamicAverage` (VIDYA) (#2659), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `SimpleMovingAverage` (SMA) (#2655), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `VolumeWeightedAveragePrice` (VWAP) (#2661), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `WeightedMovingAverage` (WMA) (#2662), cảm ơn @nicolad
- Cải thiện Cython-Rust indicator parity cho `ArcherMovingAveragesTrends` (AMAT) (#2669), cảm ơn @nicolad
- Cải thiện zero size trade logging cho Binance Futures (#2588), cảm ơn @bartolootrit
- Cải thiện error handling trên API key authentication errors cho Polymarket
- Cải thiện execution client debug logging cho Polymarket
- Cải thiện exception khi deserializing order từ cache database
- Cải thiện `None` condition checks cho value types, giờ raise `TypeError` thay vì `AttributeError` khó hiểu
- Thay đổi `VecDeque` thành fixed-capacity `ArrayDeque` trong SMA indicator (#2666), cảm ơn @nicolad
- Thay đổi `VecDeque` thành fixed-capacity `ArrayDeque` trong LinearRegression (#2667), cảm ơn @nicolad
- Implement Display còn lại cho orders trong Rust (#2614), cảm ơn @nicolad
- Implement `_subscribe_instrument` cho dYdX và Bybit (#2636), cảm ơn @davidsblom
- Untangled `ratelimiter` quota từ `python` flag (#2595), cảm ơn @twitu
- Tinh chỉnh `BacktestDataIterator` correctness (#2591), cảm ơn @faysou
- Tinh chỉnh formatting của IB adapter files (#2639), cảm ơn @faysou
- Tối ưu message bus topic-matching logic trong Rust gấp 100× (#2634), cảm ơn @twitu
- Thay đổi thành faster message bus pattern matching logic từ Rust (#2643), cảm ơn @twitu
- Nâng cấp Rust (MSRV) lên 1.87.0
- Nâng cấp Cython lên v3.1.0 (giờ stable)
- Nâng cấp `databento` crate lên v0.26.0
- Nâng cấp `redis` crate lên v0.31.0
- Nâng cấp `sqlx` crate lên v0.8.6
- Nâng cấp `tokio` crate lên v1.45.1

### Sửa lỗi
- Sửa portfolio account updates dẫn đến balances không chính xác (#2632, #2637), cảm ơn báo cáo @bartolootrit và @DeirhX
- Sửa portfolio handling của `OrderExpired` events không cập nhật state (margin requirements có thể thay đổi)
- Sửa event handling cho `ExecutionEngine` để nó cập nhật đầy đủ `Portfolio` trước khi publish execution events (#2513), cảm ơn báo cáo @stastnypremysl
- Sửa PnL calculation cho margin account khi position flip (#2657), cảm ơn báo cáo @Egisess
- Sửa notional value pre-trade risk check khi order sử dụng quote quantity (#2628), cảm ơn báo cáo @DeevsDeevs
- Sửa position snapshot cache access cho `ExecutionEngine`
- Sửa position snapshot `SystemError` calling `copy.deepcopy()` bằng cách đơn giản sử dụng `pickle` round trip để copy position instance
- Sửa event purging edge cases cho account và position nơi ít nhất một event phải được guaranteed
- Sửa authentication cho Redis khi password được cung cấp mà không có username
- Sửa various numpy và pandas FutureWarning(s)
- Sửa sockets exponential backoff immediate reconnect value khi reset (điều này ngăn immediate reconnects trong reconnect sequence tiếp theo)
- Sửa message bus subscription matching logic trong Rust (#2646), cảm ơn @twitu
- Sửa trailing stop market fill behavior khi top-level exhausted để align với market orders (#2540), cảm ơn báo cáo @stastnypremysl
- Sửa stop limit fill behavior khi initial trigger nơi limit order tiếp tục fill như taker beyond available liquidity, cảm ơn báo cáo @hope2see
- Sửa matching engine trade processing khi aggressor side là `NO_AGGRESSOR` (chúng ta vẫn có thể cập nhật matching core)
- Sửa modifying và updating trailing stop orders (#2619), cảm ơn @hope2see

---

**Ghi chú**: Đây chỉ là phần trích dẫn của file RELEASES.md gốc. File đầy đủ chứa lịch sử phát hành chi tiết của tất cả các phiên bản NautilusTrader. Để xem toàn bộ lịch sử phát hành, vui lòng tham khảo file RELEASES.md gốc.
