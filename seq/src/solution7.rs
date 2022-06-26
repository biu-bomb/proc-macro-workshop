pub(crate) fn parse_range(input: &syn::parse::ParseStream) -> syn::Result<(usize, usize)> {
    let begin = input.parse::<syn::LitInt>()?.base10_parse()?;
    let _ = input.parse::<syn::Token!(..)>()?;
    let mut incude_grater = false;
    if input.peek(syn::Token!(=)) {
        input.parse::<syn::Token!(=)>()?;
        incude_grater = true;
    }
    let mut end = input.parse::<syn::LitInt>()?.base10_parse()?;
    if incude_grater {
        end += 1;
    }
    syn::Result::Ok((begin, end))
}
