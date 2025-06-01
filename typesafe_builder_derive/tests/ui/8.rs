use typesafe_builder_derive::Builder;

struct Empty;
struct Filled;

fn main() {
    #[derive(Builder)]
    struct Config {
        #[builder(optional)]
        enable_feature: Option<bool>,
        #[builder(optional_if = "enable_feature")]
        feature_config: Option<String>,
    }

    // compile error because feature_config is required when enable_feature is not set (None/false)
    let config = ConfigBuilder::new().build();
} 