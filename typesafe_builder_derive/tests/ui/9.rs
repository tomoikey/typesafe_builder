use typesafe_builder_derive::Builder;

struct Empty;
struct Filled;

fn main() {
    #[derive(Builder)]
    struct User {
        #[builder(optional_if = "name", required_if = "age")]
        email: Option<String>,
        #[builder(optional)]
        name: Option<String>,
        #[builder(optional)]
        age: Option<u8>,
    }
} 