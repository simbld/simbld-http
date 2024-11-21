### 🌟 Simbld-HTTP

---

**_`Simbld-HTTP`_** is a modular and comprehensive Rust library designed for managing HTTP response codes. Whether you're building APIs, handling custom response codes, or integrating middleware, Simbld-HTTP provides an organized and extensible framework to simplify your workflow.

---

### **✨ Key Features**

**Support for Standard and Custom HTTP Codes:**

_Handles all standard HTTP response families: 1xx to 5xx.
Extends with custom families: 6xx, 7xx, and 9xx.
Includes detailed descriptions for every status code._

**Extensive Utilities:**

_JSON and XML response formatting.
Helpers for paginated and generic HTTP responses.
Middleware integration for Actix Web._

**Easy Extensibility:**

_Add new families or custom helpers with minimal effort._

---

### 🚀 Why Choose Simbld-HTTP?

**Developer-Friendly:** _Intuitive API with detailed documentation._
**Modular Design:** _Use only the parts you need for your project._
**Future-Proof:** _Easily extend to accommodate evolving HTTP standards and custom needs._
**Battle-Tested:** _Includes robust tests to ensure reliability._

---

### 📦 Installation

` Add Simbld-HTTP to your ``Cargo.toml `:

```toml
[dependencies]
simbld-http = "0.1.0"

```

---

### 📚 Documentation

##### Local Documentation

Run the following command to generate and open the documentation:

```bash
cargo doc --no-deps --open
```

###### This will

- _Generate detailed API documentation from inline comments (///)._
- _Display all public modules, enums, and methods._

##### Online Documentation

The full documentation will be available on **docs.rs** after publishing. Stay tuned for updates!

---

### 🔧 Usage Examples

##### Basic Usage

```bash
use simbld_http::responses::ResponsesTypes;

let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
println!("Code: {}, Description: {}", response.to_u16(), response.description());
```

##### Retrieve a Crawler Code

```bash
use simbld_http::responses::ResponsesCrawlerCodes;
use strum::EnumProperty;

fn main() {
    let code = ResponsesCrawlerCodes::ParsingErrorHeader;
    println!(
        "Code: {}, Description: {}",
        code.to_u16(),
        code.get_str("Description").unwrap()
    );
}

```

##### Run the example with

```bash
cargo run --example usage
```

##### Using the Middleware

```bash
cargo run --example middleware_usage
```

##### Test the middleware response with _**curl**_

```bash
curl -i http://127.0.0.1:8080/
```

---

### **⚙️ Structure of Families**

| Family  | Description                                                             |
| ------- | ----------------------------------------------------------------------- |
| **1xx** | Informational responses (e.g., `Continue`, `Processing`)                |
| **2xx** | Successful responses (e.g., `OK`, `Created`)                            |
| **3xx** | Redirection responses (e.g., `Moved Permanently`, `Temporary Redirect`) |
| **4xx** | Client errors (e.g., `Bad Request`, `Unauthorized`)                     |
| **5xx** | Server errors (e.g., `Internal Server Error`, `Service Unavailable`)    |
| **6xx** | Service operations (e.g., `Service Timeout`)                            |
| **7xx** | Crawler responses (e.g., `Rate Limited`, `Crawl Blocked`)               |
| **9xx** | Local API errors (e.g., `InsufficientFunds`, `ExpiredCard`)             |

---

### **🤝 Contributing to Simbld-HTTP**

We welcome contributions to Simbld-HTTP! Help us make this library the go-to solution for HTTP response code management in Rust.

###### `1` Fork this repository

###### `2` Clone your fork

```bash
git clone git@github.com:<your-username>/simbld-http.git

```

###### `3` Create a branch for your changes

```bash
git switch -c feature/<your-feature-name>


```

###### `4` Test your changes

```bash
cargo test

```

###### `5` Open a pull request

---

### 📜 License

This project is licensed under the **MIT** License. See the **LICENSE** file for details.

---

<p align="center">
  🛠️  
  <a href="https://crates.io/crates/simbld-http">
    <img src="https://img.shields.io/crates/v/simbld-http.svg" alt="Crates.io">
  </a>
  <a href="https://docs.rs/simbld-http">
    <img src="https://docs.rs/simbld-http/badge.svg" alt="Docs.rs">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT">
  </a>
</p>
