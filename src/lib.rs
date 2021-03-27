extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use std::sync::atomic::{AtomicU64, Ordering};

static CURRENT_ID: AtomicU64 = AtomicU64::new(0);

#[proc_macro_hack]
pub fn const_id(_input: TokenStream) -> TokenStream {
    let expr = CURRENT_ID.fetch_add(1, Ordering::SeqCst);
    TokenStream::from(quote! {
        (#expr)
    })
}