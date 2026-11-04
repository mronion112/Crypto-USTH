# 2.4 ECDSA — Hướng dẫn viết

**Phụ trách:** Nguyễn Hoàng Minh.

## Bố cục trong `draft.md`

### 2.4.1 Mathematical Theory

- Chọn **một** đường cong làm cấu hình chính (secp256k1 hoặc NIST P-256), ghi rõ lý do và dùng nhất quán trong phân tích/thử nghiệm.
- Mô tả tạo khóa, ký và xác minh; giải thích vai trò của nonce k, hàm băm và các tham số đường cong.
- Phân tích cơ sở an toàn ECDLP và Pollard’s rho; nêu các giới hạn của cách đánh giá bằng số đo toy key.
- Giải thích vì sao tái sử dụng nonce hoặc nonce có thiên lệch có thể làm lộ khóa bí mật; trình bày biện pháp giảm rủi ro.

### 2.4.2 Application in Banking System

- So sánh ngắn ECDSA với Ed25519 và SM2 về cấu hình, quản lý nonce, khả năng tương thích và bối cảnh triển khai.
- Nêu khả năng áp dụng trong ngân hàng và các lưu ý về thư viện, sinh nonce, xác minh đầu vào.

## Sản phẩm bàn giao

- `draft.md`, ghi rõ đường cong và hàm băm sử dụng.
- Nguồn tiêu chuẩn hoặc tài liệu đường cong chính thức và tài liệu về ECDSA.
- Thông tin cấu hình thống nhất gửi nhóm benchmark.

## Rà soát

Không thử tìm khóa ECDSA chuẩn bằng brute force. Nếu nói về Pollard’s rho, phân biệt đây là thuật toán giải bài toán logarit rời rạc với benchmark vét cạn đồ chơi.
