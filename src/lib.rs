extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Field, DeriveInput};
use quote::Tokens;

mod mut_getters;
mod getters;
mod setters;
mod constructor;
mod functions;

#[proc_macro_derive(Functions, attributes(__function))]
pub fn functions(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for Functions");

    // Build the impl
    let gen = produce_contructor(&ast, functions::implement);

    println!("String Function : {}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(Constructor, attributes(__constructor))]
pub fn constructor(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for Contructor");

    // Build the impl
    let gen = produce_contructor(&ast, constructor::implement);

    println!("String constructor : {}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(Getters, attributes(get))]
pub fn getters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for getters");

    // Build the impl
    let gen = produce(&ast, getters::implement);

    println!("String Getters : {}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(MutGetters, attributes(get_mut))]
pub fn mut_getters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for getters");

    // Build the impl
    let gen = produce(&ast, mut_getters::implement);

    println!("String MutGetters : {}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(Setters, attributes(set))]
pub fn setters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for setters");

    // Build the impl
    let gen = produce(&ast, setters::implement);

    println!("String Setters : {}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}

fn produce(ast: &DeriveInput, worker: fn(&Field) -> Tokens) -> Tokens {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {

        let generated = fields.iter().map(worker).collect::<Vec<_>>();

        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#generated)*
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(Getters)] is only defined for structs, not for enums!");
    }
}

fn produce_contructor(ast: &DeriveInput, worker: fn(&Field, &String) -> Tokens) -> Tokens {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        let classname = format!("{}", ast.ident);
        let generated = fields.iter().map(|x| { (
            worker(x, &classname)
        ) }).collect::<Vec<_>>();

        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#generated)*
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(Getters)] is only defined for structs, not for enums!");
    }
}
