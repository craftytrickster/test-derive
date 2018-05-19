extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;



#[proc_macro_derive(SchemaObject)]
pub fn schema_object(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_schema_object(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_schema_object(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    quote! {
        impl<'a> SchemaObject<'a> for #name {
        }
    }
}