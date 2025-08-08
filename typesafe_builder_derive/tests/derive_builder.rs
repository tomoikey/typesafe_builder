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

#[test]
fn custom_builder_name_success() {
    #[derive(Builder, PartialEq)]
    #[builder(name = "MyCustomBuilder")]
    struct User {
        #[builder(required)]
        name: String,
    }
    let user = MyCustomBuilder::new()
        .with_name("Alice".to_string())
        .build();
    assert_eq!(user.name, "Alice");
}

#[test]
fn default_value_success() {
    #[derive(Builder, PartialEq)]
    struct Config {
        #[builder(default = "String::from(\"default_name\")")]
        name: String,
        #[builder(default = "42")]
        port: i32,
        #[builder(optional)]
        description: Option<String>,
    }

    let config = ConfigBuilder::new().build();
    assert_eq!(config.name, "default_name");
    assert_eq!(config.port, 42);
    assert_eq!(config.description, None);

    let config = ConfigBuilder::new()
        .with_name("custom".to_string())
        .with_port(8080)
        .with_description("test".to_string())
        .build();
    assert_eq!(config.name, "custom");
    assert_eq!(config.port, 8080);
    assert_eq!(config.description, Some("test".to_string()));
}

#[test]
fn complex_default_values() {
    #[derive(Builder, PartialEq)]
    struct ComplexConfig {
        #[builder(default = "vec![1, 2, 3]")]
        numbers: Vec<i32>,
        #[builder(default = "std::collections::HashMap::new()")]
        map: std::collections::HashMap<String, i32>,
        #[builder(required)]
        id: u32,
    }

    let config = ComplexConfigBuilder::new().with_id(1).build();
    assert_eq!(config.numbers, vec![1, 2, 3]);
    assert_eq!(config.map, std::collections::HashMap::new());
    assert_eq!(config.id, 1);
}

#[test]
fn into_value_success() {
    #[derive(Builder, PartialEq)]
    struct User {
        #[builder(required)]
        #[builder(into)]
        name: String,
        #[builder(optional)]
        #[builder(into)]
        address: Option<String>,
    }

    let user = UserBuilder::new()
        .with_name("Alice")
        .with_address("foo")
        .build();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.address, Some("foo".to_string()));
}

#[test]
fn bare_default_success() {
    #[derive(Builder, PartialEq)]
    struct Config {
        #[builder(default)]
        name: String,
        #[builder(default)]
        port: i32,
        #[builder(default)]
        enabled: bool,
    }

    let config = ConfigBuilder::new().build();
    assert_eq!(config.name, String::default());
    assert_eq!(config.port, i32::default());
    assert_eq!(config.enabled, bool::default());

    let config = ConfigBuilder::new()
        .with_name("custom".to_string())
        .with_port(8080)
        .with_enabled(true)
        .build();
    assert_eq!(config.name, "custom");
    assert_eq!(config.port, 8080);
    assert_eq!(config.enabled, true);
}

#[test]
fn mixed_default_and_expression_default() {
    #[derive(Builder, PartialEq)]
    struct Config {
        #[builder(default)]
        name: String,
        #[builder(default = "42")]
        port: i32,
        #[builder(default = "vec![1, 2, 3]")]
        numbers: Vec<i32>,
    }

    let config = ConfigBuilder::new().build();
    assert_eq!(config.name, String::default());
    assert_eq!(config.port, 42);
    assert_eq!(config.numbers, vec![1, 2, 3]);
}

#[test]
fn bare_default_with_custom_types() {
    #[derive(PartialEq, Default, Debug)]
    struct CustomType {
        value: i32,
    }

    #[derive(Builder, PartialEq)]
    struct Config {
        #[builder(default)]
        custom: CustomType,
    }

    let config = ConfigBuilder::new().build();
    assert_eq!(config.custom, CustomType::default());
}
