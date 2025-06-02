use typesafe_builder_derive::Builder;

struct Empty;
struct Filled;

fn main() {
    #[derive(Builder)]
    struct User {
        #[builder(optional)]
        name: String, // Error: not Option<String>
    }
}