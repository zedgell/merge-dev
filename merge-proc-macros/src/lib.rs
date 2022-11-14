extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, LitStr, Type};

macro_rules! derive_parse {(
    @derive_only
    $( #[$attr:meta] )*
    $pub:vis
    struct $StructName:ident {
        $(
            $( #[$field_attr:meta] )*
            $field_pub:vis
            $field_name:ident : $FieldTy:ty
        ),* $(,)?
    }
) => (
    impl Parse for $StructName {
        fn parse (input: ParseStream)
          -> ::syn::Result<Self>
        {
            mod kw {
                $(
                    ::syn::custom_keyword!( $field_name );
                )*
            }
            use ::core::ops::Not as _;

            $(
                let mut $field_name = ::core::option::Option::None::< $FieldTy >;
            )*
            while input.is_empty().not() {
                let lookahead = input.lookahead1();
                match () {
                  $(
                    _case if lookahead.peek(kw::$field_name) => {
                        let span = input.parse::<kw::$field_name>().unwrap().span;
                        let _: ::syn::Token![ = ] = input.parse()?;
                        let prev = $field_name.replace(input.parse()?);
                        if prev.is_some() {
                            return ::syn::Result::Err(::syn::Error::new(span, "Duplicate key"));
                        }
                    },
                  )*
                    _default => return ::syn::Result::Err(lookahead.error()),
                }
                let _: ::core::option::Option<::syn::Token![ , ]> = input.parse()?;
            }
            Ok(Self {
                $(
                    $field_name: $field_name.ok_or_else(|| ::syn::Error::new(
                        ::proc_macro2::Span::call_site(),
                        ::core::concat!("Missing key `", ::core::stringify!($field_name), "`"),
                    ))?,
                )*
            })
        }
    }
); (
    $( #[$attr:meta] )* $pub:vis struct $($rest:tt)*
) => (
    $( #[$attr] )* $pub struct $($rest)*

    derive_parse! { @derive_only  $( #[$attr] )* $pub struct $($rest)* }
)}

#[proc_macro_attribute]
pub fn generate_url_params(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);

    let mut string_name = vec![];

    for field in fields.iter() {
        string_name.push(field.clone().ident.unwrap().to_string())
    }

    TokenStream::from(quote! {
        #input

        impl #struct_name {
            fn generate_url_params(&self) -> String {
                let mut url = "".to_string();
                let mut has_params = false;
                #(
                    if self.#field_name.is_some() {
                        if has_params {
                            url = format!("{}&{}={}", url, #string_name, self.clone().#field_name.unwrap());
                        } else {
                            url = format!("?{}={}", #string_name, self.clone().#field_name.unwrap());
                            has_params = true;
                        }
                    }
                )*
                url
            }
        }
    })
}

derive_parse! {
    struct SendRequestArgsLit {
        service: LitStr,
        model: LitStr,
        return_type: Type,
    }
}

#[proc_macro_attribute]
pub fn send_request(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as SendRequestArgsLit);

    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Expected a struct with named fields"),
    };

    let mut has_config = false;

    let mut has_params = false;

    let mut has_id = false;

    for field in fields.iter() {
        let name = field.clone().ident.unwrap().to_string();
        if name == "config" {
            has_config = true;
        } else if name == "params" {
            has_params = true;
        } else if name == "id" {
            has_id = true;
        }
    }

    if !has_params || !has_config {
        panic!("Expected a struct with fields named config and params");
    }

    let return_type = args.return_type.to_token_stream();

    let model = args.model.value();

    let service = args.service.value();

    let url_token = if has_id {
        quote! {
            let url = format!("{}/api/{}/v1/{}/{}{}", url_base, #service, #model, &self.clone().id, url_params);
        }
    } else {
        quote! {
            let url = format!("{}/api/{}/v1/{}{}", url_base, #service, #model, url_params);
        }
    };

    let tokens = quote! {
        #input

        impl #struct_name {
            pub async fn send_request(&self) -> Result<#return_type, String> {
                #[cfg(test)]
                use mockito;

                #[cfg(not(test))]
                let url_base = "https://api.merge.dev";

                #[cfg(test)]
                let url_base = &mockito::server_url();

                let mut url_params = "".to_string();

                if self.clone().params.is_some() {
                    url_params = self.clone().params.unwrap().generate_url_params();
                }

                #url_token

                let req_status = reqwest::Client::new()
                    .get(&url)
                    .bearer_auth(self.clone().config.api_key)
                    .header("X-Account-Token", self.clone().config.access_token)
                    .send()
                    .await;

                match req_status {
                    Ok(req) => match req.status().is_success() {
                        true => match serde_json::from_str::<#return_type>(req.text().await.unwrap().as_str()) {
                            Ok(body) => Ok(body),
                            Err(err) => Err(err.to_string())
                        }
                        false => Err(format!(
                            "Request was not successfully status: {} body: {}",
                            req.status().as_u16(),
                            req.text().await.unwrap()
                        )),
                    },
                    Err(err) => Err(err.to_string()),
                }
            }
        }
    };

    tokens.into()
}
