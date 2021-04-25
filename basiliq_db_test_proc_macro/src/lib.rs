extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn basiliq_test(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input as ItemFn);
    let attrs = parse_macro_input!(metadata as AttributeArgs);
    let mut run_migrations: bool = false;
    let mut init_values: bool = false;

    for attr in attrs {
        match attr {
            NestedMeta::Meta(meta) => match meta {
                Meta::Path(path) => match path.get_ident() {
                    Some(ident) => {
                        if ident.eq("run_migrations") {
                            run_migrations = true;
                        } else if ident.eq("init_values") {
                            init_values = true;
                        }
                    }
                    _ => continue,
                },
                _ => continue,
            },
            NestedMeta::Lit(_) => (),
        }
    }
    let function_name = input_fn.sig.ident.clone();
    let new_function_name = Ident::new(format!("db_{}", function_name).as_str(), Span::call_site());
    input_fn.sig.ident = new_function_name.clone();

    TokenStream::from(quote! {
        #[tokio::test]
        async fn #function_name()
        {
            let (db_id, pool) = basiliq_db_test_utils::init_db(#run_migrations, #init_values).await;
            #new_function_name(pool.clone()).await;
            pool.close().await;
            basiliq_db_test_utils::deinit_db(db_id).await;
        }

        #input_fn
    })
}
