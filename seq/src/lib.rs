mod parser;
mod solution12;
mod solution3;
mod solution4;
mod solution56;
mod solution7;


#[proc_macro]
pub fn seq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = syn::parse_macro_input!(input as crate::parser::SeqParser);
    if let std::option::Option::Some(repeat_section_stream) = parser.expand_section() {
        return repeat_section_stream.into();
    }
    return parser.expend_repeat().into();
}

