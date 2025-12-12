#[cfg(feature = "builder_lite")]
mod builder_lite;

#[cfg(feature = "doc_display")]
mod doc_display;

#[cfg(feature = "builder_lite")]
#[proc_macro_derive(BuilderLite, attributes(builder))]
/// Automatically implements the builder lite pattern for a struct
pub fn derive_builder_lite(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    builder_lite::expand_builder_lite(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[cfg(feature = "doc_display")]
#[proc_macro_derive(DocDisplay)]
/// Automatically generates an `std::fmt::Display` implementation
/// for structs and enums based on the documentation comments of the
/// given struct/enum.
pub fn derive_doc_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    doc_display::expand_doc_display(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
