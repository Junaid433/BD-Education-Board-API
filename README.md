# Bangladesh Education Board Results API

A lightweight Rust service that wraps the official Bangladesh Education Board website to provide a clean JSON API for exam results. It handles the captcha automatically so you don't have to.

Built with **Rust** and **Axum**.

## Key Features

*   **Fast**: Written in Rust for minimal overhead.
*   **Auto-Captcha**: Automatically solves the math captcha on the official site.
*   **JSON Output**: Parses the HTML result sheet into a structured JSON response.
*   **Full Support**: Works for all education boards and exam types (SSC, HSC, JSC, Diploma, etc).

## Supported Exams & Boards

*   **Exams**: SSC/Dakhil, JSC/JDC, HSC/Alim, Vocational, BM, Diploma in Commerce/Business Studies.
*   **Boards**: Dhaka, Barisal, Chittagong, Comilla, Dinajpur, Jessore, Mymensingh, Rajshahi, Sylhet, Madrasah, Technical, DIBS.

## Usage

You can hit the API via `GET` or `POST`. Both do the same thing.

**Base URL**: `http://localhost:3000`

### 1. Fetch Result

**Endpoint**: `/fetch`

**Parameters:**

| Field | Description | Example |
| :--- | :--- | :--- |
| `exam` | Exam name | `ssc`, `hsc` |
| `year` | Exam year | `2024` |
| `board` | Board name | `dhaka` |
| `roll` | Student Roll | `123456` |
| `reg` | Registration No | `1234567890` |

### curl Example

```bash
curl -X POST http://localhost:3000/fetch \
  -H "Content-Type: application/json" \
  -d '{
    "exam": "ssc",
    "year": "2024",
    "board": "dhaka",
    "roll": "123456",
    "reg": "1234567890"
  }'
```

### Response

```json
{
  "name": "Student Name",
  "board": "Dhaka",
  "result": "Passed",
  "gpa": "5.00",
  "grades": [
    { "subject": "Bangla", "grade": "A+" },
    { "subject": "English", "grade": "A" }
  ]
}
```

## Setup & Run

You'll need [Rust](https://rustup.rs/) installed.

```bash
git clone https://github.com/Junaid433/eduboardapi.git
cd eduboardapi
cargo run --release
```

Server starts at `http://localhost:3000`. To change the port, just edit the `addr` bind in `src/main.rs`.

## Contributing

Feel free to open a PR if you want to add something or fix a bug. If it's a major change, open an issue first so we can chat about it.

## Disclaimer

This is an unofficial tool for educational purposes. It just scrapes data from the public education board website. Use it responsibly.

## Author

**Junaid Rahman**
[GitHub](https://github.com/Junaid433) | [Facebook](https://facebook.com/jnaid.rahman.im)