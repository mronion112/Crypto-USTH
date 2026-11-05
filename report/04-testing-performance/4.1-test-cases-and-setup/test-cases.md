# 4.1 Test Cases

Bảng dưới đây ghi lại các test case đã thiết kế, kết quả mong đợi và kết quả thực tế. Các phép đo thời gian chi tiết nằm trong mục 4.2; bảng này chỉ tóm tắt trạng thái và số đo correctness.

| ID | Nhóm | Mục tiêu / dữ liệu vào | Kết quả mong đợi | Số đo / thời gian | Kết quả thực tế |
|---|---|---|---|---|---|
| C1 | Correctness | RSA-PSS, khóa 2048-bit, thông điệp cố định | Ký và xác minh thành công | Kích thước chữ ký | **PASS**, 256 B |
| C2 | Correctness | RSA-PSS, khóa 4096-bit, thông điệp cố định | Ký và xác minh thành công | Kích thước chữ ký | **PASS**, 512 B |
| C3 | Correctness | ECDSA trên P-256, SHA-256 | Ký và xác minh thành công | Kích thước chữ ký DER | **PASS**, 71 B |
| C4 | Correctness | Ed25519, thông điệp cố định | Ký và xác minh thành công | Kích thước chữ ký | **PASS**, 64 B |
| C5 | Correctness | SM2/SM3 qua OpenSSL CLI | Ký và xác minh thành công | Kích thước chữ ký DER | **PASS**, 72 B |
| C6 | Correctness | ML-DSA-44, thông điệp cố định | Ký và xác minh thành công | Kích thước chữ ký | **PASS**, 2420 B |
| C7 | Correctness | ML-DSA-65, thông điệp cố định | Ký và xác minh thành công | Kích thước chữ ký | **PASS**, 3309 B |
| C8 | Correctness | ML-DSA-87, thông điệp cố định | Ký và xác minh thành công | Kích thước chữ ký | **PASS**, 4627 B |
| I1 | Implementation risk | Hai chữ ký ECDSA P-256 dùng cùng nonce | Khôi phục đúng khóa bí mật; không tìm kiếm vét cạn | $d_{\mathrm{rec}} = d$, $dG = Q$, hai chữ ký xác minh | **PASS** |
| K1 | Key recovery | RSA toy, 32/40/48/56 bit; 10 modulus mỗi kích thước | Tìm được thừa số, $p \cdot q = n$ | Thời gian, tốc độ | **PASS**, xem mục 4.2 |
| K2 | Key recovery | ECDLP toy, $p$ 18/22/26 bit; 10 khóa mỗi đường cong | Tìm được $d$, $dG = Q$ | Thời gian, tốc độ | **PASS**, xem mục 4.2 |
| S1 | Scaling | Cùng phép đo với 1 luồng và 12 luồng | 12 luồng nhanh hơn ở kích thước đủ lớn | Tỷ số thời gian | **Đã đo**, xem mục 4.2 |
| G1 | GPU | Cùng bài toán toy trên NVIDIA RTX 4050 Laptop GPU | Kết quả và xác nhận giống CPU | Wall time, kernel time | **PASS**, xem mục 4.2 |

## Cách xác nhận nghiệm

- RSA toy: kiểm tra $\mathit{factor} > 1$, $n \bmod \mathit{factor} = 0$, sau đó kiểm tra cả $\mathit{factor}$ và $n / \mathit{factor}$ là số nguyên tố bằng Miller-Rabin xác định.
- ECDLP toy: kiểm tra $\mathit{scalar}$ nằm trong $[1, q - 1]$, $\mathit{scalar} = d$ (dùng cho kiểm tra) và $\mathit{scalar} \cdot G = Q$.
- Nonce reuse: kiểm tra nonce khôi phục bằng $\frac{z_1 - z_2}{s_1 - s_2}$, khóa khôi phục bằng $\frac{s_1 k - z_1}{r}$, sau đó kiểm tra $dG = Q$ và xác minh độc lập hai chữ ký.

## Giới hạn của bảng test case

- C1–C8 chỉ kiểm tra correctness của thư viện hoặc OpenSSL, không so sánh tốc độ ký/xác minh.
- I1 là tấn công đại số do lỗi triển khai, không phải brute force và không phải tấn công ECDLP.
- K1–K2 dùng tham số đồ chơi. Không chạy brute force trên RSA-2048, RSA-4096 hoặc ECC 256-bit.
- G1 đã có số đo GPU; dữ liệu thô nằm trong `data/raw/gpu_cubecl_raw.csv`, bảng tổng hợp trong `data/summary/gpu_cubecl_summary.csv` của mục 4.2.
