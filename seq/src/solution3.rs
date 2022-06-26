impl crate::parser::SeqParser {
    pub(crate) fn expend_repeat(&self) -> proc_macro2::TokenStream {
        let mut res = proc_macro2::TokenStream::new();
        for i in self.begin..self.end {
            res.extend(self.do_expand_repeat(&self.body, i));
        }
        res
    }

    pub(crate) fn do_expand_repeat(
        &self,
        ts: &proc_macro2::TokenStream,
        n: usize,
    ) -> proc_macro2::TokenStream {
        let buf = &ts.clone().into_iter().collect::<Vec<_>>();
        let mut res = proc_macro2::TokenStream::new();
        let mut idx = 0usize;
        while idx < buf.len() {
            let node = &buf[idx];
            match node {
                proc_macro2::TokenTree::Group(group) => {
                    let recurrence_expand_stream = self.do_expand_repeat(&group.stream(), n);
                    let mut wrap_in_group_stream =
                        proc_macro2::Group::new(group.delimiter(), recurrence_expand_stream);
                    wrap_in_group_stream.set_span(group.clone().span());
                    res.extend(quote::quote!(#wrap_in_group_stream));
                }
                proc_macro2::TokenTree::Ident(ident) => {
                    if let std::option::Option::Some(token_stream) =
                        self.process_prefix(&mut idx, n, ident, buf)
                    {
                        res.extend(token_stream);
                        continue;
                    }
                    if ident == &self.variable_ident {
                        let new_ident = proc_macro2::Literal::i64_unsuffixed(n as i64);
                        res.extend(quote::quote!(#new_ident));
                    } else {
                        res.extend(quote::quote!(#node));
                    }
                }
                _ => {
                    res.extend(quote::quote!(#node));
                }
            }
            idx += 1;
        }
        res
    }
}
