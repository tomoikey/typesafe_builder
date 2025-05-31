# typesafe_builder

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
