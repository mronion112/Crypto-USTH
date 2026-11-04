# 2.3 RSA-2048 và RSA-4096 — Hướng dẫn viết

**Phụ trách:** Vũ Minh Hoàng.

## Bố cục trong `draft.md`

### 2.3.1 Mathematical Theory

- Phạm vi là chữ ký **RSA-PSS** với modulus RSA-2048 và RSA-4096; phân biệt RSA ký với mã hóa RSA.
- Tóm tắt tạo khóa và quy trình ký/xác minh RSA-PSS; nêu các tham số PSS quan trọng và cấu hình được giả định.
- Giải thích cơ sở an toàn liên quan đến phân tích thừa số số nguyên lớn, vai trò của hai số nguyên tố bí mật và tác động của kích thước khóa.
- Phân tích GNFS (General Number Field Sieve) là phương pháp phân tích thừa số tổng quát thực tế quan trọng; phân biệt với thử vét cạn mọi khóa.
- Nêu rủi ro padding/cấu hình sai, khóa yếu hoặc rò rỉ triển khai khi có nguồn phù hợp.
- Tóm tắt tác động lý thuyết của Shor ở mục 4.3 hoặc dẫn chiếu tới mục đó.

### 2.3.2 Application in Banking System

- So sánh RSA-2048 và RSA-4096 về kích thước, chi phí và tính phù hợp triển khai; không suy ra an toàn chỉ từ một lần đo.
- Mô tả vai trò có thể có của RSA-PSS trong ngân hàng và các yêu cầu khi triển khai/chọn tham số.

## Sản phẩm bàn giao

- `draft.md` và bảng so sánh RSA-2048/RSA-4096.
- Nguồn tiêu chuẩn/tài liệu chính thức về chữ ký và lựa chọn tham số.
- Gửi phần 4 mô tả rõ vì sao benchmark toy brute force không tương đương GNFS.

## Rà soát

Không chạy brute force trên khóa RSA chuẩn. Không gọi GNFS là brute force; không dùng kết quả toy key để ước lượng trực tiếp thời gian phá RSA-2048/4096.
