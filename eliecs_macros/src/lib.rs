use heck::ToSnakeCase;
use proc_macro::{Ident, TokenStream};
use proc_macro_error::{abort, emit_error, emit_warning, proc_macro_error};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemStruct, Result, Visibility,
};

struct ComponentDefs {
    s: Vec<ItemStruct>,
}

impl Parse for ComponentDefs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut s: Vec<ItemStruct> = Vec::new();
        while !input.is_empty() {
            s.push(input.parse()?);
            if !s[s.len() - 1].ident.to_string().starts_with("C") {
                abort!(
                    s[s.len() - 1].ident.span(),
                    "component's name does not start with C"
                );
            }
        }
        Ok(ComponentDefs { s })
    }
}

#[proc_macro_error]
#[proc_macro]
pub fn components(input: TokenStream) -> TokenStream {
    let input_parse = input.clone();
    let components = parse_macro_input!(input_parse as ComponentDefs);

    let tokens: proc_macro2::TokenStream = input.into();

    let fat_fields = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );
            quote! { pub #renamed_ident: Option<#ident> }
        })
        .collect::<Vec<_>>();

    let fat_methods = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );
            quote! { pub fn #renamed_ident (mut self, v: #ident) -> Self {
                self. #renamed_ident = Some(v);
                self
            } }
        })
        .collect::<Vec<_>>();

    let ecs_fields = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );
            quote! { #renamed_ident: Pool<#ident> }
        })
        .collect::<Vec<_>>();

    let ecs_fields_init = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );
            quote! { #renamed_ident: Pool::new() }
        })
        .collect::<Vec<_>>();

    quote! {
        #tokens

        #[derive(Default, Debug)]
        struct FatEntity {
            #(#fat_fields),*
        }

        impl FatEntity {
            pub fn new() -> Self {
                Self::default()
            }

            #(#fat_methods)*
        }

        struct ECS {
            entities: Vec<eliecs::Entity>,
            destroyed: Option<Entity>,
            #(#ecs_fields),*
        }

        impl ECS {
            pub fn new() -> Self {
                Self {
                    #(#ecs_fields_init),*
                }
            }
            pub fn spawn(&mut self, e: FatEntity) -> eliecs::Entity {
                let e: eliecs::Entity;
                if let Some(d) = self.destroyed {
                    let idx = d;

                } else {
                    e = eliecs::Entity::new(self.entities.len() as u32, std::num::NonZeroU32::MIN);
                    self.entities.push(
                        e
                    );
                }
                eliecs::Entity::new(0, std::num::NonZeroU32::MIN)
            }

            pub fn despawn
        }
    }
    .into()
}
