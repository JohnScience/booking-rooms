use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, ItemFn, ReturnType};

// "Impls expression" is a stipulative definition for an expression that evaluates to true
// if the type implements a trait, and false otherwise.
// Hence, `serde::Deserialize` "impls expression" is an expression that evaluates to true
// if the type implements `serde::Deserialize`, and false otherwise.
fn extend_with_serde_deseralize_impls_expr(
    ts: &mut proc_macro2::TokenStream,
    fn_arg_type: &syn::Type,
) {
    ts.extend(quote! {
        {
            trait DoesNotImplSerdeDeserialize {
                const IMPLS_SERDE_DESERIALIZE: bool = false;
            }

            impl<T: ?Sized> DoesNotImplSerdeDeserialize for T {}

            struct Wrapper<T: ?Sized> (core::marker::PhantomData<T>);

            #[allow(dead_code)]
            impl<T: ?Sized + Copy> Wrapper<T> {
                const IMPLS_SERDE_DESERIALIZE: bool = true;
            }

            <Wrapper<#fn_arg_type>>::IMPLS_SERDE_DESERIALIZE
        }
    });
}

// Extends the token stream with a const assertion that checks if the type
// implements `serde::Deserialize`. If it doesn't, the compilation fails with
// a custom error message.
fn extend_with_serde_deserialize_impls_expr_assert(
    ts: &mut proc_macro2::TokenStream,
    fn_arg_type: &syn::Type,
) {
    let mut group_contents = proc_macro2::TokenStream::new();
    extend_with_serde_deseralize_impls_expr(&mut group_contents, fn_arg_type);
    let group_contents = proc_macro2::TokenStream::from(group_contents);
    ts.extend(quote! {
        // #[doc = "Checks that "]
        // #[doc = stringify!(#fn_arg_type)]
        // #[doc = " implements serde::Deserialize"]
        #[doc = "Check if anything changes"]
        const _: () = if !(#group_contents) {
            panic!("The first argument of the command must implement serde::Deserialize");
        };
    });
}

fn extend_with_command(ts: &mut proc_macro2::TokenStream, fn_item: &ItemFn) {
    let fn_name: &syn::Ident = &fn_item.sig.ident;
    let cmd_name = format!("__command_{fn_name}");
    let cmd_name = syn::Ident::new(&cmd_name, fn_name.span());
}

fn extend_with_serde_deserialize_impls_expr_asserts(
    ts: &mut proc_macro2::TokenStream,
    args: &Punctuated<FnArg, Comma>,
) {
    let mut args_iter = args.clone().into_iter();
    let Some(maybe_receiver) = args_iter.next() else {
        return;
    };

    let FnArg::Typed(first_typed_arg) = maybe_receiver else {
        panic!("The first argument of the command can't be a receiver like `self`, `&self`, or `&mut self`")
    };

    let first_arg_type: &syn::Type = &first_typed_arg.ty;
    extend_with_serde_deserialize_impls_expr_assert(ts, first_arg_type);

    for arg in args_iter {
        let FnArg::Typed(typed_arg) = arg else {
            panic!("The receiver like `self`, `&self`, or `&mut self` can't be a non-first argument of a function");
        };
        let arg_type: &syn::Type = &typed_arg.ty;
        extend_with_serde_deserialize_impls_expr_assert(ts, arg_type);
    }
}

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    {
        let mut tt_iter: TokenTreeIter = attr.into_iter();
        assert!(
            tt_iter.next().is_none(),
            "The {fn_name} attribute does not take any arguments",
            fn_name = stringify!(command)
        );
    }

    let fn_item = syn::parse_macro_input!(item as ItemFn);
    let fn_sig: &syn::Signature = &fn_item.sig;

    let inputs: &Punctuated<FnArg, Comma> = &fn_sig.inputs;
    let output: &ReturnType = &fn_sig.output;

    let mut ts = proc_macro2::TokenStream::new();
    extend_with_serde_deserialize_impls_expr_asserts(&mut ts, inputs);
    extend_with_command(&mut ts, &fn_item);
    ts.extend(quote!(#fn_item));
    ts.into()
}
