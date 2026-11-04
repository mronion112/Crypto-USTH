# 4. Testing and Performance Analysis

**Phụ trách:** Nguyễn Quang Vũ Hoàng (thiết kế/triển khai thử nghiệm); Nguyễn Hoàng Anh (bảng kết quả, phân tích, biên soạn và rà soát phương pháp). Thành viên thuật toán hỗ trợ giải thích kết quả.

## Mục tiêu

Thiết lập thử nghiệm có thể lặp lại, báo cáo thời gian/tốc độ trên CPU và GPU trong trường hợp áp dụng, rồi giới hạn kết luận đúng theo môi trường và kích thước khóa thử nghiệm. Thư mục con chia nội dung thành setup, benchmark và phân tích lượng tử.

## Cách trình bày kết quả theo mẫu

- Mở từng mục bằng mục đích kiểm tra, rồi đưa bảng test case hoặc bảng benchmark có tên, số liệu, đơn vị và kết quả.
- Sau mỗi bảng/hình, viết đoạn phân tích giải thích xu hướng, điểm bất thường và giới hạn; không để số liệu tự đứng thay cho nhận xét.
- Chỉ ghi kết quả đã thực sự chạy. Nếu một phép đo chưa thực hiện hoặc không áp dụng được thì đánh dấu rõ, không tự điền số liệu.
- Đánh số bảng/hình và dẫn chiếu trong phần văn bản; giữ cấu hình, phép đo và giới hạn nhất quán.

## Nguyên tắc bắt buộc

- Chỉ dùng dữ liệu thử nghiệm và không gian khóa thu nhỏ/tham số đồ chơi.
- Không thử brute force RSA-2048, RSA-4096 hoặc ECC 256-bit; không thử trên khóa, dữ liệu hay dịch vụ ngân hàng thật.
- Tách kết quả đo ký/xác minh (nếu có) khỏi thời gian tìm/khôi phục khóa.
- Phân biệt brute force toy key với GNFS cho phân tích thừa số RSA, Pollard’s rho cho ECDLP, lattice reduction cho PQC và khai thác nonce/lỗi triển khai.
- Nêu rõ phép đo không chứng minh khả năng phá khóa chuẩn; tránh ngoại suy tuyến tính thiếu căn cứ.
- Không yêu cầu hoặc tuyên bố benchmark trên máy tính lượng tử.

## Sản phẩm chung

- Mã nguồn thử nghiệm, hướng dẫn chạy và cấu hình phần cứng/phần mềm.
- Raw data, cách tính chỉ số và bảng/biểu đồ có chú thích đơn vị.
- `draft.md` diễn giải kết quả, độ biến thiên, giới hạn và mối liên hệ với phân tích lý thuyết.

Xem các GUIDE trong thư mục con để biết yêu cầu từng mục 4.1–4.3.
