# 2. Method — Cryptographic Algorithms

**Phụ trách:** các thành viên phụ trách từng thuật toán trong các thư mục con; Vũ Minh Hoàng phụ trách RSA và ML-DSA.

## Cách tổ chức

Viết mỗi thuật toán trong thư mục con tương ứng. Để bám cách trình bày của `E_Voting_Report.pdf`, mỗi tiểu mục thuật toán dùng cùng hai tiêu đề con:

### 2.x.1 Mathematical Theory

Nêu cấu hình/tham số, nguyên lý và các bước tạo khóa/ký/xác minh; giải thích giả định toán học, cơ sở an toàn, tấn công chính và rủi ro triển khai/nonce.

### 2.x.2 Application in Banking System

Giải thích vai trò của thuật toán trong bối cảnh ngân hàng, yêu cầu tích hợp/quản lý khóa, ưu điểm, hạn chế và cấu hình phù hợp. Nếu thuật toán chỉ được xem xét như phương án thay thế, nói rõ điều đó; không mô tả như các thuật toán được ghép vào cùng một hệ thống.

## Yêu cầu biên soạn

- Tách bạch chi phí ký/xác minh với độ khó khôi phục khóa.
- Nêu đúng các tham số và cấu hình; không gộp nhiều biến thể như thể chúng giống nhau.
- Trình bày giả định an toàn có điều kiện, tránh gọi thuật toán “không thể phá”.
- Gửi cho nhóm phần 4 thông tin về tham số benchmark phù hợp, loại tấn công cần phân biệt và nguồn gốc của các nhận định.
- Nguồn ưu tiên: tiêu chuẩn/RFC, bài báo gốc và tài liệu mật mã chính thức.
- Có thể dùng phương trình và bảng tham số như mẫu, nhưng giải thích ký hiệu ngay cạnh hoặc ngay sau công thức; đánh số và trích dẫn nhất quán.

## Sản phẩm bàn giao

- Một `draft.md` cho mỗi thuật toán, có trích dẫn trong nội dung và mục nguồn.
- Theo dõi các điểm chưa thống nhất trong bản nháp để nhóm tổng hợp xử lý.
