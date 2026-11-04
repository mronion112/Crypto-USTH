# Report Outline and Presentation Guide

Tài liệu này là khung để ghép các bản nháp thành báo cáo cuối. Cách trình bày tham khảo `E_Voting_Report.pdf`: trang bìa, mục lục, đề mục đánh số, công thức/sơ đồ/bảng có chú thích, phân tích sau kết quả và tài liệu tham khảo đánh số. Chỉ kế thừa cách tổ chức; không sao chép nội dung, kết quả hay kết luận từ báo cáo mẫu.

## Front matter

### Cover page

Trình bày cân đối, căn giữa như mẫu. Thay các dòng trong ngoặc vuông theo thông tin chính thức của lớp:

```text
[Tên trường]
[Khoa/Bộ môn]

[Tên học phần]
[Giảng viên]

DIGITAL SIGNATURE ALGORITHMS AND THEIR RESISTANCE TO ATTACKS IN BANKING
Group [số nhóm]

Team members
[Họ tên] – [Mã sinh viên]
...

[Địa điểm], [ngày/tháng/năm]
```

Tạo Table of Contents tự động sau trang bìa khi xuất bản PDF; cập nhật số trang sau khi ghép xong.

## Main body

### 1 Introduction

#### 1.1 Motivation

#### 1.2 Purpose of the Project

### 2 Cryptographic Algorithms

Mỗi thuật toán có cùng hai tiêu đề con để dễ đối chiếu:

#### 2.1 Ed25519

- **2.1.1 Mathematical Theory** — nguyên lý, quy trình ký/xác minh, giả định an toàn và tấn công.
- **2.1.2 Application in Banking System** — vai trò, triển khai, ưu điểm và giới hạn.

#### 2.2 SM2

- **2.2.1 Mathematical Theory**
- **2.2.2 Application in Banking System**

#### 2.3 RSA-2048 and RSA-4096 (RSA-PSS)

- **2.3.1 Mathematical Theory**
- **2.3.2 Application in Banking System**

#### 2.4 ECDSA (chọn secp256k1 hoặc NIST P-256)

- **2.4.1 Mathematical Theory**
- **2.4.2 Application in Banking System**

#### 2.5 Post-Quantum Cryptography: ML-DSA

- **2.5.1 Mathematical Theory**
- **2.5.2 Application in Banking System**

### 3 Mechanism of Operation

#### 3.1 Cryptographic Components and Security Roles

#### 3.2 System Workflow Phases

Đặt sơ đồ tổng quan gần phần giới thiệu workflow; sau đó mô tả từng phase bằng văn bản, công thức cần thiết và các bước kiểm soát.

### 4 Testing and Performance Analysis

#### 4.1 Test Cases and Experimental Setup

#### 4.2 CPU/GPU Key-Recovery Benchmark

#### 4.3 Quantum Computer — Theoretical Analysis

Đặt bảng test/benchmark gần đoạn mô tả; tiếp theo bảng là diễn giải kết quả, yếu tố bất thường và giới hạn đo.

### 5 Conclusion & Future Work

#### 5.1 What the Analysis Shows

#### 5.2 Limitations and Open Questions

#### 5.3 Future Directions

### 6 References

Đánh số theo kiểu IEEE trong nội dung, ví dụ `[1]`; bảo đảm mọi trích dẫn có mục tương ứng trong danh sách cuối.

## Figures, tables, and equations

- Đánh số và nhắc tới từng hình/bảng/công thức trong nội dung. Hình có caption bên dưới; bảng có tiêu đề bên trên.
- Viết đơn vị trong tiêu đề cột hoặc caption; giải thích chữ viết tắt và tham số ngay tại lần xuất hiện đầu tiên.
- Với phương trình, định nghĩa ký hiệu và nói rõ phương trình biểu diễn bước nào trong thuật toán/workflow.
- Dùng cùng kiểu chữ, ký hiệu, cách đặt caption và đánh số xuyên suốt báo cáo.

## Writing style

- Mẫu tham khảo viết bằng tiếng Anh; nên dùng tiếng Anh nhất quán trong bản cuối nếu phù hợp yêu cầu học phần. GUIDE trong repo viết tiếng Việt để hỗ trợ phân công.
- Mở mỗi mục bằng ý chính; dùng tiểu mục, bảng, sơ đồ và phương trình khi chúng giúp giải thích cụ thể.
- Đặt nguồn sát với nhận định được hỗ trợ. Viết diễn giải của nhóm, tránh bê nguyên câu từ nguồn hoặc mẫu.
- Tách dữ liệu/quan sát, phân tích và kết luận; công khai hạn chế và kết quả chưa đạt.

## Appendices (optional)

Đưa mã dài, lệnh chạy, bảng bổ sung hoặc chi tiết cấu hình vào phụ lục nếu làm gián đoạn nội dung chính. Phần thân vẫn phải tóm tắt phương pháp và liên kết rõ tới phụ lục hoặc tệp trong repo.
