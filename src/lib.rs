extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;

use lib_voxels_application_core as vxapp_core;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let block = &input.block;

    let user_main_fn = quote! {
        #(#attrs)*
        #vis fn __voxels_user_main #sig {
            #block
        }
    };



    // Rewrite the function: we generate a real main that loads config and passes it to the user's main
    let wrapper_main = quote! {
        fn main() {
            let config_str = std::fs::read_to_string("voxels.toml")
                .expect("Could not read voxels.toml");
            let config: vxapp_core::application::Application = toml::from_str(&config_str)
                .expect("Failed to parse voxels.toml");
            voxels_application_core::set_config(config);
            __voxels_user_main();
        }
    };

    let output = quote! {
        #user_main_fn
        #wrapper_main
    };

    output.into()
}