# 4.1 Test Cases and Experimental Setup

## 4.1.1 Mục tiêu và phạm vi

Mục này thiết lập một quy trình thử nghiệm có thể lặp lại cho hai nhóm nội dung:

1. **Correctness và rủi ro triển khai:** kiểm tra chữ ký của các họ thuật toán được phân tích trong mục 2 và minh họa rủi ro tái sử dụng nonce trong ECDSA.
2. **Khôi phục khóa trên tham số đồ chơi:** đo thời gian và tốc độ tìm khóa trên không gian khóa thu nhỏ cho RSA và bài toán logarit rời rạc trên đường cong elliptic (ECDLP).

Thử nghiệm chỉ dùng khóa, thông điệp và đường cong sinh cho mục đích đo đạc. Không sử dụng khóa, dữ liệu giao dịch hay dịch vụ ngân hàng thật. Không chạy brute force trên RSA-2048, RSA-4096 hoặc đường cong chuẩn 256-bit.

## 4.1.2 Môi trường thử nghiệm

Bảng 4.1 ghi cấu hình tại thời điểm thu thập số liệu CPU và GPU.

**Bảng 4.1. Cấu hình phần cứng và phần mềm**

| Thành phần | Giá trị |
|---|---|
| CPU | AMD Ryzen 5 7535HS, 6 lõi / 12 luồng |
| RAM | 14 GiB |
| Hệ điều hành | CachyOS, kernel Linux 7.3.1-1-cachyos x86_64 |
| Rust | rustc 1.99.1, cargo 1.99.1 |
| Python | 3.14.9 |
| cryptography | 50.0.2 |
| OpenSSL | 3.6.5 |
| CUDA toolkit | 13.4 (V13.4.92) |
| CubeCL | 0.11.3 |
| GPU | NVIDIA GeForce RTX 4050 Laptop GPU (AD107M), 6141 MiB; driver 615.78.08; compute capability 8.9 |

GPU benchmark chạy trên cùng máy với cấu hình trong Bảng 4.1. Thông tin driver, bộ nhớ và compute capability được lấy từ `nvidia-smi` khi chạy script GPU. Kết quả CPU và GPU được trình bày riêng trong mục 4.2; các bảng GPU chỉ dùng số đo thực, không dùng giá trị ước lượng. Bản ghi thô của lần thu thập CPU nằm trong `data/environment.txt` của thư mục này.

## 4.1.3 Thiết kế test case

Các test case được chia thành bốn nhóm:

| Nhóm | Mục đích | Case |
|---|---|---|
| Correctness | Kiểm tra tạo khóa, ký và xác minh của từng họ thuật toán | C1–C8 |
| Implementation risk | Minh họa khôi phục khóa do tái sử dụng nonce, không dùng tìm kiếm vét cạn | I1 |
| Key recovery | Đo thời gian và tốc độ khôi phục khóa toy | K1–K2 |
| Scaling và nền tảng | So sánh 1 luồng với 12 luồng; chuẩn bị phép đo GPU | S1, G1 |

Bảng đầy đủ về dữ liệu vào, kết quả mong đợi và kết quả thực tế nằm trong `test-cases.md`. Các test case correctness dùng thư viện chuẩn, không đo hiệu năng và không dùng để so sánh thuật toán. Các test case key recovery dùng tham số đồ chơi và chỉ đo chi phí tìm kiếm.

C1–C2 tuân theo cấu hình RSA-PSS trong FIPS 186-5 [[1]](../4.2-cpu-gpu-key-recovery/references.md#ref-1); C4 dùng Ed25519 theo RFC 8032 [[2]](../4.2-cpu-gpu-key-recovery/references.md#ref-2); C5 dùng SM2 theo GB/T 32918.2-2016 [[3]](../4.2-cpu-gpu-key-recovery/references.md#ref-3); C6–C8 dùng ML-DSA theo FIPS 204 [[4]](../4.2-cpu-gpu-key-recovery/references.md#ref-4). Phần SM2 chạy qua OpenSSL CLI [[5]](../4.2-cpu-gpu-key-recovery/references.md#ref-5); các họ còn lại dùng thư viện `cryptography` [[6]](../4.2-cpu-gpu-key-recovery/references.md#ref-6).

## 4.1.4 Tham số đồ chơi và sinh dữ liệu

### RSA toy

Mỗi mẫu là một modulus $n = p \cdot q$ có độ dài 32, 40, 48 hoặc 56 bit; $p$ và $q$ là số nguyên tố ngẫu nhiên có kích thước xấp xỉ một nửa modulus. Với mỗi kích thước, 10 modulus được sinh từ seed cố định `20261105`. Lần quét toàn bộ dùng mẫu đầu tiên; 10 lần đo thời gian khôi phục dùng lần lượt 10 mẫu.

Bài toán được phát biểu là tìm một ước không tầm thường của $n$ bằng cách thử các ứng viên dạng $6k - 1$ và $6k + 1$. Nghiệm được xác nhận khi $p \cdot q = n$ và cả hai thừa số vượt qua kiểm tra Miller-Rabin xác định.

### ECDLP toy

Ba đường cong Weierstrass ngắn $y^2 \equiv x^3 + ax + b \pmod{p}$ được sinh ngẫu nhiên với $p$ có 18, 22 và 26 bit. Công cụ sinh đường cong đếm điểm bằng ký hiệu Legendre, phân tích bậc nhóm và chọn một nhóm con cấp nguyên tố $q$. Bảng 4.2 ghi tham số đã dùng.

**Bảng 4.2. Tham số đường cong toy**

| $p$ bits | $p$ | $q$ | Bậc nhóm | Đối hạng |
|---|---|---|---|---|
| 18 | 163987 | 81737 | 163474 | 2 |
| 22 | 2610967 | 2609701 | 2609701 | 1 |
| 26 | 54523747 | 54518393 | 54518393 | 1 |

Với mỗi đường cong, 10 khóa bí mật $d$ được sinh trong khoảng $[1, q - 1]$; khóa công khai là $Q = dG$. Lần quét toàn bộ dùng khóa đầu tiên; 10 lần đo khôi phục dùng lần lượt 10 khóa.

Các đường cong này là đường cong thường (ordinary) nhưng rất nhỏ. Chúng chỉ dùng để đo chi phí tìm kiếm vét cạn, không đại diện cho đường cong chuẩn và không dùng để suy ra độ an toàn của ECDSA, Ed25519 hay SM2.

### Nonce reuse và smoke test

Test case I1 dùng đường cong P-256 chuẩn, sinh một khóa thử nghiệm và hai chữ ký có cùng nonce. Khóa được khôi phục bằng đại số từ hai phương trình chữ ký; không có bước tìm kiếm khóa. Kết quả được xác nhận bằng $dG = Q$ và bằng xác minh chữ ký độc lập qua thư viện `cryptography`. Rủi ro tái sử dụng nonce và yêu cầu sinh nonce duy nhất được quy định trong FIPS 186-5 [[1]](../4.2-cpu-gpu-key-recovery/references.md#ref-1) và RFC 6979 [[7]](../4.2-cpu-gpu-key-recovery/references.md#ref-7).

Các smoke test C1–C8 sinh khóa ngắn hạn trong bộ nhớ, ký một thông điệp cố định và xác minh ngay. Không có khóa riêng nào được ghi vào kho dữ liệu.

## 4.1.5 Quy trình đo

### Đồng hồ và số lần chạy

- CPU: đo bằng `std::time::Instant` bao quanh toàn bộ tìm kiếm. Binary được build bằng profile release của Cargo: `opt-level = 3`, `lto = "thin"`, `codegen-units = 1`.
- GPU: đo wall time bằng `std::time::Instant` và thời gian kernel bằng CUDA event.
- Mỗi cấu hình có một lần chạy warm-up không ghi nhận, sau đó 10 lần đo.
- CPU và GPU được đo trên nguồn AC để giảm biến thiên tần số do chính sách năng lượng.
- Báo cáo trung vị, khoảng tứ phân vị (IQR), nhỏ nhất, lớn nhất và trung bình. Không loại bỏ ngoại lai.
- CPU được đo với 1 luồng và 12 luồng. GPU dùng block 256 luồng.

### Hai chế độ đo

- **Full scan:** thử toàn bộ không gian khóa. Phép đo này cho tốc độ xử lý ứng viên và ít phụ thuộc vị trí khóa mục tiêu.
- **Recovery:** tìm một khóa mục tiêu ngẫu nhiên và dừng khi tìm thấy. Phép đo này cho thời gian khôi phục thực tế, nhưng có độ phân tán lớn vì vị trí khóa thay đổi giữa các lần chạy.

Tốc độ được tính bằng số ứng viên chia cho thời gian. Với chế độ recovery, tốc độ chỉ mang tính tham khảo vì số ứng viên đã thử phụ thuộc vị trí khóa và việc dừng sớm của các luồng.

### Tương đương CPU/GPU

Hai phiên bản dùng cùng không gian tìm kiếm, cùng thứ tự ứng viên và cùng tiêu chí xác nhận. Cách cài đặt khác nhau: CPU duyệt tuần tự theo khối và cập nhật điểm bằng phép cộng điểm; GPU dùng một thread cho mỗi ứng viên hoặc mỗi phép nhân vô hướng. Vì vậy phép so sánh CPU/GPU chỉ hợp lệ khi cả hai cùng kích thước toy và cùng chế độ đo; thời gian kernel và thời gian end-to-end được báo riêng.

## 4.1.6 Tái lập

Toàn bộ quy trình CPU chạy bằng:

```bash
cd report/04-testing-performance/4.2-cpu-gpu-key-recovery
bash bench/scripts/run_all.sh
```

Các bước riêng và lệnh chạy GPU nằm trong README của mục 4.2. Dữ liệu thô được lưu ở `data/raw/`, bảng tổng hợp ở `data/summary/`, hình ở `figures/`. Seed, kích thước và số lần chạy nằm trong `bench/config/bench.json`; tham số đường cong nằm trong `bench/config/curves.json`.

Các kiểm tra tự động gồm:

```bash
cargo test --release --offline --lib --bins --tests --manifest-path bench/Cargo.toml
python3 bench/scripts/smoke_sign_verify.py
python3 bench/scripts/nonce_reuse.py
```

## 4.1.7 Giới hạn

- Kích thước toy không đại diện cho khóa chuẩn. Không ngoại suy thời gian toy thành thời gian phá RSA-2048, RSA-4096 hoặc ECC 256-bit.
- Đường cong toy nhỏ và không phải đường cong chuẩn; kết quả chỉ phản ánh chi phí tìm kiếm vét cạn trên không gian đã chọn.
- Máy đo là laptop; tần số CPU có thể thay đổi do nhiệt độ và chính sách năng lượng. Warm-up, 10 lần đo và trung vị/IQR được dùng để giảm ảnh hưởng.
- Smoke test correctness dùng thư viện chuẩn; kết quả pass/fail không phải phép đo hiệu năng.
- Chưa tối ưu block size, số block hay cách ánh xạ ứng viên cho GPU; kết quả chỉ ứng với block 256.
- Kết quả correctness C1–C8 chỉ xác nhận thư viện hoạt động đúng, không phải benchmark hiệu năng.

Các nguồn [1]–[7] được liệt kê trong [`references.md`](../4.2-cpu-gpu-key-recovery/references.md).
