use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    // println!("{}", input.sig.ident);
    let test_name_ident = &input.sig.ident;
    let test_name = format!("{}", test_name_ident);

    let output = quote! {
        // serial_println!("test {}\t", #test_name);
        #[cfg(test)]
        #[test_case]
        #input
        // serial_println!("[ok]");
    };

    output.into()
}
