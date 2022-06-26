pub(crate) fn to_token_stream(t: impl quote::ToTokens) -> proc_macro2::TokenStream {
    t.to_token_stream()
}

pub(crate) fn check_order(
    names: Vec<(String, &dyn quote::ToTokens)>,
) -> std::option::Option<syn::Error> {
    let origin_names = names;
    let mut sorted_names = origin_names.clone();
    sorted_names.sort_by(|a, b| a.0.cmp(&b.0));
    for (a, b) in origin_names.iter().zip(sorted_names.iter()) {
        if a.0 != b.0 {
            return std::option::Option::Some(syn::Error::new_spanned(
                b.1,
                format!("{} should sort before {}", b.0, a.0),
            ));
        }
    }
    std::option::Option::None
}

pub(crate) struct MatchVisitor {
    pub(crate) err: std::option::Option<syn::Error>,
}

impl syn::visit_mut::VisitMut for MatchVisitor {
    fn visit_expr_match_mut(&mut self, i: &mut syn::ExprMatch) {
        let mut target_idx: isize = -1;
        for (idx, attr) in i.attrs.iter().enumerate() {
            if path_to_string(&attr.path) == "sorted" {
                target_idx = idx as isize;
                break;
            }
        }
        if target_idx != -1 {
            i.attrs.remove(target_idx as usize);
            let mut match_arm_names: Vec<(String, &dyn quote::ToTokens)> = Vec::new();
            for arm in i.arms.iter() {
                match &arm.pat {
                    syn::Pat::Path(p) => {
                        match_arm_names.push((path_to_string(&p.path), &p.path));
                    }
                    syn::Pat::TupleStruct(p) => {
                        match_arm_names.push((path_to_string(&p.path), &p.path));
                    }
                    syn::Pat::Struct(p) => {
                        match_arm_names.push((path_to_string(&p.path), &p.path));
                    }
                    syn::Pat::Ident(p) => {
                        match_arm_names.push((p.ident.to_string(), &p.ident));
                    }
                    syn::Pat::Wild(p) => {
                        match_arm_names.push(("_".to_string(), &p.underscore_token));
                    }
                    _ => {
                        self.err = std::option::Option::Some(syn::Error::new_spanned(
                            &arm.pat,
                            "unsupported by #[sorted]",
                        ));
                        return;
                    }
                }
            }
            if let Some(e) = check_order(match_arm_names) {
                self.err = std::option::Option::Some(e);
                return;
            }
        }
        syn::visit_mut::visit_expr_match_mut(self, i)
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}
