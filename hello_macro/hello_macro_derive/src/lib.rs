// Procedural macros accept source code as input, operate on that
// source code, and return some code as output. There are three
// kinds of macros: custom derive, attribute-like, and function-like.
//
// The definition of a procedural macro must live in its own crate,
// often nested in the main library crate that it's associated with.
//
// The proc_macro crate ships with the Rust compiler and provides the
// the API that allows us to manipulate Rust source code.
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of the Rust source code as
    // an AST that we can walk and manipulate.
    //
    // Note that procedural macros should panic! with a more
    // specific error message than what will be provuded if .unwrap()
    // causes a panic.
    let ast = syn::parse(input).unwrap();

    // Ensure the actual operation on the source code is performed
    // by the impl_hello_macro function.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // Get the identifier of the struct instance. For example, in:
    //
    // #[derive HelloMacro]
    // struct Pancakes;
    //
    // this would be the string "Pancakes".
    let name = &ast.ident;
    // The quote! macro allows us to define the code we want to return.
    let gen = quote! {
        impl HelloMacro for #name {
            // The stringify! macro is a Rust built-in — it turns any
            // expression into a string literal at compile time.
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // The .into method turns the code into a TokenStream type that can be
    // consumed by the Rust compiler, post_transformation.
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
