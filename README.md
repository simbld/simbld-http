### Overview
---
***`Simbld-HTTP`*** is a comprehensive and modular Rust library designed to manage HTTP response codes. It provides an organized structure for standard and custom HTTP response codes, complete with explicit descriptions and tools for simplified manipulation.
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
 
| Family             | Description                                                                |
|--------------------|----------------------------------------------------------------------------|
| **1xx**            | Informational responses (e.g., `Continue`, `Processing`).                  |
| **2xx**            | Successful responses (e.g., `OK`, `Created`).                              |
| **3xx**            | Redirection responses (e.g., `Moved Permanently`, `Temporary Redirect`).   |
| **4xx**            | Client errors (e.g., `Bad Request`, `Unauthorized`).                       |
| **5xx**            | Server errors (e.g., `Internal Server Error`, `Service Unavailable`).      |
| **6xx**            | Server/service-specific operations (e.g., `Service Timeout`).              |
| **7xx**            | Crawler-related responses (e.g., `Rate Limited`, `Crawl Blocked`).         |

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

To generate and view the documentation locally:

```bash
cargo doc --no-deps --open
```
---
What does this command do?

**`Generate HTML Documentation`**: This explains that cargo doc generates HTML documentation using source code comments.

**`Open in Browser`**: This describes that the --open option of the command automatically opens the generated documentation in the default browser.

This is useful for exploring all public modules, enums, and functions with their descriptions and examples.

What happens when you run this command?

**`HTML Documentation is Generated`**: Rust processes the inline comments (///) in the source code and creates a detailed API reference in HTML format.

**`Documentation Opens Automatically`**: The --open flag opens the generated HTML documentation in your default web browser.

## Online Documentation:

For developers who prefer online access, we are working on hosting the documentation using either **`GitHub Pages`** or **`docs.rs`**.

# 🛠️ Usage

## Example: Retrieve a Crawler Code

```rust
use simbld_http::responses::ResponsesCrawlerCodes;

fn main() {
    let code = ResponsesCrawlerCodes::RateLimited;
    println!("Code: {}, Description: {}", code.to_u16(), code.description());
}
```

# **🤝 Contributing to Simbld-HTTP**

We welcome contributions! the goal here is to create an exhaustive library of HTTP code and their detailed description for our Rust projects.

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
