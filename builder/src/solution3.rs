pub(super) fn soultion(
    fields: &crate::common::FieldsType,
    builder_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let tys: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    quote::quote! {
        impl #builder_ident {
            #(
                pub fn #idents(&mut self, #idents: #tys) -> &mut Self {
                    self.#idents = std::option::Option::Some(#idents);
                    self
                }
            )*
        }
    }
}
