### Overview

---

**_`Simbld-HTTP`_** is a comprehensive and modular Rust library designed to manage HTTP response codes. It provides an organized structure for standard and custom HTTP response codes, complete with explicit descriptions and tools for simplified manipulation.
It supports both standard families (1xx to 5xx) and custom extensions (6xx and 7xx).

---

## **Key Features**

- **Organized HTTP response families**:
  - Standard families: `1xx`, `2xx`, `3xx`, `4xx`, `5xx`.
  - Custom extensions: `0`, `-x`, `-xx`, `-xxx`, `6xx`, `7xx`.
  - Explicit descriptions for each code.
- **Robust utilities**:

  - Helpers for paginated JSON responses and generic HTTP responses.
  - Conversion tools to retrieve numeric values (`u16`/`i16`) for each code.

- **Extensibility**: Easily add new families or helpers as needed.

---

## **Structure of Families**

| Family  | Description                                                              |
| ------- | ------------------------------------------------------------------------ |
| **1xx** | Informational responses (e.g., `Continue`, `Processing`).                |
| **2xx** | Successful responses (e.g., `OK`, `Created`).                            |
| **3xx** | Redirection responses (e.g., `Moved Permanently`, `Temporary Redirect`). |
| **4xx** | Client errors (e.g., `Bad Request`, `Unauthorized`).                     |
| **5xx** | Server errors (e.g., `Internal Server Error`, `Service Unavailable`).    |
| **6xx** | Server/service-specific operations (e.g., `Service Timeout`).            |
| **7xx** | Crawler-related responses (e.g., `Rate Limited`, `Crawl Blocked`).       |

---

# 🛠️ Installation

To use Simbld-HTTP in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
simbld-http = { git = "https://github.com/simbld/simbld-http.git" }
serde = { version = "1.0", features = ["derive"] }
actix-web = "4.0"
```

# 🎯 Documentation

### Local Documentation

Simbld-HTTP provides full API documentation that is auto-generated using Rust's built-in documentation tool, `cargo doc`.

The `cargo doc --no-deps --open` command:

- Generates HTML documentation from your source code comments (///)
- Creates a detailed API reference showing all public modules, enums and functions
- Automatically opens generated documentation in your default web browser
- Exclude dependency documentation with --no-deps option
  This is useful for exploring and reviewing your crate's full API documentation.

## Online Documentation:

For developers who prefer online access, we are working on hosting the documentation using either **`GitHub Pages`** or **`docs.rs`**.

# 🛠️ Usage

## Example: Retrieve a Crawler Code

```rust
use simbld-http::responses::ResponsesCrawlerCodes;

fn main() {
    let code = ResponsesCrawlerCodes::RateLimited;
    println!("Code: {}, Description: {}", code.to_u16(), code.description());
}
```

# **🤝 Contributing to Simbld-HTTP**

We welcome contributions! The goal is to create an exhaustive library of HTTP codes and their detailed descriptions for Rust projects.

## Fork this repository.

Clone your fork:

```
git clone git@github.com:your-username/simbld-http.git
```

Create a branch for your changes:

```
git checkout -b your-contribution
```

Test your changes:

```rust
cargo test
```

Open a pull request.
