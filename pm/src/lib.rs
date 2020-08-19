use syn::spanned::Spanned;
use syn::{FnArg, ItemFn};

#[proc_macro_attribute]
pub fn pm(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item2 = item.clone();
    let f: ItemFn = syn::parse_macro_input!(item2);

    for arg in f.sig.inputs {
        if let FnArg::Receiver(_) = arg {
            let span = arg.span();
            return syn::Error::new(span, &format!("Saw receiver with span {:?}", span))
                .to_compile_error()
                .into();
        }
    }

    return item;
}
