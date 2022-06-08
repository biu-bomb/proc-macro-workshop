use proc_macro::TokenStream;
use quote::format_ident;
use syn::DeriveInput;
use syn::Token;
use syn::parse_macro_input;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    solution1(&ast)
}

/// 直接创建返回一个简单的tokenStream即可 TokenStream::new
/// 为了后续方便，顺便做一些简单的解析
/// 1. ast 
/// 2. fields
/// 3. final_stream: 因为stream可以拼接，每个问题都单独分离，在此方法进行聚合
fn solution1(ast: &DeriveInput) -> TokenStream {
    let fields = parse_fields(ast);
    let mut final_stream = proc_macro2::TokenStream::new();

    // solution2:创建builder和默认构造
    let origin_ident = &ast.ident;
    let build_ident = &format_ident!("{}Builder", origin_ident);
    let solution2_stream = solution2(fields, build_ident, origin_ident);
    final_stream.extend(solution2_stream);

    // solution3：创建setter方法
    let solution3_stream = solution3(fields, build_ident);
    final_stream.extend(solution3_stream);

    // solution45: 创建builder方法
    let solution45_stream = solution45(fields, build_ident, origin_ident);
    final_stream.extend(solution45_stream);

    TokenStream::from(final_stream)
}

type FieldsType = syn::punctuated::Punctuated<syn::Field, Token!(,)>;

/// 如果有引用的情况下，返回引用会自动跟随状态
fn parse_fields(ast: &DeriveInput) -> &FieldsType {
    if let syn::Data::Struct(
        syn::DataStruct {
            fields: syn::Fields::Named(
                syn::FieldsNamed {
                    ref named,
                    ..
                }
            ),
            ..
        }
    ) = ast.data {
        return named
    }
    panic!("parse fields error")
}
/// 问题2主要实现一个builder方法
/// 但是提示推荐的是
/// 1. 生成一个build结构
/// 2. build创建默认的build结构体
fn solution2(fields: &FieldsType, build_ident: &proc_macro2::Ident, origin_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    quote::quote! {
        // 创建基础XBuilder
        pub struct #build_ident {
            // 循环字段定义
            #(
                pub #idents: std::option::Option<#types>
            ),*
        }
        
        // 实现builder方法
        impl #origin_ident {
            pub fn builder() -> #build_ident {
                // 定义默认结构
                #build_ident {
                    #(
                        #idents: std::option::Option::None
                    ),*
                }
            }
        }
    }
}

/// 第三问主要是为builder创建setter
fn solution3(fields: &FieldsType, build_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    quote::quote!{
        impl #build_ident {
            #(
                pub fn #idents(&mut self, #idents: #types) -> &mut Self {
                    self.#idents = std::option::Option::Some(#idents);
                    self
                }
            )*
        }
    }
}

/// 第四关主要是将XBuilder在此转化为X
/// 主要问题在于返回的结果是Result
/// 其中unwrap的条件使用判空做标准
/// 1. 循环判空
/// 2. 结构生成
fn solution45(fields: &FieldsType, build_ident: &proc_macro2::Ident, origin_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    quote::quote! {
        // 为XBuilder生成
        impl #build_ident {
            // build方法生成
            pub fn build(&mut self) -> std::result::Result<#origin_ident, std::boxed::Box<dyn std::error::Error>> {
                // 循环if检测
                #(
                    if self.#idents.is_none() {
                        let err = format!("field {} is missing", stringify!(#idents));
                        return std::result::Result::Err(err.into());
                    }
                )*
                // 结构生成
                let res = #origin_ident {
                    #(
                        #idents: self.#idents.clone().unwrap()
                    ),*
                };
                return std::result::Result::Ok(res);
            }
        }
    }
}