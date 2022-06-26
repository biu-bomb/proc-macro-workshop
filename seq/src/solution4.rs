impl crate::parser::SeqParser {
    pub(crate) fn process_prefix(
        &self,
        idx: &mut usize,
        n: usize,
        prefix: &syn::Ident,
        buf: &Vec<proc_macro2::TokenTree>,
    ) -> std::option::Option<proc_macro2::TokenStream> {
        if *idx + 2 < buf.len() {
            if let proc_macro2::TokenTree::Punct(p) = &buf[*idx + 1] {
                if p.as_char() == '~' {
                    if let proc_macro2::TokenTree::Ident(ident) = &buf[*idx + 2] {
                        if ident == &self.variable_ident
                            && prefix.span().end() == p.span().start()
                            && p.span().end() == ident.span().start()
                        {
                            let combine_ident_litral = format!("{}{}", prefix.to_string(), n);
                            let combine_ident =
                                syn::Ident::new(&combine_ident_litral, prefix.span());
                            *idx += 3;
                            return std::option::Option::Some(quote::quote! {
                                #combine_ident
                            });
                        }
                    }
                }
            }
        }
        std::option::Option::None
    }
}
