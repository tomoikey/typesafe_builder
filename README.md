<h2 align="center">Refined Type</h2>

<div align="center">
    <div>
        <img src="https://img.shields.io/crates/v/typesafe_builder.svg"/>
        <img src="https://img.shields.io/crates/d/typesafe_builder"/>
    </div>
    <i>Code More simply, More safely, for all Rustaceans.🦀</i>
    <br/>
    <div>
        <a href="https://github.com/tomoikey/typesafe_builder/stargazers">
            <img src="https://img.shields.io/github/stars/tomoikey/typesafe_builder" alt="Stars Badge"/>
        </a>
        <a href="https://github.com/tomoikey/typesafe_builder/network/members">
            <img src="https://img.shields.io/github/forks/tomoikey/typesafe_builder" alt="Forks Badge"/>
        </a>
    </div>
    <a href="https://github.com/tomoikey/typesafe_builder/pulls">
        <img src="https://img.shields.io/github/issues-pr/tomoikey/typesafe_builder" alt="Pull Requests Badge"/>
    </a>
    <a href="https://github.com/tomoikey/typesafe_builder/issues">
        <img src="https://img.shields.io/github/issues/tomoikey/typesafe_builder" alt="Issues Badge"/>
    </a>
    <a href="https://github.com/tomoikey/typesafe_builder/graphs/contributors">
        <img alt="GitHub contributors" src="https://img.shields.io/github/contributors/tomoikey/typesafe_builder?color=2b9348">
    </a>
    <a href="https://github.com/tomoikey/typesafe_builder/blob/main/LICENSE">
        <img src="https://img.shields.io/github/license/tomoikey/typesafe_builder?color=2b9348" alt="License Badge"/>
    </a>
</div>

---

A procedural macro for Rust that enables type-safe builder patterns. This crate enforces required, optional, and conditionally required fields at the type level, ensuring safe and flexible struct construction.

## Features
- Per-field control: required, optional, and conditionally required (`required_if`)
- Compile-time enforcement of builder state using type parameters
- Flexible conditional requirements with AND/OR/NOT expressions

## Installation
Add the following to your Cargo.toml:

```toml
[dependencies]
typesafe_builder = "0.1.0"
```

## Usage
### Required Field
```rust
use typesafe_builder::*;

#[derive(Builder)]
struct User {
    #[builder(required)]
    name: String,
}

let user = UserBuilder::new().with_name("Alice".to_string()).build();
```

### Optional Field
```rust
use typesafe_builder::*;

#[derive(Builder)]
struct User {
    #[builder(optional)]
    name: Option<String>,
}

let user = UserBuilder::new().build();
let user2 = UserBuilder::new().with_name("Alice".to_string()).build();
```

### Conditionally Required Field (`required_if`)
```rust
use typesafe_builder::*;

#[derive(Builder)]
struct User {
    #[builder(optional)]
    name: Option<String>,
    #[builder(required_if = "name")]
    age: Option<u8>,
}
```

```rust
// The following will not compile because age is required if name is Some:
let user = UserBuilder::new().with_name("Alice".to_string()).build();
```

#### Complex Conditional Expressions
```rust
use typesafe_builder::Builder;

#[derive(Builder)]
struct User {
    #[builder(optional)]
    name: Option<String>,
    #[builder(optional)]
    age: Option<u8>,
    #[builder(optional)]
    address: Option<String>,
    #[builder(required_if = "name && (age || address)")]
    email: Option<String>,
}
```

```rust
// The following will not compile

// because email is required if name and age are Some
let user1 = UserBuilder::new().with_name("Alice".to_string()).with_age(20).build();

// because email is required if name and address are Some
let user2 = UserBuilder::new().with_name("Alice".to_string()).with_address("123 Main St".to_string()).build();
```

## License
MIT
