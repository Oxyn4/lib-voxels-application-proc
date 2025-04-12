extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemFn};
use quote::quote;


#[proc_macro_attribute]
pub fn vxapp_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let user_main = &input_fn.block;
    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let sig = &input_fn.sig;

    // Rewrite the function: we generate a real main that loads config and passes it to the user's main
    let generated = quote! {
        #(#attrs)*
        #vis fn main() {
            let config_str = std::fs::read_to_string("voxels.toml")
                .expect("Failed to read voxels.toml");
            let config: voxels_application::VoxelsConfig = toml::from_str(&config_str)
                .expect("Failed to parse voxels.toml");

            fn user_main #sig {
                #user_main
            }

            user_main(config);
        }
    };

    generated.into()
}