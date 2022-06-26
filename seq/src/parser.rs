pub(crate) struct SeqParser {
    pub(crate) variable_ident: syn::Ident,
    pub(crate) begin: usize,
    pub(crate) end: usize,
    pub(crate) body: proc_macro2::TokenStream,
}
