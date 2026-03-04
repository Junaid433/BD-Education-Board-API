# Bangladesh Education Board Results API

Rust API for Bangladesh Education Board result lookups, deployed as a Vercel serverless function.

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/Junaid433/BD-Education-Board-API&project-name=eduboardapi&repository-name=BD-Education-Board-API)

## Overview

- Runtime: Rust + Axum
- Deployment target: Vercel
- API style: JSON over HTTP (`GET` and `POST`)
- Hosted endpoint pattern: `https://<your-project>.vercel.app/fetch`

## API Contract

### Endpoint

- `POST /fetch`
- `GET /fetch` (query parameters)

### Required Parameters

| Field | Description | Example |
| :--- | :--- | :--- |
| `exam` | Exam code | `ssc`, `hsc` |
| `year` | Exam year | `2024` |
| `board` | Board code | `dhaka`, `dinajpur` |
| `roll` | Student roll number | `277794` |
| `reg` | Registration number | `2117829468` |

### Example Request (Production)

```bash
curl -sS -X POST "https://eduboardapi.vercel.app/fetch" \
  -H "Content-Type: application/json" \
  -d '{
    "exam": "ssc",
    "year": "2024",
    "board": "dinajpur",
    "roll": "277795",
    "reg": "2117829469"
  }'
```

### Example Response

```json
{
  "roll": "277795",
  "reg": "2117829469",
  "name": "Student Name",
  "father_name": "Father Name",
  "mother_name": "Mother Name",
  "board": "Dinajpur",
  "group": "Science",
  "exam_type": "SSC",
  "dob": "01-Jan-2008",
  "institute": "Institute Name",
  "result": "Passed",
  "gpa": "5.00",
  "grades": [
    { "code": "101", "subject": "Bangla", "grade": "A+" }
  ]
}
```

## Deploy to Vercel

### One-Click

Use the button at the top of this README.

### CLI

```bash
vercel
vercel --prod
```

## Vercel Runtime Layout

- `api/axum.rs`: Vercel function entrypoint for the Axum router
- `vercel.json`: Rewrite `/fetch` to `/api/axum` while serving landing page files at `/`
- `index.html`, `styles.css`, `script.js`: Static landing page and quick API playground
- `src/lib.rs`: Shared application builder (`build_app`) used by the Vercel function and Rust consumers

## Use From Rust

### 1. Call Result Service Directly

```toml
[dependencies]
eduboardapi = { package = "EduBoardAPI", git = "https://github.com/Junaid433/BD-Education-Board-API" }
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
        board: "dinajpur".to_string(),
        roll: "277794".to_string(),
        reg: "2117829468".to_string(),
    };

    let result = fetch_result(&client, &req).await.unwrap();
    println!("{:#?}", result);
}
```

### 2. Reuse Router in a Rust Service

```rust
let app = eduboardapi::build_app();
```

## License

MIT
