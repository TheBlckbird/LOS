#![no_std]

use proc_macro::TokenStream;
use serial::serial_println;

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    serial_println!("i");
    item
}
