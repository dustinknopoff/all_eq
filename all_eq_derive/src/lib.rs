extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(AllEq)]
pub fn derive_alleq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // get any generics/type info for use
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to compare all fields of a struct.
    let comparers = all_comparisons(&input.data);
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics all_eq::AllEq for #name #ty_generics #where_clause {
            fn all_eq(&self, other: &#name) -> bool {
                #comparers
            }
        }
    };
    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Generate an expression to compare all fields of a struct.
fn all_comparisons(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    //    self.name == other.name && self.email == other.email
                    //
                    // but using fully qualified function call syntax.
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            self.#name == other.#name
                        }
                    });
                    quote! {
                        #(#recurse )&&*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to an expression like
                    //
                    //     self.0 == other.0 && self.1 == other.1
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                             self.#index == other.#index
                        }
                    });
                    quote! {
                        #(#recurse )&&*
                    }
                }
                Fields::Unit => {
                    quote!{}
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
