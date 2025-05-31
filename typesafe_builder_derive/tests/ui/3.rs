use typesafe_builder_derive::Builder;

struct Empty;
struct Filled;

fn main() {
    #[derive(Builder)]
    struct User {
        #[builder(optional)]
        name: Option<String>,
        #[builder(required_if = "!name")]
        age: Option<u8>,
    }

    // compile error because age is required if name is not present
    let user = UserBuilder::new().build();
} 