# 2.5 Post-Quantum Cryptography — ML-DSA

**Phụ trách:** Vũ Minh Hoàng.

## Bố cục trong `draft.md`

### 2.5.1 Mathematical Theory

- Lấy ML-DSA theo NIST FIPS 204 làm trọng tâm; ghi rõ bộ tham số ML-DSA-44, ML-DSA-65 hoặc ML-DSA-87 nếu đi sâu vào một cấu hình.
- Giải thích ở mức khái niệm nền tảng lattice/module-lattice và các giả định liên quan; nêu rõ giới hạn hiểu biết hoặc phạm vi không đi vào chứng minh.
- Mô tả khái quát tạo khóa, ký, xác minh và các đặc trưng thực tiễn như kích thước khóa/chữ ký.
- Phân tích các hướng tấn công, gồm lattice reduction, cùng với lý do không thể diễn giải chúng như brute force thông thường.

### 2.5.2 Application in Banking System

- Đánh giá kích thước, hiệu năng, lưu trữ, truyền tải, tích hợp và lộ trình crypto-agility trong ngân hàng.
- Có thể đối chiếu ngắn với SLH-DSA theo FIPS 205; giữ ML-DSA là trọng tâm.

## Sản phẩm bàn giao

- `draft.md`, trích dẫn FIPS 204 và nguồn kỹ thuật đáng tin cậy.
- Bảng so sánh ngắn ML-DSA với các chữ ký cổ điển về nền tảng, kích thước và rủi ro lượng tử.
- Ghi rõ đây là đánh giá trên các giả định hiện tại, không phải cam kết an toàn tuyệt đối.

## Rà soát

Không mô tả ML-DSA là đã được chứng minh không thể bị phá. Không dùng benchmark CPU/GPU toy key của thuật toán khác để tuyên bố định lượng an toàn ML-DSA.
