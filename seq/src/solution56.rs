impl crate::parser::SeqParser {
    pub(crate) fn expand_section(&self) -> std::option::Option<proc_macro2::TokenStream> {
        let buffer = syn::buffer::TokenBuffer::new2(self.body.clone());
        let (expended, expend_section_stream) = self.do_expand_section(buffer.begin());
        if expended {
            return std::option::Option::Some(expend_section_stream);
        }
        std::option::Option::None
    }

    pub(crate) fn do_expand_section(
        &self,
        origin_cursor: syn::buffer::Cursor,
    ) -> (bool, proc_macro2::TokenStream) {
        let mut found = false;
        let mut res = proc_macro2::TokenStream::new();
        let mut cursor = origin_cursor;
        while !cursor.eof() {
            if let Some((prefix, prefix_next_cursor)) = cursor.punct() {
                if prefix.as_char() == '#' {
                    if let Some((group_cursor, _, group_next_cursor)) =
                        prefix_next_cursor.group(proc_macro2::Delimiter::Parenthesis)
                    {
                        if let Some((suffix, suffix_next_cursor)) = group_next_cursor.punct() {
                            if suffix.as_char() == '*' {
                                for i in self.begin..self.end {
                                    let t = self.do_expand_repeat(&group_cursor.token_stream(), i);
                                    res.extend(t);
                                }
                                cursor = suffix_next_cursor;
                                found = true;
                                continue;
                            }
                        }
                    }
                }
            }
            if let Some((group_cursor, _, group_next_cursor)) =
                cursor.group(proc_macro2::Delimiter::Brace)
            {
                let (sub_found, sub_stream) = self.do_expand_section(group_cursor);
                found = sub_found;
                res.extend(quote::quote!({#sub_stream}));
                cursor = group_next_cursor;
                continue;
            } else if let Some((group_cursor, _, group_next_cursor)) =
                cursor.group(proc_macro2::Delimiter::Bracket)
            {
                let (sub_found, sub_stream) = self.do_expand_section(group_cursor);
                found = sub_found;
                res.extend(quote::quote!([#sub_stream]));
                cursor = group_next_cursor;
                continue;
            } else if let Some((group_cursor, _, group_next_cursor)) =
                cursor.group(proc_macro2::Delimiter::Parenthesis)
            {
                let (sub_found, sub_stream) = self.do_expand_section(group_cursor);
                found = sub_found;
                res.extend(quote::quote!((#sub_stream)));
                cursor = group_next_cursor;
                continue;
            } else if let Some((punct, next)) = cursor.punct() {
                res.extend(quote::quote!(#punct));
                cursor = next;
                continue;
            } else if let Some((ident, next)) = cursor.ident() {
                res.extend(quote::quote!(#ident));
                cursor = next;
                continue;
            } else if let Some((literal, next)) = cursor.literal() {
                res.extend(quote::quote!(#literal));
                cursor = next;
                continue;
            } else if let Some((lifetime, next)) = cursor.lifetime() {
                res.extend(quote::quote!(#lifetime));
                cursor = next;
                continue;
            }
        }
        (found, res)
    }
}
