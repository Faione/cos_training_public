use proc_macro::TokenStream;
mod driver;
mod helpers;

#[proc_macro]
pub fn driver(ts: TokenStream) -> TokenStream {
    driver::driver(ts)
}
