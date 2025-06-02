use typesafe_builder_derive::Builder;

struct Empty;
struct Filled;

fn main() {
    #[derive(Builder)]
    struct User {
        #[builder(required)]
        id: u32,
        
        #[builder(optional_if = "id")]
        name: String, // Error: not Option<String>
    }
}