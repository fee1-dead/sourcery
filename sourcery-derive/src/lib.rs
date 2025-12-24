use syn::{Lit, MetaNameValue, parse2};
use synstructure::BindStyle;

use quote::quote;

fn derive_walk(mut input: synstructure::Structure) -> proc_macro2::TokenStream {
    input.bind_with(|_| BindStyle::RefMut);
    let walk_variants = input.each(|binding| {
        quote! { ::sourcery::passes::Visit::visit(#binding, pass) }
    });

    input.gen_impl(quote! {
        gen impl ::sourcery::passes::Walk for @Self {
            fn walk<P: ::sourcery::passes::Pass + ?Sized>(&mut self, pass: &mut P) {
                match *self { #walk_variants }
            }
        }
    })
}

fn derive_respace(mut input: synstructure::Structure) -> proc_macro2::TokenStream {
    input.bind_with(|_| BindStyle::RefMut);


    let walk_variants = input.each(|binding| {
        if let Some(attr) = binding.ast().attrs.iter().find(|attr| attr.path().is_ident("sourcery")) {
            let list: MetaNameValue = parse2(attr.meta.require_list().unwrap().tokens.clone()).unwrap();
            assert!(list.path.is_ident("spaces"));
            
            let syn::Expr::Lit(syn::ExprLit { attrs: _, lit }) = list.value else {
                panic!("needs str literal as value")
            };

            match lit {
                Lit::Int(r) if r.base10_digits() == "1" => quote! { ::sourcery::passes::style::spaces::s1(#binding) },
                Lit::Int(r) if r.base10_digits() == "0" => quote! { ::sourcery::passes::style::spaces::s0(#binding) },
                Lit::Str(s) if s.value() == "ignore" => quote! {},
                other => panic!("unrecognized: {other:?}"),
            }
        } else {
            quote! { ::sourcery::passes::style::spaces::Respace::respace(#binding, pass) }
        }
    });
    input.gen_impl(quote! {
        gen impl ::sourcery::passes::style::spaces::Respace for @Self {
            fn respace(&mut self, pass: &mut ::sourcery::passes::style::spaces::Spaces) {
                match *self { #walk_variants }
            }
        }
    })
}

synstructure::decl_derive!([Walk] => derive_walk);
synstructure::decl_derive!([Respace, attributes(sourcery)] => derive_respace);


fn derive_print(input: synstructure::Structure) -> proc_macro2::TokenStream {
    let print_variants = input.each(|binding| {
        quote! { ::sourcery::Print::print(#binding, dest) }
    });

    input.gen_impl(quote! {
        gen impl ::sourcery::Print for @Self {
            fn print(&self, dest: &mut String) {
                match *self { #print_variants }
            }
        }
    })
}

synstructure::decl_derive! {
    [Print] => derive_print
}
