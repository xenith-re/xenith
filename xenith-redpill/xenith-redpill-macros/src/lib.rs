use proc_macro::TokenStream;
use quote::quote;
// use syn::{parse_macro_input, Attribute, ItemFn};

// use crate::detector::{DetectionResult, TechniqueResult};

// fn technique_1() -> TechniqueResult {
//     Ok(DetectionResult::NotDetected)
// }

// struct Technique1;
// impl Technique for Technique1 {
//     fn name(&self) -> &'static str {
//         "technique_1"
//     }
//     fn description(&self) -> &'static str {
//         "sample description"
//     }
//     fn execute(&self) -> TechniqueResult {
//         technique_1()
//     }
// }

// use static_init::dynamic;

// #[dynamic]
// static INIT: () = {
//     let _ = register_technique(Technique1);
// };

#[proc_macro_attribute]
pub fn technique(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let expanded = quote! {
        struct Hello;
    };

    expanded.into()
}

// #[proc_macro_attribute]
// pub fn technique(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let args = parse_macro_input!(attr as Attribute);
//     let function = parse_macro_input!(item as ItemFn);
//     let name = function.sig.ident.clone();

//     let mut technique_name = String::new();
//     let mut technique_description = String::new();

//     for arg in args.iter() {
//         if let syn::NestedMeta::Meta(syn::Meta::NameValue(ref nv)) = arg {
//             if nv.path.is_ident("name") {
//                 if let syn::Lit::Str(ref lit) = nv.lit {
//                     technique_name = lit.value();
//                 }
//             } else if nv.path.is_ident("description") {
//                 if let syn::Lit::Str(ref lit) = nv.lit {
//                     technique_description = lit.value();
//                 }
//             }
//         }
//     }

//     let expanded = quote! {
//         fn #name() -> TechniqueResult {
//             #function
//         }

//         struct #name;
//         impl Technique for #name {
//             fn name(&self) -> &'static str { #technique_name }
//             fn description(&self) -> &'static str { #technique_description }
//             fn execute(&self) -> TechniqueResult { #name() }
//         }

//         #[ctor::ctor]
//         static INIT: () = {
//             register_technique(#name);
//         };
//     };

//     expanded.into()
// }
