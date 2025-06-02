use typesafe_builder_derive::Builder;

struct _TypesafeBuilderEmpty;
struct _TypesafeBuilderFilled;

fn main() {
    #[derive(Builder)]
    struct User {
        #[builder(optional)]
        name: Option<String>,
        #[builder(optional)]
        age: Option<u8>,
        #[builder(required_if = "name && age")]
        address: Option<String>,
    }

    // compile error because address is required if name and age are present
    let user = UserBuilder::new()
        .with_name("Alice".to_string())
        .with_age(20)
        .build();
} 