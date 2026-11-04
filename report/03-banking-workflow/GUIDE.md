# 3. Mechanism of Operation — Banking Workflow

**Phụ trách chính:** Nguyễn Văn Đức.

## Mục tiêu

Thiết kế sơ đồ vòng đời giao dịch ngân hàng có chữ ký số, giải thích thành phần tham gia, dữ liệu được ký và các điểm kiểm soát. Sơ đồ cần thể hiện cả nhánh chấp nhận và từ chối.

## Bố cục theo mẫu

### 3.1 Cryptographic Components and Security Roles

Giải thích chữ ký số cung cấp bảo đảm nào trong workflow, thành phần nào ký/xác minh, và vai trò của chứng thư, HSM, TLS, xác thực/phân quyền. Các thuật toán ở mục 2 là các lựa chọn cần so sánh, không mặc định kết hợp tất cả trong một pipeline.

### 3.2 System Workflow Phases

Chia sơ đồ và phần mô tả thành các giai đoạn tuần tự. Với mỗi giai đoạn, ghi tác nhân, dữ liệu vào/ra, kiểm tra an toàn và điều kiện chấp nhận/từ chối.

## Các bước tối thiểu trong sơ đồ

1. Khởi tạo giao dịch và xác thực người yêu cầu.
2. Chuẩn hóa/canonicalize dữ liệu giao dịch.
3. Tạo nội dung cần ký, ràng buộc đầy đủ mã giao dịch, tài khoản nguồn/đích, số tiền, loại tiền, thời điểm, nonce hoặc mục đích.
4. Kiểm tra quyền/hạn mức phê duyệt theo chính sách.
5. Lấy khóa ký từ HSM hoặc môi trường bảo vệ khóa; thực hiện ký.
6. Gửi giao dịch và chữ ký qua kênh truyền được bảo vệ.
7. Phía nhận xác minh chữ ký, danh tính/chứng thư, trạng thái hiệu lực và quyền tương ứng.
8. Kiểm tra transaction ID/nonce để chống replay; kiểm tra quy tắc nghiệp vụ và gian lận.
9. Chấp nhận hoặc từ chối; ghi log/audit có thể truy vết.

## Sản phẩm bàn giao

- `workflow.mmd` (hoặc nguồn sơ đồ chỉnh sửa được khác) và ảnh xuất `workflow.png`/`workflow.svg`.
- `draft.md` giải thích vai trò từng thành phần, dữ liệu được ký, các điểm kiểm soát và xử lý lỗi.
- Ghi rõ ranh giới tin cậy, nơi khóa được sử dụng và cách xử lý chữ ký/chứng thư không hợp lệ.
- Đặt chú thích cho hình, đánh số hình và dẫn chiếu từ nội dung như cách trình bày trong mẫu.

## Rà soát

Chữ ký phải ràng buộc các trường quan trọng và có cơ chế chống replay. Chữ ký không thay TLS, xác thực người dùng, phân quyền, giám sát gian lận hay kiểm soát giao dịch.
