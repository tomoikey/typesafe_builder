<div align="center">

# TypeSafe Builder

<div>
    <img src="https://img.shields.io/crates/v/typesafe_builder.svg" alt="crates.io"/>
    <img src="https://img.shields.io/crates/d/typesafe_builder" alt="downloads"/>
    <img src="https://img.shields.io/github/license/tomoikey/typesafe_builder" alt="license"/>
    <img src="https://img.shields.io/badge/rustc-1.87+-blue" alt="rustc"/>
</div>

<div>
    <a href="https://github.com/tomoikey/typesafe_builder/stargazers">
        <img src="https://img.shields.io/github/stars/tomoikey/typesafe_builder?style=social" alt="GitHub stars"/>
    </a>
    <a href="https://github.com/tomoikey/typesafe_builder/network/members">
        <img src="https://img.shields.io/github/forks/tomoikey/typesafe_builder?style=social" alt="GitHub forks"/>
    </a>
</div>

<h3>Compile-Time Type Safety • Zero Runtime Cost • Blazing Fast Builds</h3>

**The Ultimate Builder Pattern Implementation Powered by Rust's Type System**

<img width="550" src="https://github.com/user-attachments/assets/a72e996f-5f18-45ed-ab61-5f56bc04e8cc">

*Eliminate bugs at the type level and revolutionize your development experience*

---
</div>

## Why TypeSafe Builder?

Traditional builder patterns can't detect missing required fields until runtime.
**TypeSafe Builder** leverages Rust's powerful type system to verify all constraints **at compile time**.

```rust
// ❌ Traditional builder - potential runtime errors
let user = UserBuilder::new()
    .name("Alice")
    .build()?; // Compiles even with missing required fields

// ✅ TypeSafe Builder - compile-time safety guarantee
let user = UserBuilder::new()
    .with_name("Alice".to_string())
    .with_email("alice@example.com".to_string()) // Compile error if email is required
    .build(); // Always guaranteed to succeed
```

## Key Features

### Type-Level Constraint System
- **Required Fields** - Completely prevent missing required field configuration
- **Optional Fields** - Freely configurable fields
- **Default Values** - Fields with intelligent default values using any Rust expression
- **Conditional Requirements** - Express dynamic dependencies at the type level
- **Complex Logic** - Support for AND/OR/NOT operators in complex conditional expressions
- **Into Conversion** - Ergonomic setters with automatic type conversion via `Into<T>`

### Performance Characteristics
- **Zero Runtime Cost** - All validation completed at compile time

### Safety Guarantees
- **No Panic** - Complete elimination of runtime panics

## Quick Start

```toml
[dependencies]
typesafe_builder = "*.*.*" # Replace with the actual version
```

```rust
use typesafe_builder::*;

#[derive(Builder)]
struct User {
    #[builder(required)]
    name: String,
    #[builder(optional)]
    age: Option<u32>,
    #[builder(default = "String::from(\"user@example.com\")")]
    email: String,
}

// Type-safe builder pattern
let user = UserBuilder::new()
    .with_name("Alice".to_string())
    .with_age(30)
    .build(); // email will be "user@example.com"
```

## Advanced Features

### 1. Conditional Required Fields

```rust
use typesafe_builder::*;

#[derive(Builder)]
struct Account {
    #[builder(optional)]
    email: Option<String>,
    #[builder(required_if = "email")]  // Required when email is set
    email_verified: Option<bool>,
}

// ✅ Compiles successfully
let account1 = AccountBuilder::new().build();

// ✅ Compiles successfully
let account2 = AccountBuilder::new()
    .with_email("user@example.com".to_string())
    .with_email_verified(true)
    .build();

// ❌ Compile error: email_verified is not set
// let account3 = AccountBuilder::new()
//     .with_email("user@example.com".to_string())
//     .build();
```

### 2. Conditional Optional Fields

```rust
use typesafe_builder::*;

#[derive(Builder)]
struct Config {
    #[builder(optional)]
    debug_mode: Option<bool>,
    #[builder(optional_if = "debug_mode")]  // Required when debug_mode is not set
    log_level: Option<String>,
}

// ✅ When debug_mode is not set, log_level is required
let config1 = ConfigBuilder::new()
    .with_log_level("INFO".to_string())
    .build();

// ✅ When debug_mode is set, log_level is optional
let config2 = ConfigBuilder::new()
    .with_debug_mode(true)
    .build();
```

### 3. Complex Conditional Logic

```rust
use typesafe_builder::*;

#[derive(Builder)]
struct ApiClient {
    #[builder(optional)]
    use_auth: Option<bool>,
    #[builder(optional)]
    use_https: Option<bool>,
    #[builder(optional)]
    api_key: Option<String>,

    // Secret is required if using auth OR HTTPS
    #[builder(required_if = "use_auth || use_https")]
    secret: Option<String>,

    // Certificate is required only when using both auth AND HTTPS
    #[builder(required_if = "use_auth && use_https")]
    certificate: Option<String>,

    // Warning is required when using neither auth NOR HTTPS
    #[builder(required_if = "!use_auth && !use_https")]
    insecure_warning: Option<String>,

    // Complex condition: Token required when (auth OR HTTPS) AND (no API key)
    #[builder(required_if = "(use_auth || use_https) && !api_key")]
    fallback_token: Option<String>,
}

// ✅ All dependencies satisfied (auth + HTTPS)
let client1 = ApiClientBuilder::new()
    .with_use_auth(true)
    .with_use_https(true)
    .with_api_key("key123".to_string())
    .with_secret("secret456".to_string())
    .with_certificate("cert.pem".to_string())
    .build();

// ✅ Insecure configuration with warning
let client2 = ApiClientBuilder::new()
    .with_use_auth(false)
    .with_use_https(false)
    .with_insecure_warning("WARNING: Insecure connection!".to_string())
    .build();

// ✅ Using fallback token when API key is not set
let client3 = ApiClientBuilder::new()
    .with_use_auth(true)
    .with_secret("secret".to_string())
    .with_fallback_token("backup_token".to_string())
    .build();
```

### 4. Default Values

```rust
use typesafe_builder::*;

#[derive(Builder)]
struct ServerConfig {
    #[builder(default = "String::from(\"localhost\")")]
    host: String,

    #[builder(default = "8080")]
    port: u16,

    #[builder(default = "vec![\"GET\".to_string(), \"POST\".to_string()]")]
    allowed_methods: Vec<String>,

    #[builder(default = "std::collections::HashMap::new()")]
    headers: std::collections::HashMap<String, String>,

    #[builder(required)]
    service_name: String,

    #[builder(optional)]
    ssl_cert: Option<String>,
}

// ✅ Use default values
let config1 = ServerConfigBuilder::new()
    .with_service_name("my-api".to_string())
    .build();
// host: "localhost", port: 8080, allowed_methods: ["GET", "POST"], headers: {}

// ✅ Override some defaults
let config2 = ServerConfigBuilder::new()
    .with_service_name("my-api".to_string())
    .with_host("0.0.0.0".to_string())
    .with_port(3000)
    .build();
// host: "0.0.0.0", port: 3000, allowed_methods: ["GET", "POST"], headers: {}

// ✅ Complex default expressions
#[derive(Builder)]
struct AppConfig {
    #[builder(default = "std::env::var(\"APP_NAME\").unwrap_or_else(|_| \"default-app\".to_string())")]
    app_name: String,

    #[builder(default = "chrono::Utc::now()")]
    created_at: chrono::DateTime<chrono::Utc>,

    #[builder(default = "uuid::Uuid::new_v4()")]
    instance_id: uuid::Uuid,
}
```

Key features of default values:
- Flexible expressions: Use any valid Rust expression as default value
- No type restrictions: Works with primitives, collections, function calls, etc.
- Environment variables: Access environment variables at build time
- Function calls: Call any function or method as default value
- Standalone attribute: Cannot be combined with `required`, `optional`, etc.

### 5. Negation Operator Support

```rust
use typesafe_builder::*;

#[derive(Builder)]
struct Database {
    #[builder(optional)]
    use_ssl: Option<bool>,

    // Warning message required when NOT using SSL
    #[builder(required_if = "!use_ssl")]
    warning_message: Option<String>,
}

// ✅ Warning configuration for non-SSL usage
let db = DatabaseBuilder::new()
    .with_use_ssl(false)
    .with_warning_message("Insecure connection!".to_string())
    .build();
```

### 6. Into Conversion Support

The `#[builder(into)]` attribute allows setter methods to accept any type that implements `Into<T>` for the field type `T`, providing more ergonomic APIs:

```rust
use typesafe_builder::*;

#[derive(Builder)]
struct User {
    #[builder(required)]
    #[builder(into)]
    name: String,

    #[builder(optional)]
    #[builder(into)]
    email: Option<String>,
}

// ✅ Accept &str directly (converts to String via Into)
let user1 = UserBuilder::new()
    .with_name("Alice")  // &str -> String
    .with_email("alice@example.com")  // &str -> String
    .build();

// ✅ Still works with String directly
let user2 = UserBuilder::new()
    .with_name("Bob".to_string())
    .build();
```

Key benefits:
- Ergonomic APIs: Accept `&str` for `String` fields without manual conversion
- Type flexibility: Any `Into<T>` implementation works automatically
- Zero overhead: Conversion happens at the call site
- Backward compatible: Works alongside existing setter patterns

### 7. Custom Builder Name

```rust
use typesafe_builder::*;

#[derive(Builder)]
#[builder(name = "MyCustomBuilder")]  // Customize the builder name
struct User {
    #[builder(required)]
    name: String,
}

// Use the customized builder name
let user = MyCustomBuilder::new()
    .with_name("Alice".to_string())
    .build();
```

## Error Handling

### Compile-Time Error Examples

```rust
#[derive(Builder)]
struct User {
    #[builder(required)]
    name: String,
}

// ❌ Compile error
let user = UserBuilder::new().build();
//                           ^^^^^
// error: no method named `build` found for struct `UserBuilder<_TypesafeBuilderEmpty>`
//        method `build` is available on `UserBuilder<_TypesafeBuilderFilled>`
```

### Constraint Violation Error Examples

```rust
#[derive(Builder)]
struct Config {
    #[builder(optional)]
    feature: Option<bool>,
    #[builder(required_if = "feature")]
    config: Option<String>,
}

// ❌ Compile error
let config = ConfigBuilder::new()
    .with_feature(true)
    .build();
//   ^^^^^
// error: no method named `build` found for struct `ConfigBuilder<_TypesafeBuilderFilled, _TypesafeBuilderEmpty>`
//        method `build` is available on `ConfigBuilder<_TypesafeBuilderFilled, _TypesafeBuilderFilled>`
```

## Real-World Use Cases

### Web API Configuration

```rust
#[derive(Builder)]
struct ApiConfig {
    #[builder(required)]
    base_url: String,

    #[builder(optional)]
    use_auth: Option<bool>,

    #[builder(required_if = "use_auth")]
    api_key: Option<String>,

    #[builder(required_if = "use_auth")]
    secret: Option<String>,

    #[builder(default = "30")]
    timeout_seconds: u64,

    #[builder(default = "String::from(\"application/json\")")]
    content_type: String,
}
```

### Database Connection

```rust
#[derive(Builder)]
struct DatabaseConfig {
    #[builder(required)]
    host: String,

    #[builder(required)]
    database: String,

    #[builder(default = "5432")]
    port: u16,

    #[builder(default = "10")]
    max_connections: u32,

    #[builder(optional)]
    use_ssl: Option<bool>,

    #[builder(required_if = "use_ssl")]
    ssl_cert_path: Option<String>,

    #[builder(optional_if = "!use_ssl")]
    allow_insecure: Option<bool>,
}
```

## Contributing

We welcome contributions to TypeSafe Builder!

### Development Environment Setup

```bash
git clone https://github.com/tomoikey/typesafe_builder.git
cd typesafe_builder
cargo test
```

### Running Tests

```bash
# Run all tests
cargo test

# UI tests (compile error verification)
cargo test --package typesafe_builder_derive --test ui
```

## Contributors

Amazing developers who have contributed to this project:

<div align="center">

<table>
  <tr>
    <td align="center">
      <a href="https://github.com/tomoikey">
        <img src="https://github.com/tomoikey.png?size=100" width="100px;" alt="tomoikey"/>
        <br />
        <sub><b>tomoikey</b></sub>
        <br />
        <sub>Creator & Maintainer</sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/ramsyana">
        <img src="https://github.com/ramsyana.png?size=100" width="100px;" alt="ramsyana"/>
        <br />
        <sub><b>ramsyana</b></sub>
        <br />
        <sub>Contributor</sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/tomoikey/typesafe_builder/graphs/contributors">
        <img src="https://via.placeholder.com/100x100/f0f0f0/999999?text=%3F" width="100px;" alt="You?"/>
        <br />
        <sub><b>Your Name Here</b></sub>
        <br />
        <sub>Next Contributor</sub>
      </a>
    </td>
  </tr>
</table>

*Want to see your name here? [Contribute now](https://github.com/tomoikey/typesafe_builder/blob/main/CONTRIBUTING.md) and join our amazing community!*

[![Contributors](https://contrib.rocks/image?repo=tomoikey/typesafe_builder)](https://github.com/tomoikey/typesafe_builder/graphs/contributors)

</div>

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Give us a star!

If you find this project useful, please consider giving it a star!

---

<div align="center">
    <strong>Made with ❤️ by Rust community</strong>
    <br />
    <sub>Type safety is not a luxury, it's a necessity.</sub>
</div>
