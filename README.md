# Bangladesh Education Board Results API

Fast REST API for fetching Bangladesh Education Board results. Built with Rust and Axum.

## What it does

Fetches education board results from Bangladesh. Works with SSC, HSC, JSC and other exams.

## Supported Stuff

**Exams:**
- SSC/Dakhil/Equivalent
- JSC/JDC
- SSC (Vocational)
- HSC/Alim
- HSC (Vocational)
- HSC (BM)
- Diploma in Commerce
- Diploma in Business Studies

**Boards:**
Barisal, Chittagong, Comilla, Dhaka, Dinajpur, Jessore, Mymensingh, Rajshahi, Sylhet, Madrasah, Technical, DIBS

## How to use

### API Call

```
GET /fetch
```

**Parameters:**
- `exam` - which exam (ssc, hsc, etc)
- `year` - exam year (1996-2025)
- `board` - board name
- `roll` - roll number (6 digits max)
- `reg` - registration number

### Example

```bash
curl "http://localhost:3000/fetch?exam=ssc&year=2024&board=dhaka&roll=123456&reg=1234567890"
```

**Response:**
```json
{
  "roll": "123456",
  "reg": "1234567890",
  "name": "Student Name",
  "father_name": "Father's Name",
  "mother_name": "Mother's Name",
  "board": "Dhaka",
  "group": "Science",
  "exam_type": "Regular",
  "dob": "01-01-2008",
  "institute": "School Name",
  "result": "Passed",
  "gpa": "5.00",
  "grades": [
    {
      "subject": "Bangla",
      "grade": "A+"
    },
    {
      "subject": "English",
      "grade": "A"
    }
  ]
}
```

## Setup

You need Rust installed. Get it from rustup.rs if you don't have it.

```bash
git clone https://github.com/Junaid433/eduboardapi.git
cd eduboardapi
cargo build --release
cargo run --release
```

Server runs on port 3000.

## Development

```bash
cargo run       
cargo test      
cargo clippy   
cargo fmt      
```

## Errors

- 200 = success
- 400 = bad params or captcha failed
- 502 = network/parsing error

## Contributing

PRs welcome. Open an issue first for big changes.

## Note

This is for educational use. Data belongs to Bangladesh Education Board. Use responsibly.

## Contact

Junaid Rahman - [facebook.com/jnaid.rahman.im](https://facebook.com/jnaid.rahman.im)

Repo: [github.com/Junaid433/eduboardapi](https://github.com/Junaid433/eduboardapi)
