# Bangladesh Education Board Results API

A fast, reliable REST API for fetching Bangladesh Education Board results. Built with Rust and Axum for high performance and reliability.

## Features

- 🚀 Fast and efficient result fetching
- 🔧 Built with Rust for performance and safety
- 📊 Support for multiple examination types (SSC, HSC, JSC, etc.)
- 🏫 All major education boards supported
- 🔄 Automatic captcha solving
- 📡 RESTful API with JSON responses
- 📝 Comprehensive logging and error handling

## Supported Examinations

- SSC/Dakhil/Equivalent
- JSC/JDC
- SSC (Vocational)
- HSC/Alim
- HSC (Vocational)
- HSC (BM)
- Diploma in Commerce
- Diploma in Business Studies

## Supported Boards

- Barisal
- Chittagong
- Comilla
- Dhaka
- Dinajpur
- Jessore
- Mymensingh
- Rajshahi
- Sylhet
- Madrasah
- Technical
- DIBS (Dhaka)

## API Usage

### Endpoint

```
GET /fetch
```

### Query Parameters

| Parameter | Type   | Required | Description                  |
|-----------|--------|----------|------------------------------|
| exam      | string | Yes      | Examination type (e.g., ssc) |
| year      | string | Yes      | Year (1996-2025)            |
| board     | string | Yes      | Board name                   |
| roll      | string | Yes      | Roll number (max 6 digits)   |
| reg       | string | Yes      | Registration number          |

### Example Request

```bash
curl "http://localhost:3000/fetch?exam=ssc&year=2024&board=dhaka&roll=123456&reg=1234567890"
```

### Example Response

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

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git

### Build from Source

```bash
# Clone the repository
git clone https://github.com/Junaid433/eduboardapi.git
cd eduboardapi

# Build the project
cargo build --release

# Run the server
cargo run --release
```

The API server will start on `http://localhost:3000`

## Configuration

The server runs on port 3000 by default. You can modify this in `src/main.rs`:

```rust
let addr: SocketAddr = "0.0.0.0:3000".parse()?;
```

## Development

```bash
# Run in development mode with hot reloading
cargo watch -x run

# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt
```

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK` - Successful request
- `400 Bad Request` - Invalid parameters or captcha error
- `502 Bad Gateway` - Network error or parsing error

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This API is for educational purposes only. The data belongs to the Bangladesh Education Board. Please use responsibly and in accordance with the board's terms of service.

## Acknowledgments

- Bangladesh Education Board for providing public access to results
- The Rust community for excellent libraries and tools

## Support

If you find this project helpful, consider:

- ⭐ Starring the repository
- 🐛 Reporting bugs
- 💡 Suggesting new features
- ☕ [Buy me a coffee](https://www.buymeacoffee.com/Junaid433)

## Contact

Junaid Rahman - [@Junaid Rahman](https://facebook.com/jnaid.rahman.im)

Project Link: [https://github.com/Junaid433/eduboardapi](https://github.com/Junaid433/eduboardapi)
