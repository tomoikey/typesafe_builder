use typesafe_builder_derive::Builder;

struct _TypesafeBuilderEmpty;
struct _TypesafeBuilderFilled;

fn main() {
    #[derive(Builder)]
    struct User {
        #[builder(required)]
        id: u32,
        
        #[builder(optional_if = "id")]
        name: String, // Error: not Option<String>
    }
}