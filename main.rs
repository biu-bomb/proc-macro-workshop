// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use derive_builder::Builder;

#[derive(Builder)]
struct Test {
    name: String,
    #[builder(each = "f")]
    family: Vec<String>,
    wife: Option<String>
}


fn main() {}
