use typesafe_builder::*;

#[derive(Builder)]
struct User {
    #[builder(required)]
    name: String,
    #[builder(optional)]
    age: Option<u8>,
}

#[test]
fn test_builder() {
    let user = UserBuilder::new()
        .with_name("Alice".to_string())
        .build();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, None);

    let user = UserBuilder::new()
        .with_name("Alice".to_string())
        .with_age(20)
        .build();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, Some(20));
}