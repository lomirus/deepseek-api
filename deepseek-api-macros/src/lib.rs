use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    Attribute, Error, Expr, FnArg, Ident, ItemFn, Lit, Meta, Pat, Result, parse_macro_input,
};

#[proc_macro_attribute]
pub fn tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    if !attr.is_empty() {
        return Error::new(Span::call_site(), "#[tool] does not accept any arguments")
            .into_compile_error()
            .into();
    }

    let item_fn = parse_macro_input!(item as ItemFn);

    match expand_tool(item_fn) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn expand_tool(item_fn: ItemFn) -> Result<proc_macro2::TokenStream> {
    if item_fn.sig.asyncness.is_none() {
        return Err(Error::new_spanned(
            &item_fn.sig.ident,
            "#[tool] only supports async functions",
        ));
    }

    if !item_fn.sig.generics.params.is_empty() || item_fn.sig.generics.where_clause.is_some() {
        return Err(Error::new_spanned(
            &item_fn.sig.generics,
            "#[tool] does not support generic functions",
        ));
    }

    let fn_name = &item_fn.sig.ident;
    let fn_name_string = raw_ident_string(fn_name);
    let description = extract_description(&item_fn.attrs)?;
    let description = syn::LitStr::new(&description, fn_name.span());
    let tool_name = syn::LitStr::new(&fn_name_string, fn_name.span());
    let tool_const_ident = format_ident!("{}", fn_name_string.to_ascii_uppercase());
    let args_struct_ident = format_ident!("__deepseek_api_{}_args", fn_name);
    let schema_fn_ident = format_ident!("__deepseek_api_{}_schema", fn_name);
    let call_fn_ident = format_ident!("__deepseek_api_{}_call", fn_name);
    let vis = &item_fn.vis;
    let cfg_attrs = item_fn
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr"))
        .collect::<Vec<_>>();

    let params = parse_params(&item_fn)?;
    let arg_fields = params
        .iter()
        .map(|(ident, ty)| quote! { #ident: #ty })
        .collect::<Vec<_>>();
    let call_args = params
        .iter()
        .map(|(ident, _)| quote! { __deepseek_api_args.#ident })
        .collect::<Vec<_>>();

    let invoke = if call_args.is_empty() {
        quote! { #fn_name().await }
    } else {
        quote! { #fn_name(#(#call_args),*).await }
    };

    Ok(quote! {
        #item_fn

        #(#cfg_attrs)*
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[derive(::deepseek_api::__private::serde::Deserialize, ::deepseek_api::__private::schemars::JsonSchema)]
        struct #args_struct_ident {
            #(#arg_fields,)*
        }

        #(#cfg_attrs)*
        #[doc(hidden)]
        fn #schema_fn_ident() -> &'static ::deepseek_api::__private::schemars::Schema {
            static SCHEMA: ::std::sync::OnceLock<::deepseek_api::__private::schemars::Schema> =
                ::std::sync::OnceLock::new();
            SCHEMA.get_or_init(|| ::deepseek_api::__private::schemars::schema_for!(#args_struct_ident))
        }

        #(#cfg_attrs)*
        #[doc(hidden)]
        fn #call_fn_ident(args: ::std::string::String) -> ::deepseek_api::ToolFuture {
            ::std::boxed::Box::pin(async move {
                let __deepseek_api_args: #args_struct_ident =
                    ::deepseek_api::__private::serde_json::from_str(&args).unwrap();
                let result = #invoke;
                ::deepseek_api::__private::serde_json::to_string(&result).unwrap()
            })
        }

        #(#cfg_attrs)*
        #vis const #tool_const_ident: ::deepseek_api::Tool = ::deepseek_api::Tool::new(
            #tool_name,
            #description,
            #schema_fn_ident,
            #call_fn_ident,
        );
    })
}

fn parse_params(item_fn: &ItemFn) -> Result<Vec<(Ident, syn::Type)>> {
    item_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(receiver) => Err(Error::new_spanned(
                receiver,
                "#[tool] does not support methods or self parameters",
            )),
            FnArg::Typed(pat_type) => match pat_type.pat.as_ref() {
                Pat::Ident(pat_ident) if pat_ident.subpat.is_none() => {
                    Ok((pat_ident.ident.clone(), (*pat_type.ty).clone()))
                }
                _ => Err(Error::new_spanned(
                    &pat_type.pat,
                    "#[tool] only supports simple named parameters",
                )),
            },
        })
        .collect()
}

fn extract_description(attrs: &[Attribute]) -> Result<String> {
    let mut lines = Vec::new();

    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }

        match &attr.meta {
            Meta::NameValue(meta) => match &meta.value {
                Expr::Lit(expr_lit) => match &expr_lit.lit {
                    Lit::Str(lit) => {
                        let raw = lit.value();
                        let trimmed = raw.strip_prefix(' ').unwrap_or(&raw);
                        lines.push(trimmed.to_string());
                    }
                    _ => {
                        return Err(Error::new_spanned(
                            &expr_lit.lit,
                            "invalid doc comment value",
                        ));
                    }
                },
                _ => return Err(Error::new_spanned(&meta.value, "invalid doc comment value")),
            },
            _ => return Err(Error::new_spanned(attr, "invalid doc comment")),
        }
    }

    let description = lines.join("\n").trim().to_string();
    if description.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[tool] requires a non-empty doc comment as the tool description",
        ));
    }

    Ok(description)
}

fn raw_ident_string(ident: &Ident) -> String {
    let name = ident.to_string();
    name.strip_prefix("r#").unwrap_or(&name).to_string()
}
