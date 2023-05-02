use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Rs2Js)]
pub fn derive_rs_js_obj(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    do_derive_rs_js_obj(input.into()).into()
}

fn do_derive_rs_js_obj(input: TokenStream) -> TokenStream {
    let input = match syn::parse2::<DeriveInput>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(err) => return err.to_compile_error(),
    };

    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        let to_js = fields.named.iter().map(|field| {
            let name = field.ident.as_ref().unwrap();
            let name_str = name.to_string();
            quote! {
                res.unchecked_ref::<rsjs_obj::ObjectExt>().set(#name_str.into(), (&self.#name).into());
            }
        });

        let from_js_defs = fields.named.iter().map(|field| {
            let name = field.ident.as_ref().unwrap();
            let typ = &field.ty;
            quote! {
                let mut #name: Option<#typ> = None;
            }
        });

        let from_js = fields.named.iter().map(|field| {
            let name = field.ident.as_ref().unwrap();
            let name_str = name.to_string();
            let cast = if field.ty.to_token_stream().to_string() == "String" {
                quote! { let value: JsString = value.try_into()?; }
            } else {
                quote! {}
            };
            quote! {
                if key == #name_str {
                    #cast
                    #name = Some(value.into());
                    continue;
                }
            }
        });

        let from_js_collect = fields.named.iter().map(|field| {
            let name = field.ident.as_ref().unwrap();
            let err = format!("Missing field {}", name);
            quote! {
                #name: #name.context(#err)?,
            }
        });

        let name = input.ident;

        quote! {
            impl rsjs_obj::RsJsObj for #name {
                fn to_js(&self) -> JsValue {
                    let mut res = Object::new();
                    #(#to_js)*
                    res.into()
                }

                fn from_js(js: JsValue) -> anyhow::Result<Self> {
                    use anyhow::Context;
                    if !js.is_object() {
                        anyhow::bail!("JsValue is not an object");
                    }
                    #(#from_js_defs)*
                    let entries = Object::entries(js.unchecked_ref());
                    for pair in entries.iter() {
                        let pair = pair.unchecked_into::<Array>();
                        let key: JsString = pair.get(0).try_into()?;
                        let value = pair.get(1);
                        #(#from_js)*
                    }
                    Ok(Self {
                        #(#from_js_collect)*
                    })
                }
            }
        }
    } else {
        panic!("Not a struct with named fields");
    }
}

#[test]
fn test_struct() {
    let input = quote! {
        struct Test {
            my_string_field: String,
        }
    };
    let output: TokenStream = do_derive_rs_js_obj(input);
    pretty_assertions::assert_eq!(
        output.to_string(),
        quote! {
            impl rsjs_obj::RsJsObj for Test {
                fn to_js(&self) -> JsValue {
                    let mut res = Object::new();
                    res.unchecked_ref::<rsjs_obj::ObjectExt>().set("my_string_field".into(), (&self.my_string_field).into());
                    res.into()
                }

                fn from_js(js: JsValue) -> anyhow::Result<Self> {
                    use anyhow::Context;
                    if !js.is_object() {
                        anyhow::bail!("JsValue is not an object");
                    }
                    let mut my_string_field: Option<String> = None;
                    let entries = Object::entries(js.unchecked_ref());
                    for pair in entries.iter() {
                        let pair = pair.unchecked_into::<Array>();
                        let key: JsString = pair.get(0).try_into()?;
                        let value = pair.get(1);
                        if key == "my_string_field" {
                            my_string_field = Some(value.into());
                            continue;
                        }
                    }
                    Ok(Self {
                        my_string_field: my_string_field.context("Missing field my_string_field")?,
                    })
                }
            }

        }
        .to_string()
    );
}
