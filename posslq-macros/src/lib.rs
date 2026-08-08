use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::fs;

#[proc_macro]
pub fn build_posslq_matrix(_input: TokenStream) -> TokenStream {
    let macro_crate_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory env var not found");

    let target_file = std::path::Path::new(&macro_crate_dir)
        .parent()
        .unwrap()
        .join("harper-core")
        .join("src")
        .join("dict_word_metadata.rs");

    let content = fs::read_to_string(&target_file).unwrap_or_else(|_| {
        panic!(
            "posslq-macro could not find harper-core source at absolute path: {:?}",
            target_file
        )
    });

    let ast = syn::parse_file(&content).expect("Failed to parse harper-core syntax tree");

    let mut enum_variants = Vec::new();
    let mut ctor_matches = Vec::new();
    let mut variant_idents = Vec::new();
    let mut variant_prop_counts = Vec::new();

    for item in ast.items {
        if let syn::Item::Struct(item_struct) = item {
            let struct_name = item_struct.ident.to_string();
            if !struct_name.ends_with("Data") {
                continue;
            }

            let raw_name = struct_name.strip_suffix("Data").unwrap_or(&struct_name);
            let variant_ident = format_ident!("{}", raw_name);
            let metadata_field_ident = format_ident!("{}", raw_name.to_lowercase());

            variant_idents.push(variant_ident.clone());

            let mut packing_exprs = Vec::new();

            for (index, field) in item_struct.fields.iter().enumerate() {
                let field_type = &field.ty;
                let ty_tokens = quote! { #field_type };
                let ty_str = ty_tokens.to_string().replace(" ", "");

                if ty_str == "Option<bool>"
                    && let Some(field_ident) = &field.ident
                {
                    let bit_shift = index * 2;
                    packing_exprs.push(quote! {
                        (Trit::from_opt(data.#field_ident) as u64) << #bit_shift
                    });
                }
            }

            enum_variants.push(quote! { #variant_ident(u64) });

            // Save the exact number of Option<bool> properties this struct possessed
            variant_prop_counts.push(packing_exprs.len());

            if packing_exprs.is_empty() {
                ctor_matches.push(quote! {
                    if let Some(_data) = &wmd.#metadata_field_ident {
                        results.push(Self::#variant_ident(0));
                    }
                });
            } else {
                ctor_matches.push(quote! {
                    if let Some(data) = &wmd.#metadata_field_ident {
                        let bits: u64 = #(#packing_exprs)|*;
                        results.push(Self::#variant_ident(bits));
                    }
                });
            }
        }
    }

    let expanded = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[repr(u8)]
        pub enum Trit {
            None = 0b00,
            False = 0b01,
            True = 0b10,
        }

        impl Trit {
            #[inline(always)]
            pub fn from_opt(val: Option<bool>) -> Self {
                match val {
                    None => Self::None,
                    Some(false) => Self::False,
                    Some(true) => Self::True,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum PosslqPropertyField {
            #(#enum_variants,)*
        }

        impl PosslqPropertyField {
            pub fn from_metadata(wmd: &harper_core::DictWordMetadata) -> Vec<Self> {
                let mut results = Vec::new();
                #(#ctor_matches)*
                results
            }

            pub fn variant_name(&self) -> &'static str {
                match self {
                    #(
                        Self::#variant_idents(_) => stringify!(#variant_idents),
                    )*
                }
            }

            pub fn raw_payload(&self) -> u64 {
                match *self {
                    #(
                        Self::#variant_idents(bits) => bits,
                    )*
                }
            }

            /// Returns a compact string layout of the bitfield (e.g., "T-F--")
            pub fn trit_string(&self) -> String {
                match *self {
                    #(
                        Self::#variant_idents(bits) => {
                            let mut s = String::new();
                            // Loop over exactly how many fields this variant tracks
                            for i in 0..#variant_prop_counts {
                                let shift = i * 2;
                                let raw_trit = (bits >> shift) & 0b11;
                                match raw_trit {
                                    0b10 => s.push('T'),
                                    0b01 => s.push('F'),
                                    _    => s.push('-'),
                                }
                            }
                            s
                        }
                    )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
