use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use uuid::Uuid;

#[proc_macro_derive(RemoveSurrealPrefix, attributes(surreal))]
pub fn remove_surreal_prefix(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let data = match ast.data {
        Data::Struct(ref data) => data,
        _ => panic!("remove_surreal_prefix only supports structs"),
    };
    let fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        _ => panic!("remove_surreal_prefix only supports named fields"),
    };
    let id_fields = fields
        .iter()
        .filter(|field| field.ident.as_ref().map(|ident| ident == "id").unwrap_or(false))
        .collect::<Vec<_>>();
    if id_fields.is_empty() {
        panic!("remove_surreal_prefix requires at least one field named 'id'");
    }
    let attrs = id_fields.iter().map(|field| {
        quote! {
            #[serde(deserialize_with = "uuid_from_str")]
        }
    });
    let field_id = id_fields[0].ident.as_ref().unwrap();
    let attrs2 = quote! {
        #(#attrs)*
    };
    let expanded = quote! {
        #[derive(#ast)]
        #[serde(rename_all = "camelCase")]
        pub struct #name {
            #attrs2
            pub #field_id: uuid::Uuid,
            #(#fields),*
        }
    };
    TokenStream::from(expanded)
}

fn uuid_from_str<'de, D>(deserializer: D) -> Result<uuid::Uuid, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    let s = s.replace("⟨", "").replace("⟩", "");
    let uuid_string = s.split(":").last().unwrap();
    uuid::Uuid::parse_str(uuid_string).map_err(serde::de::Error::custom)
}