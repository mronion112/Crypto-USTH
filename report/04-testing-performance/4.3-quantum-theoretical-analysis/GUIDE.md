# 4.3 Quantum Computer — Theoretical Analysis

**Phụ trách:** Nguyễn Hoàng Anh phối hợp với Vũ Minh Hoàng và các thành viên thuật toán.

## Nội dung cần có

- Giải thích tác động lý thuyết của thuật toán Shor lên phân tích thừa số RSA và logarit rời rạc trên đường cong elliptic (ảnh hưởng tới ECDSA, Ed25519 và SM2).
- Nêu điều kiện tổng quát để tấn công thực tế, gồm yêu cầu về máy lượng tử có khả năng sửa lỗi và quy mô tài nguyên; tránh khẳng định khi chưa có bằng chứng.
- Đối chiếu ML-DSA: nền tảng module-lattice và các giả định hậu lượng tử; nêu rằng “hậu lượng tử” không đồng nghĩa bảo đảm tuyệt đối.
- Có thể đề cập rủi ro “harvest now, decrypt/sign later” hoặc yêu cầu crypto-agility nếu liên quan trực tiếp đến chữ ký và lộ trình chuyển đổi.
- Trích dẫn bài báo gốc/nguồn chính thức; phân biệt kết quả lý thuyết, ước lượng tài nguyên và khả năng hiện có.

## Sản phẩm bàn giao

- `draft.md` với bảng tóm tắt thuật toán cổ điển/PQC, loại bài toán bị ảnh hưởng và kết luận phù hợp.
- Tài liệu tham khảo cho mọi con số hoặc dự đoán tài nguyên.

## Rà soát

Đây là phân tích lý thuyết, không phải benchmark lượng tử. Không tuyên bố máy tính lượng tử hiện tại đã phá khóa RSA/ECC chuẩn.
