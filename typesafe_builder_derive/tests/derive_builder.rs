use typesafe_builder_derive::Builder;

struct Empty;

struct Filled;

#[test]
fn required_field_success() {
    #[derive(Builder, PartialEq)]
    struct User {
        #[builder(required)]
        name: String,
    }
    let user = UserBuilder::new().with_name("Alice".to_string()).build();
    assert_eq!(user.name, "Alice");
}

#[test]
fn optional_field_success() {
    #[derive(Builder, PartialEq)]
    struct User {
        #[builder(optional)]
        name: Option<String>,
    }
    let user = UserBuilder::new().build();
    assert_eq!(user.name, None);

    let user = UserBuilder::new().with_name("Alice".to_string()).build();
    assert_eq!(user.name, Some("Alice".to_string()));
}

#[test]
fn required_if_success() {
    #[derive(Builder, PartialEq)]
    struct Person {
        #[builder(optional)]
        email: Option<String>,
        #[builder(required_if = "email")]
        address: Option<String>,
    }
    // email is optional and address is required if email is present.
    // so this should be ok.
    let p1 = PersonBuilder::new().build();
    assert_eq!(p1.email, None);
    assert_eq!(p1.address, None);

    // email is present and address is required if email is present.
    // so this should be ok.
    let p2 = PersonBuilder::new()
        .with_email("a@b.com".to_string())
        .with_address("tokyo".to_string())
        .build();
    assert_eq!(p2.email, Some("a@b.com".to_string()));
    assert_eq!(p2.address, Some("tokyo".to_string()));
}
