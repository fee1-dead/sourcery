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
