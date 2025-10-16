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

synstructure::decl_derive! {
    [Walk] => derive_walk
}

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
