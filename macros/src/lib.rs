use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn declare_reframework_handle(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_ident = parse_macro_input!(input as Ident);
    let struct_name = quote::format_ident!("__{}", input_ident);

    let expanded = quote! {
        pub struct #struct_name;
        pub type #input_ident = *const #struct_name;
    };

    expanded.into()
}
