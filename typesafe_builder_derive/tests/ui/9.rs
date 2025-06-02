use typesafe_builder_derive::Builder;

struct _TypesafeBuilderEmpty;
struct _TypesafeBuilderFilled;

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