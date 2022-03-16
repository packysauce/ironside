use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(cmd), supports(struct_named))]
struct Command {
    ident: Ident,
    generics: syn::Generics,
    data: Data<(), CommandField>,
    /// OID used to identify the command between host and MCU
    oid: u32,
    /// Name of the reponse type this command
    response: Option<String>,
}

#[derive(FromField)]
struct CommandField {
    // These come from the FromField derive, they're magic
    ident: Option<Ident>,
    ty: Type,
    // These come from our #[cmd(rename=blah, response=Blah)]
    response: Type,
    rename: Option<&'static str>,
}

#[proc_macro_derive(Command, attributes(klip))]
pub fn command(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);
    let Command {
        ident,
        generics,
        data,
        oid,
        response,
    } = Command::from_derive_input(&input).unwrap();
    let resp_type = response.or_else(|| Some("()".to_string()));

    quote! {
        impl Request for #ident {
            const OID: u32 = #oid;
            pub type Resp = ();
        }
    }
    .into()
}
