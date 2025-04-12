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

    let stmts = &input.block.stmts;


    let wrapped_main = quote! {
        #(#attrs)*
        #vis fn #sig {
            let config_str = include_str!("voxels.toml");

            let config : lib_voxels_application_core::application::Application = toml::from_str(include_str!("voxels.toml")).unwrap();

            #(#stmts)*
        }
    };

    wrapped_main.into()
}