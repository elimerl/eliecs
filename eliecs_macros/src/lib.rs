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
        s.sort_by_key(|v| v.ident.to_string());
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
            quote! { #renamed_ident: std::cell::UnsafeCell<Pool<#ident>> }
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
            quote! { #renamed_ident: std::cell::UnsafeCell::new(Pool::new()) }
        })
        .collect::<Vec<_>>();

    let ecs_fields_deser = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );
            quote! { #renamed_ident }
        })
        .collect::<Vec<_>>();

    let ecs_deser = components
        .s
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );
            let len = proc_macro2::Literal::usize_suffixed(i + 2);

            quote! { let #renamed_ident = std::cell::UnsafeCell::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(#i, &self))?,
            ); }
        })
        .collect::<Vec<_>>();

    let ecs_ser = components
        .s
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );

            quote! {
                s.serialize_element(unsafe { &*(self.#renamed_ident.get()) })?;
            }
        })
        .collect::<Vec<_>>();

    let ecs_per_component_methods = components.s.iter().map(|v| {
        let ident = &v.ident;
        let renamed_ident = proc_macro2::Ident::new(
            &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
            ident.span(),
        );
        let renamed_ident_unwrap = proc_macro2::Ident::new(
            &((ident.to_string().strip_prefix("C").unwrap()).to_snake_case() + "_unwrap"),
            ident.span(),
        );
        let renamed_ident_mut = proc_macro2::Ident::new(
            &((ident.to_string().strip_prefix("C").unwrap()).to_snake_case() + "_mut"),
            ident.span(),
        );
        let renamed_ident_mut_unwrap = proc_macro2::Ident::new(
            &((ident.to_string().strip_prefix("C").unwrap()).to_snake_case() + "_mut_unwrap"),
            ident.span(),
        );
        let query_renamed_ident = proc_macro2::Ident::new(
            &("query_".to_string()
                + &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case()),
            ident.span(),
        );
        let query_renamed_ident_mut = proc_macro2::Ident::new(
            &("query_".to_string()
                + &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case()
                + "_mut"),
            ident.span(),
        );
        let add_renamed_ident = proc_macro2::Ident::new(
            &("add_".to_string() + &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case()),
            ident.span(),
        );
        let remove_renamed_ident = proc_macro2::Ident::new(
            &("remove_".to_string()
                + &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case()),
            ident.span(),
        );

        let error_message =
            proc_macro2::Literal::string(&format!("expected entity to have component {}", ident));

        quote! {
            pub fn #renamed_ident(&self, id: u32) -> Option<&#ident> {
                unsafe { (*(self.#renamed_ident.get())).get(id) }
            }

            pub fn #renamed_ident_unwrap(&self, id: u32) -> &#ident {
                unsafe { (*(self.#renamed_ident.get())).get(id) }
                    .expect(#error_message)
            }

            pub fn #renamed_ident_mut(&self, id: u32) -> Option<&mut #ident> {
                unsafe { (*(self.#renamed_ident.get())).get_mut(id) }
            }

            pub fn #renamed_ident_mut_unwrap(&self, id: u32) -> &mut #ident {
                unsafe { (*(self.#renamed_ident.get())).get_mut(id) }
                    .expect(#error_message)
            }

            pub fn #query_renamed_ident(&self) -> impl Iterator<Item = (u32, &#ident)> {
                unsafe { &mut *(self.#renamed_ident.get()) }.iter()
            }
            pub fn #query_renamed_ident_mut(&self) -> impl Iterator<Item = (u32, &mut #ident)> {
                unsafe { &mut *(self.#renamed_ident.get()) }.iter_mut()
            }

            pub fn #add_renamed_ident(&self, id: u32, v: #ident) -> bool {
                unsafe { &mut *(self.#renamed_ident.get()) }.insert(id, v)
            }

            pub fn #remove_renamed_ident(&self, id: u32) {
                unsafe { &mut *(self.#renamed_ident.get()) }.remove(id);
            }
        }
    });

    let ecs_tuple_size = proc_macro2::Literal::usize_suffixed(components.s.len() + 2);

    let component_types = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            quote! { #ident }
        })
        .collect::<Vec<_>>();
    let component_types_containing = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            quote! { #ident (#ident) }
        })
        .collect::<Vec<_>>();
    let component_types_add_to_fat_entity = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );

            quote! { Self::#ident (v) => { fat.#renamed_ident (v) } }
        })
        .collect::<Vec<_>>();

    let spawn_per_component = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );

            quote! { if let Some(v) = data.#renamed_ident {
                self.#renamed_ident.get_mut().insert(e.id, v);
            } }
        })
        .collect::<Vec<_>>();

    let despawn_per_component = components
        .s
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let renamed_ident = proc_macro2::Ident::new(
                &(ident.to_string().strip_prefix("C").unwrap()).to_snake_case(),
                ident.span(),
            );

            quote! {self.#renamed_ident.get_mut().remove(e.id);}
        })
        .collect::<Vec<_>>();

    quote! {
        use eliecs::{Entity, Pool};
        use serde::{
            de::Visitor,
            ser::{SerializeStruct, SerializeTuple},
        };

            #tokens

            #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
            pub enum ComponentType {
                #(#component_types),*
            }

            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            pub enum ComponentTypeContaining {
                #(#component_types_containing),*
            }

            impl ComponentTypeContaining {
                pub fn add_to_fat_entity(self, fat: FatEntity) -> FatEntity {
                    match self {
                        #(#component_types_add_to_fat_entity),*
                    }
                }
            }

            #[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
            pub struct FatEntity {
                #(#fat_fields),*
            }

            impl FatEntity {
                pub fn new() -> Self {
                    Self::default()
                }

                #(#fat_methods)*
            }

            pub struct Ecs {
                existence: Pool<std::num::NonZeroU32>,
                free_list: Vec<Entity>,
                        #(#ecs_fields),*
            }

    #[allow(clippy::mut_from_ref)]
    impl Ecs {
        pub fn new() -> Self {
            Self {
                existence: Pool::new(),
                free_list: Vec::new(),
                #(#ecs_fields_init),*
            }
        }
        pub fn is_alive(&self, e: eliecs::Entity) -> bool {
            self.existence.get(e.id).copied() == Some(e.version)
        }
        pub fn get_entity_from_id(&self, id: u32) -> Option<Entity> {
            Entity::new(id, self.existence.get(id).copied())
        }
        pub fn spawn(&mut self, data: FatEntity) -> eliecs::Entity {
            let e: eliecs::Entity;
            if let Some(v) = self.free_list.pop() {
                e = v;
            } else {
                e = eliecs::Entity::new(self.existence.len(), std::num::NonZeroU32::MIN);
            }
            self.existence.insert(e.id, e.version);

            #(#spawn_per_component)*

            e
        }

        pub fn despawn(&mut self, e: eliecs::Entity) {
            if self.is_alive(e) {
                self.existence.remove(e.id);

                #(#despawn_per_component)*

                let mut v = e;
                v.version = if let Some(v) = v.version.checked_add(1) {
                    v
                } else {
                    std::num::NonZeroU32::MIN
                };
                self.free_list.push(v);
            }
        }

        #(#ecs_per_component_methods)*
    }

    impl serde::Serialize for Ecs {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut s = serializer.serialize_tuple(#ecs_tuple_size)?;
            s.serialize_element(&self.existence)?;
            s.serialize_element(&self.free_list)?;
            #(#ecs_ser)*
            s.end()
        }
    }
    impl<'de> serde::Deserialize<'de> for Ecs {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct ECSVisitor;
            impl<'de> Visitor<'de> for ECSVisitor {
                type Value = Ecs;
                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
                {
                    let existence = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                    let free_list = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                    #(#ecs_deser)*

                    Ok(Ecs {
                        existence,
                        free_list,
                        #(#ecs_fields_deser),*
                    })
                }

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a serialized ECS")
                }
            }
            deserializer.deserialize_tuple(4, ECSVisitor)
        }
    }
        }
    .into()
}
