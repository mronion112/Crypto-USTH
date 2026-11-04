# 4.2 CPU/GPU Key-Recovery Benchmark

**Phụ trách triển khai:** Nguyễn Quang Vũ Hoàng. **Phân tích và biên soạn:** Nguyễn Hoàng Anh; thành viên thuật toán phối hợp diễn giải.

## Nội dung cần ghi

- Định nghĩa chính xác bài toán khôi phục khóa toy, không gian tìm kiếm, cách sinh khóa/thông điệp/chữ ký thử nghiệm và cách xác nhận nghiệm.
- Báo cáo kích thước không gian khóa, thời gian chạy, số phép thử/giây, số lượt chạy, độ phân tán và mức tăng tốc CPU/GPU nếu so sánh hợp lệ.
- Ghi cấu hình phần cứng/phần mềm và liên kết tới quy trình ở mục 4.1.
- Trình bày biểu đồ/bảng với đơn vị, mô tả xu hướng và các giới hạn đo được.
- Phân biệt rõ: brute force trên toy key; GNFS là phân tích thừa số RSA; Pollard’s rho giải ECDLP; tấn công nonce/lỗi triển khai khai thác điểm yếu khác; lattice reduction liên quan PQC.
- Chỉ ngoại suy trong phạm vi có giải thích và giả định. Không suy ra thời gian phá khóa chuẩn từ một phép đo toy key.

Ưu tiên bảng có cột: **Thuật toán/case**, **Kích thước toy**, **Nền tảng**, **Thời gian**, **Tốc độ thử**, **Số lượt/độ phân tán**. Nếu đưa baseline và tối ưu, mô tả chính xác hai cách cài đặt trước khi so sánh.

## Sản phẩm bàn giao

- Mã nguồn benchmark, lệnh chạy, raw data và bảng/biểu đồ.
- `draft.md` nêu kết quả, diễn giải, giới hạn và kết luận được dữ liệu hỗ trợ.
- Danh sách giả định và câu hỏi cần thành viên thuật toán rà soát.

## Rà soát

Không chạy brute force trên RSA-2048/RSA-4096 hay ECC 256-bit. Không diễn giải kết quả là khả năng tấn công hệ thống thực.
