# SanitizeText

A minimalist web application built with the Dioxus framework for real-time input sanitization. This project focuses on providing a lightweight and fast solution for escaping HTML and JavaScript to prevent cross-site scripting (XSS) vulnerabilities in web applications.

[Use it here](https://benschneider.github.io/SanitizeText/)

## Features
- **Real-time Sanitization:** Input is sanitized as you type.
- **HTML/JS Escape Protection:** Prevents common XSS attacks by escaping malicious code.
- **Lightweight and Fast:** Built with Rust and Dioxus for performance.
- **Cross-browser Compatibility:** Works in modern web browsers.

## Getting Started

### Prerequisites
- Rust toolchain (install via [rustup](https://rustup.rs))
- Dioxus framework (automatically included as a dependency in `Cargo.toml`)
- A modern web browser

### Installation

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd SanitizeText
   ```

2. Build the project for release:
   ```bash
   dx serve --platform web --features web
   ```

### Running Locally

To run the application in development mode with hot-reloading:

```bash
dx serve
```

This will start a local development server, usually at `http://localhost:8080`.

To run the release build:

```bash
# First, build the project as shown in Installation step 2
./target/release/sanitizer-web
```

## Project Structure
- `src/main.rs`: The main application entry point.
- `src/lib.rs`: Contains the core sanitization logic and Dioxus components.
- `Cargo.toml`: Project dependencies and metadata.
- `assets/`: Static assets like fonts.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
[Specify your project's license here, e.g., MIT, Apache 2.0]
