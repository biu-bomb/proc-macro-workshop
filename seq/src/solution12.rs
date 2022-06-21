impl syn::parse::Parse for crate::parser::SeqParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let variable_ident = input.parse::<syn::Ident>()?;
        let _ = input.parse::<syn::Token!(in)>()?;
        let (begin, end) = crate::solution7::parse_range(&input)?;
        let body_buf;
        let _ = syn::braced!(body_buf in input);
        let body = body_buf.parse::<proc_macro2::TokenStream>()?;
        syn::Result::Ok(
            crate::parser::SeqParser {
                variable_ident,
                begin,
                end,
                body
            }
        )
    }
}