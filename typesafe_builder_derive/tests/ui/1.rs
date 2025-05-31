use typesafe_builder_derive::Builder;

struct Empty;
struct Filled;

fn main() {
    #[derive(Builder)]
    struct User {
        #[builder(required)]
        name: String,
    }

    let user = UserBuilder::new().build(); // compile error
}