# 6. References — Hướng dẫn biên soạn

**Phụ trách:** Nguyễn Hoàng Anh và Nguyễn Hoàng Minh; mọi thành viên cung cấp nguồn cho phần mình viết.

## Quy tắc nguồn

- Ưu tiên RFC, NIST FIPS, tiêu chuẩn quốc gia phù hợp, tài liệu cơ quan/nhà phát triển chính thức và bài báo khoa học gốc.
- Mỗi thuật toán cần nguồn mô tả tiêu chuẩn và nguồn hỗ trợ các nhận định an toàn/tấn công quan trọng.
- Với benchmark, trích dẫn thư viện/phiên bản, tài liệu phần cứng hoặc phương pháp nếu dùng; kết quả tự đo phải liên kết đến mã nguồn và dữ liệu trong repo.
- Kiểm tra tác giả/cơ quan, tên tài liệu, năm, mã tiêu chuẩn, URL/DOI và ngày truy cập nếu nguồn web có thể thay đổi.
- Không thêm nguồn chưa đọc hoặc trích dẫn không hỗ trợ luận điểm. Không bịa metadata còn thiếu.

## Định dạng và sản phẩm

- Bám cách đánh số tài liệu [1], [2], ... trong mẫu; dùng kiểu IEEE cho báo cáo kỹ thuật, trừ khi giảng viên yêu cầu kiểu khác.
- Tạo `references.md`, sắp xếp và đánh số theo lần xuất hiện hoặc quy tắc IEEE thống nhất. Đối chiếu từng trích dẫn trong bản thảo với danh sách này.
- Yêu cầu thành viên gửi nguồn kèm đoạn/luận điểm được nguồn hỗ trợ thay vì chỉ gửi URL.

## Nguồn nền tảng gợi ý từ tài liệu phân công

- RFC 8032 — Edwards-Curve Digital Signature Algorithm (EdDSA).
- NIST FIPS 186-5 — Digital Signature Standard (DSS).
- NIST FIPS 204 — Module-Lattice-Based Digital Signature Standard (ML-DSA).
- NIST FIPS 205 — Stateless Hash-Based Digital Signature Standard (SLH-DSA), nếu dùng để đối chiếu.
- NIST SP 800-57 — Recommendation for Key Management.
- Tiêu chuẩn/tài liệu chính thức liên quan đến SM2 và cấu hình triển khai được chọn.
- Shor, P. W. (1994), “Algorithms for quantum computation: discrete logarithms and factoring.”

Danh sách trên là điểm bắt đầu; cần xác minh phiên bản và bổ sung nguồn thực sự được trích trong báo cáo.
