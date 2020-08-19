use syn::spanned::Spanned;
use syn::{FnArg, ItemFn};

fn main() {
    find_self(
        r#"
    fn always_true(&mut self) -> bool {
        true
    }
    "#,
    );

    find_self(
        r#"
    fn always_true(self) -> bool {
        true
    }
    "#,
    );
}

fn find_self(code: &str) {
    let f: ItemFn = syn::parse_str(code).unwrap();

    for arg in &f.sig.inputs {
        if let FnArg::Receiver(_) = arg {
            println!("{:?}", arg.span());
        }
    }
}
