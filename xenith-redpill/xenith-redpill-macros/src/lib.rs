pub(crate) mod technique;

use proc_macro::TokenStream;
use technique::technique_impl;

/// A procedural macro for defining a redpill technique
/// This macro is used to define a new detection technique.
///
/// See the implementation [`technique_impl`] for more details.
#[proc_macro_attribute]
pub fn technique(args: TokenStream, item: TokenStream) -> TokenStream {
    technique_impl(args, item)
}
