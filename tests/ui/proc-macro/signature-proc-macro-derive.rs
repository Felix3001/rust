// force-host
// no-prefer-dynamic

#![crate_type = "proc-macro"]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Blah)]
pub fn bad_input(input: String) -> TokenStream {
    //~^ ERROR mismatched derive proc macro signature
    TokenStream::new()
}

#[proc_macro_derive(Bleh)]
pub fn bad_output(input: TokenStream) -> String {
    //~^ ERROR mismatched derive proc macro signature
    String::from("blah")
}

#[proc_macro_derive(Bluh)]
pub fn bad_everything(input: String) -> String {
    //~^ ERROR mismatched derive proc macro signature
    //~| ERROR mismatched derive proc macro signature
    input
}

#[proc_macro_derive(Blih)]
pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
    //~^ ERROR mismatched derive proc macro signature
}
