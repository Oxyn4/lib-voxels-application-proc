extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let user_block = &input_fn.block;
    let user_vis = &input_fn.vis;
    let user_attrs = &input_fn.attrs;
    let user_sig = &input_fn.sig;

    // Rewrite the function: we generate a real main that loads config and passes it to the user's main
    let generated = quote! {
        #(#user_attrs)*
        #user_vis fn __voxels_user_main #user_sig {
            #user_block
        }

        // We replace main() with our own that loads config, sets it, and runs the user's main
        fn main() {
            let config_str = std::fs::read_to_string("voxels.toml")
                .expect("Could not read voxels.toml");
            let config: voxels_application_core::VoxelsConfig = toml::from_str(&config_str)
                .expect("Failed to parse voxels.toml");
            voxels_application_core::set_config(config);

            __voxels_user_main();
        }
    };

    generated.into()
}