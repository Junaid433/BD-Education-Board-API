# Bangladesh Education Board Results API

A lightweight Rust service that wraps the official Bangladesh Education Board website and returns structured JSON result data. It handles the captcha automatically.

Built with **Rust** and **Axum**.

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/Junaid433/eduboardapi&project-name=eduboardapi&repository-name=eduboardapi)

## Key Features

- **Fast**: Written in Rust for minimal overhead.
- **Auto-Captcha**: Automatically solves the math captcha on the official site.
- **JSON Output**: Parses the HTML result sheet into a structured JSON response.
- **Full Support**: Works for all education boards and exam types (SSC, HSC, JSC, Diploma, etc).
- **Server + Library**: Can run as an API server and can be embedded directly from Rust code.

## Supported Exams & Boards

- **Exams**: SSC/Dakhil, JSC/JDC, HSC/Alim, Vocational, BM, Diploma in Commerce/Business Studies.
- **Boards**: Dhaka, Barisal, Chittagong, Comilla, Dinajpur, Jessore, Mymensingh, Rajshahi, Sylhet, Madrasah, Technical, DIBS.

## API Usage

You can call the endpoint via `GET` or `POST`.

- **Local Base URL**: `http://localhost:3000`
- **Vercel Base URL**: `https://<your-project>.vercel.app`

### Fetch Result

- **Endpoint**: `/fetch`

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

## Run Locally

You need [Rust](https://rustup.rs/).

```bash
git clone https://github.com/Junaid433/eduboardapi.git
cd eduboardapi
cargo run --release
```

The server starts at `http://localhost:3000`. You can override the port with:

```bash
PORT=8080 cargo run --release
```

## Deploy on Vercel

### One-click deploy

Use the button at the top of this README.

### Manual deploy

```bash
npm i -g vercel
vercel
```

This project includes:

- `api/axum.rs` for Vercel Rust function entrypoint.
- `vercel.json` rewrite so `/fetch` works directly on your Vercel domain.

## Use as a Rust Library

You can use this crate directly in another Rust app:

```toml
[dependencies]
eduboardapi = { package = "EduBoardAPI", git = "https://github.com/Junaid433/eduboardapi" }
tokio = { version = "1", features = ["full"] }
```

```rust
use eduboardapi::config::AppConfig;
use eduboardapi::models::RequestData;
use eduboardapi::services::{fetch_result, HttpClient};

#[tokio::main]
async fn main() {
    let client = HttpClient::new(AppConfig::default());

    let req = RequestData {
        exam: "ssc".to_string(),
        year: "2024".to_string(),
        board: "dhaka".to_string(),
        roll: "123456".to_string(),
        reg: "1234567890".to_string(),
    };

    let result = fetch_result(&client, &req).await.unwrap();
    println!("{:#?}", result);
}
```

Or reuse the ready-made Axum router:

```rust
let app = eduboardapi::build_app();
```

## Contributing

Open a PR for fixes/improvements. For major changes, open an issue first.

## Disclaimer

This is an unofficial educational tool that scrapes public result data. Use responsibly.

## Author

**Junaid Rahman**  
[GitHub](https://github.com/Junaid433) | [Facebook](https://facebook.com/jnaid.rahman.im)
