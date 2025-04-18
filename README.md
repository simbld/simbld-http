### 🆕 Changelog (Version 1.0.0)

- **New Architecture: Macro System**

###### Introduction of a complete macro system for generating HTTP responses

`generate_response!`: _Central macro for dynamically creating customized HTTP responses._ `define_status!`: _Definition
of new status codes with their associated descriptions._ `response_builder!`: _Generation of fluid builders for
constructing complex responses._
_The macro system offers increased expressiveness and flexibility while significantly reducing verbosity._

- **Response Model Redesign**

###### Replacement of CustomResponse with the new HttpResponse

`HttpResponse<T>`: _Generic structure allowing for strongly-typed response bodies._ `ResponseBuilder`: _Fluid API for
progressive construction of HTTP responses._
_This redesign improves ergonomics and type safety while offering more flexibility._

- **Native Integration of Asynchronous Traits**

###### Complete support for modern Rust asynchronous patterns

`AsyncResponder`: _Trait allowing Futures to be returned as HTTP responses._ `StreamResponse`: _Support for streaming
response bodies for voluminous data._
_The asynchronous integration optimizes performance and reduces resource consumption._

### 📚 Updated Documentation and Examples

_Complete documentation and new examples are available in the project's GitHub
repository:_ ([https://github.com/simbld/simbld-http/tree/main/examples](https://github.com/simbld/simbld-http/tree/main/examples))

##### New Examples

- **Macro Usage**: _Demonstration of response generation via the new macros._
- **Fluid API**: _Examples of using the builder API to construct complex responses._
- **Asynchronous Patterns**: _Integration with Rust and Actix-Web asynchronous features._

---

### 🆕 Changelog (Version 0.4.1)

- **New Feature: Unified Response Functions**

###### Added functions returning `(u16, &'static str)` for all response families

`invalid_date_of_birth`, `invalid_majority`, `invalid_marital_status`, etc.:

_Each function returns a tuple containing a status code and a description._
_The complete list of http codes (official and unofficial) is available in the file:_
(<https://github.com/simbld/simbld-http/blob/main/THE_COMPLETE_LIST_OF_OFFICIAL_AND_UNOFFICIAL_HTTP_CODES.md>)

- **Improved Code Consistency**

###### Standardized response functions across all families

_All response families now include functions that return a tuple `(u16, &'static str)` for better consistency and ease
of use._

### 📚 Full Documentation and Examples

#### Complete examples, including detailed use cases and advanced integrations, are available in the project's GitHub repository

[https://github.com/simbld/simbld-http/tree/main/examples](https://github.com/simbld/simbld-http/tree/main/examples)  
[https://github.com/simbld/simbld-http/blob/main/src/responses](https://github.com/simbld/simbld-http/blob/main/src/responses)  
[https://github.com/simbld/simbld-http/tree/main/src/helpers](https://github.com/simbld/simbld-http/tree/main/src/helpers)

##### Included Examples And Tests

- **Http Interceptor**: _An_.
- **Middleware Integration**: _Demonstration of integration into an Actix Web server with custom middleware._
- **Response Helpers**: _Using helpers to create responses with status codes and descriptions._

- **Actix-Web Usage**: _How to integrate simbld-http with Actix-Web for handling responses and authentication._

- **Unit Testing**: _Demonstrates how to write unit tests using the new helpers._

---

### 🆕 Changelog (Version 0.3.1)

- **New Feature: Actix-Web Integration**

###### Added a `CustomResponse` compatible with Actix-Web and an authentication middleware

`custom_response_handler`: _Generates custom HTTP responses in an Actix-Web application._

`auth_middleware`: _Manages token validation via query parameters to secure routes._

- **Improved Testing Capabilities**

###### Added test files to enhance code coverage and reliability

`status_codes_test.rs`: _Contains tests to verify the generation of HTTP status codes._

`test_helpers.rs`: _Provides utility functions to capture test outputs._

- **New Helpers Features**

###### Added helpers to create responses with mock data

`mock_responses`: _Defines test responses to facilitate unit testing._

`response_functions`: _Introduces the `ResponseFunctions` trait with methods to generate response functions._

- **Miscellaneous Improvements**

###### Code optimizations and minor bug fixes

- **Updated Documentation**

###### Examples and documentation have been updated to reflect the new features

---

### 🆕 Changelog (Version 0.2.1)

- **New Feature: Cookie Management**

###### Added helpers to include cookies directly in responses

`ok_with_cookie`: _Generate an OK response with an attached cookie._
`bad_request_with_cookie`: _Generate a Bad Request response with an attached cookie._

- **New Feature: Dynamic Headers**

###### Added helpers to dynamically add custom headers in responses

`ok_with_headers`: _Generate an OK response with custom headers._
`bad_request_with_headers`: _Generate a Bad Request response with custom headers._

- **Middleware Improvement:**

###### The middleware now adds

`x-status-description`: _A description associated with the HTTP status code._
`x-response-time-ms`: _The request processing time in milliseconds._

### 📚 Complete Documentation and Examples

_Complete examples, including detailed use cases and advanced integrations, are available in the project's GitHub
repository_ (<https://github.com/simbld/simbld-http/tree/main/examples>).

##### Included Examples

- **Basic Usage**: _An introduction to using simbld-http_.

- **Middleware Integration**: _Demonstration of integration into an Actix Web server with custom middleware._
- **Response Helpers**: _Using helpers to create responses with cookies or custom headers._

---

### 🌟 Simbld-HTTP (v0.1.0)

---

**_`simbld-http`_** is a modular and comprehensive Rust library designed for managing HTTP response codes. Whether
you're building APIs, handling custom response codes, or integrating middleware, Simbld-HTTP provides an organized and
extensible framework to simplify your workflow.

---

### ✨ Key Features

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

` Add simbld-http to your ``Cargo.toml `:

```toml
[dependencies]
simbld-http = "0.2.0"

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
println!("Code: {}, Description: {}", response.get_code(), response.get_str("Description"));
```

##### Retrieve a Crawler Code

```bash
use simbld_http::responses::ResponsesCrawlerCodes;
use strum::EnumProperty;

fn main() {
    let code = ResponsesCrawlerCodes::ParsingErrorHeader;
    println!(
        "Code: {}, Description: {}",
        code.get_code(),
        code.get_str("Description").unwrap()
    );
}

```

##### Run the example with

```bash
cargo run --example examples_usage
```

##### Using the Middleware

```bash
cargo run --example middleware_advanced_usage
```

##### Test the middleware response with **_curl_**

```bash
curl -i http://127.0.0.1:8080/
```

---

### ⚙️ Structure of Families

| Family  | Description                                                             |
|---------|-------------------------------------------------------------------------|
| **1xx** | Informational responses (e.g., `ContinueRequest`, `Processing`)         |
| **2xx** | Success responses (e.g., `OK`, `Created`)                               |
| **3xx** | Redirection responses (e.g., `Moved Permanently`, `Temporary Redirect`) |
| **4xx** | Client errors (e.g., `Bad Request`, `Unauthorized`)                     |
| **5xx** | Server errors (e.g., `Internal Server Error`, `Service Unavailable`)    |
| **6xx** | Service operations (e.g., `Service Timeout`)                            |
| **7xx** | Crawler responses (e.g., `Rate Limited`, `Crawl Blocked`)               |
| **9xx** | Local API errors (e.g., `InsufficientFunds`, `ExpiredCard`)             |

---

### 🤝 Contributing to Simbld-HTTP

We welcome contributions to Simbld-HTTP! Help us make this library the go-to solution for HTTP response code management
in Rust.

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

<p>
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
