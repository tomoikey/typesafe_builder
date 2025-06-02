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
        #[builder(optional)]
        address: Option<String>,
        #[builder(required_if = "name && (age || address)")]
        email: Option<String>,
    }

    // compile error because email is required if name and age are present
    let user = UserBuilder::new()
        .with_name("Alice".to_string())
        .with_age(20)
        .build();

    // compile error because email is required if name and address are present
    let user = UserBuilder::new()
        .with_name("Alice".to_string())
        .with_address("123 Main St".to_string())
        .build();
} 