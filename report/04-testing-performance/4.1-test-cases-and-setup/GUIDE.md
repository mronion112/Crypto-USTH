# 4.1 Test Cases and Experimental Setup

**Phụ trách:** Nguyễn Quang Vũ Hoàng và Nguyễn Hoàng Anh.

## Nội dung cần ghi

- CPU, GPU (nếu có), RAM, hệ điều hành, compiler/runtime, ngôn ngữ và phiên bản thư viện mật mã.
- Mục tiêu từng test case, thuật toán, kích thước/tham số toy, cách tạo dữ liệu và tiêu chí phát hiện khóa đúng.
- Số lượt chạy, cách khởi động/đo thời gian, warm-up, đồng hồ sử dụng, cách xử lý ngoại lệ và cách báo cáo trung bình/trung vị/độ phân tán.
- Cách bảo đảm CPU/GPU thực hiện cùng một tác vụ tương đương; mô tả hạn chế nếu phép đo không hoàn toàn tương đương.
- Cách lưu mã nguồn, cấu hình, lệnh chạy, raw data và kết quả để người khác tái lập.
- Giới hạn máy cá nhân, thư viện, kích thước toy và mọi yếu tố có thể ảnh hưởng kết quả.

Trình bày test case dạng bảng như mẫu, với các cột gợi ý: **Test case**, **Mục tiêu/Dữ liệu vào**, **Kết quả mong đợi**, **Thời gian hoặc số đo**, **Kết quả thực tế**. Có thể nhóm các case theo correctness, nonce/implementation risk, hoặc key recovery. Không đưa phép đo tải web vào báo cáo nếu nhóm không thực hiện.

## Sản phẩm bàn giao

- `draft.md` mô tả phương pháp và môi trường.
- `test-cases.md` hoặc bảng test case; tệp cấu hình/lệnh chạy và ghi chú cách tái lập.
- Không đưa thông tin bí mật hoặc khóa thật vào dữ liệu thử nghiệm.

## Rà soát

Mô tả đủ chi tiết để người khác lặp lại phép đo. Ghi rõ benchmark toy key không đại diện cho độ an toàn của khóa chuẩn.
