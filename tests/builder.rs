use typesafe_builder::*;

#[derive(Builder)]
struct User {
    #[builder(required)]
    #[builder(into)]
    name: String,
    #[builder(optional)]
    #[builder(into)]
    age: Option<u8>,
    #[builder(required_if = "age")]
    #[builder(into)]
    email: Option<String>,
    #[builder(optional_if = "age")]
    #[builder(into)]
    address: Option<String>,
}

#[test]
fn test_builder() {
    let user = UserBuilder::new()
        .with_name("Alice")
        .with_address("123 Main St")
        .build();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, None);
    assert_eq!(user.email, None);
    assert_eq!(user.address, Some("123 Main St".to_string()));

    let user = UserBuilder::new()
        .with_name("Alice")
        .with_age(20)
        .with_email("123 Main St")
        .build();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, Some(20));
    assert_eq!(user.email, Some("123 Main St".to_string()));
    assert_eq!(user.address, None);
}
