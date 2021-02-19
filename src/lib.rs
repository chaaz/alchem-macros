extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn native_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}
