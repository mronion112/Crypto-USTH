# Digital Signature Algorithms and Their Resistance to Attacks in Banking

Repository này là khung làm việc cho báo cáo nhóm về thuật toán chữ ký số trong giao dịch ngân hàng. Các thành viên soạn nội dung trong thư mục `report/`; mỗi phần có `GUIDE.md` ghi người phụ trách, nội dung cần viết, sản phẩm cần bàn giao và các điểm cần kiểm tra.

`E_Voting_Report.pdf` là mẫu tham khảo cách trình bày: trang bìa, mục lục, tiêu đề đánh số, giải thích thuật toán theo lý thuyết rồi đến ứng dụng, sơ đồ/bảng kết quả và phần kết luận kèm hướng phát triển. Nhóm dùng mẫu cho bố cục và cách kể chuyện; nội dung, số liệu và kết luận phải dựa trên đề tài và nguồn của nhóm mình.

## Mục tiêu

- Phân tích Ed25519, SM2, RSA, ECDSA và chữ ký hậu lượng tử ML-DSA.
- So sánh giả định an toàn, kiểu tấn công và khả năng triển khai trong bối cảnh ngân hàng.
- Mô tả vòng đời giao dịch có chữ ký số và các điểm kiểm soát an toàn.
- Thử nghiệm khôi phục khóa trên không gian khóa đồ chơi bằng CPU/GPU, rồi phân tích giới hạn của kết quả.
- Đánh giá lý thuyết tác động của máy tính lượng tử lên các thuật toán đang nghiên cứu.

## Cấu trúc báo cáo

```text
report/
├── REPORT_OUTLINE.md
├── 01-introduction/
├── 02-cryptographic-algorithms/
│   ├── 2.1-ed25519/
│   ├── 2.2-sm2/
│   ├── 2.3-rsa/
│   ├── 2.4-ecdsa/
│   └── 2.5-post-quantum-cryptography/
├── 03-banking-workflow/
├── 04-testing-performance/
│   ├── 4.1-test-cases-and-setup/
│   ├── 4.2-cpu-gpu-key-recovery/
│   └── 4.3-quantum-theoretical-analysis/
├── 05-conclusion/
└── 06-references/
```

## Phân công

| Thành viên | Phần phụ trách | Vai trò chính |
|---|---|---|
| Hoàng Thu An | 2.1 Ed25519 | Phân tích thuật toán, cơ sở an toàn, tấn công và ứng dụng |
| Nguyễn Tiến Đạt | 2.2 SM2 | Phân tích thuật toán, tham số, cơ sở an toàn và rủi ro triển khai |
| Vũ Minh Hoàng | 2.3 RSA; 2.5 ML-DSA | Phân tích RSA-2048/RSA-4096 và ML-DSA; tập trung thêm vào lattice/PQC |
| Nguyễn Hoàng Minh | 2.4 ECDSA; phối hợp 1, 5, 6 | Phân tích ECDSA; đồng biên soạn Introduction, Conclusion, References |
| Nguyễn Văn Đức | 3 Banking Workflow | Thiết kế sơ đồ và giải thích quy trình giao dịch ngân hàng |
| Nguyễn Quang Vũ Hoàng | 4 Testing & Performance | Thiết kế và triển khai chương trình đo CPU/GPU |
| Nguyễn Hoàng Anh | 1, 4, 5, 6 | Phối hợp Introduction, Conclusion, References; tổng hợp, phân tích và rà soát phương pháp benchmark |

## Yêu cầu chung

- Giữ nguyên tên đề tài tiếng Anh như tiêu đề README. Thống nhất ngôn ngữ báo cáo với yêu cầu của giảng viên trước khi ghép các phần.
- Mỗi phần cần có trích dẫn ngay tại luận điểm liên quan và danh sách nguồn đủ thông tin để truy xuất. Ưu tiên tiêu chuẩn, RFC, tài liệu chính thức và bài báo khoa học gốc.
- Với mỗi thuật toán, nêu rõ cấu hình/tham số đang phân tích, quy trình ký và xác minh, giả định toán học, dạng tấn công, rủi ro triển khai và ứng dụng phù hợp.
- Tách bạch tấn công toán học với lỗi triển khai hoặc quản lý khóa. Không gọi SHA-512 hay SM3 là thuật toán chữ ký độc lập trong các cấu hình đang mô tả.
- Benchmark chỉ dùng dữ liệu thử nghiệm và không gian khóa thu nhỏ/tham số đồ chơi. Không thử khôi phục khóa chuẩn như RSA-2048, RSA-4096 hoặc ECC 256-bit.
- Phân biệt brute force với GNFS (RSA), Pollard’s rho (ECC), lattice reduction (PQC) và khai thác nonce/lỗi triển khai. Không ngoại suy số đo toy key thành tuyên bố có thể phá khóa chuẩn.
- Phần máy tính lượng tử là phân tích lý thuyết; không tuyên bố máy tính lượng tử hiện tại đã phá được khóa RSA/ECC chuẩn.
- Chữ ký số không thay thế TLS, xác thực người dùng, phân quyền, kiểm soát gian lận hoặc bảo vệ khóa bằng HSM.

## Cách đóng góp

1. Xem [REPORT_OUTLINE.md](report/REPORT_OUTLINE.md) để thống nhất bố cục bản ghép, sau đó đọc `GUIDE.md` trong thư mục phần mình phụ trách.
2. Soạn bản nháp Markdown (`.md`) ngay trong thư mục tương ứng; đặt tên tệp theo nội dung, ví dụ `draft.md`, `benchmark-results.md` hoặc `workflow.mmd`.
3. Ghi nguồn cạnh nội dung được hỗ trợ và thêm đầy đủ thông tin tài liệu vào `06-references/`.
4. Với sơ đồ, lưu tệp nguồn có thể chỉnh sửa (Mermaid `.mmd` hoặc định dạng tương đương) cùng ảnh xuất để tiện rà soát.
5. Khi bàn giao, ghi rõ phần đã hoàn tất, giả định, giới hạn, nguồn còn thiếu và việc cần nhóm khác phối hợp.

Chưa ghép các bản nháp thành báo cáo cuối cùng. Sau khi các phần được rà soát chéo, nhóm có thể tổng hợp theo thứ tự đánh số trong `report/`.
