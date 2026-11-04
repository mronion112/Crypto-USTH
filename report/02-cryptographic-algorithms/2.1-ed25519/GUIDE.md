# 2.1 Ed25519 — Hướng dẫn viết

**Phụ trách:** Hoàng Thu An.

## Bố cục trong `draft.md`

### 2.1.1 Mathematical Theory

- Giới thiệu Ed25519 như cấu hình EdDSA trên đường cong Edwards; phân biệt với các biến thể EdDSA khác.
- Mô tả tạo khóa, ký và xác minh ở mức khái niệm; chỉ rõ thông điệp nào được ký.
- Giải thích vai trò của SHA-512 trong cấu trúc Ed25519. SHA-512 là hàm băm thành phần, không phải thuật toán chữ ký.
- Nêu cơ sở an toàn liên quan đến bài toán logarit rời rạc trên đường cong elliptic (ECDLP) và các giả định/điều kiện liên quan.
- Phân tích Pollard’s rho ở mức phù hợp, rủi ro sinh/quản lý khóa, lỗi triển khai, kiểm tra đầu vào và kênh kề nếu có nguồn.

### 2.1.2 Application in Banking System

- Đánh giá ưu, nhược điểm và khả năng dùng trong hệ thống ngân hàng; lưu ý tương thích và yêu cầu tiêu chuẩn/chính sách của môi trường triển khai.

## Sản phẩm bàn giao

- `draft.md` theo cấu trúc chung của mục 2.
- Nguồn chính, tối thiểu RFC 8032 cùng các tài liệu kỹ thuật cần thiết.
- Ghi chú ngắn về điều gì có thể/không thể kết luận từ benchmark toy key của mục 4.

## Rà soát

Không mô tả SHA-512 như một chữ ký; không nói benchmark khóa nhỏ chứng minh có thể phá Ed25519 chuẩn.
