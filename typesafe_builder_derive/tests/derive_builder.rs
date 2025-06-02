use typesafe_builder_derive::Builder;

struct _TypesafeBuilderEmpty;

struct _TypesafeBuilderFilled;

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

#[test]
fn optional_if_success() {
    #[derive(Builder, PartialEq)]
    struct Config {
        #[builder(optional)]
        enable_feature: Option<bool>,
        #[builder(optional_if = "enable_feature")]
        feature_config: Option<String>,
    }

    let config1 = ConfigBuilder::new().with_enable_feature(true).build();
    assert_eq!(config1.enable_feature, Some(true));
    assert_eq!(config1.feature_config, None);

    let config2 = ConfigBuilder::new()
        .with_enable_feature(true)
        .with_feature_config("custom".to_string())
        .build();
    assert_eq!(config2.enable_feature, Some(true));
    assert_eq!(config2.feature_config, Some("custom".to_string()));

    let config3 = ConfigBuilder::new()
        .with_enable_feature(false)
        .with_feature_config("required".to_string())
        .build();
    assert_eq!(config3.enable_feature, Some(false));
    assert_eq!(config3.feature_config, Some("required".to_string()));
}

#[test]
fn conditional_behavior_verification() {
    #[derive(Builder, PartialEq)]
    struct Config {
        #[builder(optional)]
        feature_enabled: Option<bool>,
        #[builder(required_if = "feature_enabled")]
        feature_config: Option<String>,
    }

    let config1 = ConfigBuilder::new().build();
    assert_eq!(config1.feature_enabled, None);
    assert_eq!(config1.feature_config, None);

    let config2 = ConfigBuilder::new()
        .with_feature_enabled(true)
        .with_feature_config("advanced".to_string())
        .build();
    assert_eq!(config2.feature_enabled, Some(true));
    assert_eq!(config2.feature_config, Some("advanced".to_string()));
}

#[test]
fn optional_if_behavior_verification() {
    #[derive(Builder, PartialEq)]
    struct AppConfig {
        #[builder(optional)]
        debug: Option<bool>,
        #[builder(optional_if = "debug")]
        debug_file: Option<String>,
    }

    let config1 = AppConfigBuilder::new()
        .with_debug_file("debug.log".to_string())
        .build();
    assert_eq!(config1.debug, None);
    assert_eq!(config1.debug_file, Some("debug.log".to_string()));

    let config2 = AppConfigBuilder::new().with_debug(true).build();
    assert_eq!(config2.debug, Some(true));
    assert_eq!(config2.debug_file, None);
}

#[test]
fn lifetime_struct_success() {
    #[derive(Builder, PartialEq)]
    struct Config<'a> {
        #[builder(required)]
        name: &'a str,
    }

    let config = ConfigBuilder::new().with_name("Alice").build();
    assert_eq!(config.name, "Alice");
}

#[test]
fn generic_struct_success() {
    #[derive(Builder, PartialEq)]
    struct Container<T> {
        #[builder(required)]
        value: T,
    }

    let container = ContainerBuilder::new().with_value(42i32).build();
    assert_eq!(container.value, 42);
}

#[test]
fn lifetime_and_generic_struct_success() {
    #[derive(Builder, PartialEq)]
    struct ComplexConfig<'a, T> {
        #[builder(required)]
        name: &'a str,
        #[builder(optional)]
        value: Option<T>,
    }

    let config1: ComplexConfig<'_, i32> = ComplexConfigBuilder::new().with_name("test").build();
    assert_eq!(config1.name, "test");
    assert_eq!(config1.value, None);

    let config2 = ComplexConfigBuilder::new()
        .with_name("test")
        .with_value(100i32)
        .build();
    assert_eq!(config2.name, "test");
    assert_eq!(config2.value, Some(100));
}

#[test]
fn generic_struct_where_clause_success() {
    #[derive(Builder)]
    struct Container<T>
    where
        T: Clone,
    {
        #[builder(required)]
        value: T,
    }

    let container = ContainerBuilder::new().with_value(42i32).build();
    assert_eq!(container.value, 42);

    let container = ContainerBuilder::new()
        .with_value("hello".to_string())
        .build();
    assert_eq!(container.value, "hello");
}
