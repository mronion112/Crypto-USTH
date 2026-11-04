# 1. Introduction — Hướng dẫn viết

**Phụ trách:** Nguyễn Hoàng Anh và Nguyễn Hoàng Minh.

## Mục tiêu phần

Giới thiệu bối cảnh, vấn đề nghiên cứu, phạm vi và câu hỏi mà báo cáo sẽ trả lời. Phần này cần dẫn dắt từ nhu cầu bảo đảm giao dịch ngân hàng đến việc đánh giá thuật toán chữ ký và khả năng chống tấn công.

## Cách trình bày theo mẫu

- Mở đầu ngắn bằng vấn đề thực tiễn, sau đó giải thích động lực và mục tiêu như trong hai mục 1.1–1.2 của mẫu.
- Đặt mục tiêu thành câu hỏi hoặc việc có thể kiểm tra ở phần kết quả; tránh biến Introduction thành phần kết luận.
- Dùng cùng ngôn ngữ với toàn báo cáo. Mẫu tham khảo viết bằng tiếng Anh.

## Nội dung cần có

### 1.1 Motivation

- Nêu cách chữ ký số hỗ trợ xác thực nguồn gửi, toàn vẹn dữ liệu và chống chối bỏ trong giao dịch.
- Giải thích vì sao khóa bí mật bị lộ, nonce yếu/tái sử dụng, lỗi triển khai và thay đổi năng lực tính toán là các rủi ro đáng nghiên cứu.
- Đặt vấn đề trong hệ thống ngân hàng: giao dịch giá trị cao, kiểm soát phê duyệt, kiểm toán và vòng đời khóa.
- Dẫn nguồn cho các nhận định về tiêu chuẩn, thực tiễn hoặc rủi ro.

### 1.2 Purpose of the Project

- Nêu mục tiêu nghiên cứu năm nhóm: Ed25519, SM2, RSA, ECDSA và ML-DSA.
- Nêu các câu hỏi nghiên cứu: cơ sở an toàn là gì; các tấn công nào đáng quan tâm; benchmark toy key cho biết điều gì; PQC thay đổi lộ trình ngân hàng ra sao.
- Khoanh phạm vi: phân tích thuật toán và mô hình workflow; đo đạc chỉ trên không gian khóa thu nhỏ; máy tính lượng tử được phân tích lý thuyết.
- Nêu rõ báo cáo không đánh giá hệ thống ngân hàng thật và không tuyên bố khôi phục được khóa chuẩn.

## Sản phẩm bàn giao

- `draft.md` gồm phần mở đầu, mục 1.1 và 1.2.
- Danh sách 3–5 câu hỏi nghiên cứu hoặc mục tiêu có thể đối chiếu với kết quả ở phần 4 và kết luận.
- Trích dẫn cho số liệu/bối cảnh và danh sách nguồn chuyển cho `06-references/`.

## Kiểm tra trước khi bàn giao

- Mục tiêu, phạm vi và nội dung phần 2–4 nhất quán với nhau.
- Không khẳng định chữ ký số tự nó bảo vệ toàn bộ giao dịch ngân hàng.
- Không đưa ra kết luận về độ an toàn chỉ dựa trên tốc độ ký/xác minh.
