use typesafe_builder_derive::Builder;

fn main() {
    #[derive(Builder)]
    struct User {
        #[builder(optional)]
        name: Option<String>,
        #[builder(required_if = "nonexistent_field")]
        age: Option<u8>,
    }
} 