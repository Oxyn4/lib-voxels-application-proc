extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;

use proc_macro2::{Ident, Span};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let block = &input.block;

    let mut new_sig = sig.clone();

    new_sig.ident = Ident::new("__voxels_wrapped_main", Span::call_site());

    let user_main_fn = quote! {
        #(#attrs)*
        #vis fn #new_sig
            #block
    };

    // Rewrite the function: we generate a real main that loads config and passes it to the user's main
    let wrapper_main = quote! {
        fn main() {
            let config_str = std::fs::read_to_string("voxels.toml")
                .expect("Could not read voxels.toml");

            let config: crate::lib_voxels_application_proc::lib_voxels_application_core::application::Application
                .expect("Failed to parse voxels.toml");
            voxels_application_core::set_config(config);

            __voxels_wrapped_main();
        }
    };

    let output = quote! {
        #user_main_fn
        #wrapper_main
    };

    output.into()
}