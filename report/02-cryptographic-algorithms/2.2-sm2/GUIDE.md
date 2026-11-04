# 2.2 SM2 — Hướng dẫn viết

**Phụ trách:** Nguyễn Tiến Đạt.

## Bố cục trong `draft.md`

### 2.2.1 Mathematical Theory

- Giới thiệu chữ ký SM2 và cấu hình/tiêu chuẩn đang dùng; ghi rõ phiên bản và bộ tham số đường cong.
- Trình bày các bước tạo khóa, tạo chữ ký và xác minh, tập trung vào những gì phân biệt SM2.
- Giải thích vai trò của ID người ký trong tính giá trị băm đầu vào và vai trò của SM3 trong cấu hình thường dùng.
- Nêu giả định an toàn dựa trên ECDLP và các tấn công liên quan.
- Phân tích rủi ro nonce: tái sử dụng, thiên lệch, sinh ngẫu nhiên kém; giải thích hệ quả và biện pháp phòng tránh.

### 2.2.2 Application in Banking System

- Đánh giá quản lý khóa, khả năng tương thích tiêu chuẩn và ứng dụng trong hệ thống ngân hàng.

## Sản phẩm bàn giao

- `draft.md` với tham số đầy đủ, quy trình ký/xác minh và nguồn tiêu chuẩn chính thức liên quan đến SM2.
- Ghi rõ ID, hàm băm và đường cong nào được giả định; không trộn cấu hình giữa các tiêu chuẩn.
- Gửi nhóm benchmark đề xuất tham số toy và lưu ý triển khai quan trọng.

## Rà soát

Không mô tả SM3 như thuật toán chữ ký độc lập. Nêu rõ benchmark toy key không đánh giá khả năng phá SM2 với tham số chuẩn.
