use proc_macro::TokenStream;
use quote::{TokenStreamExt, quote};
use syn::{Attribute, ItemFn, Signature, Visibility, parse::{Parse, ParseStream}, parse_macro_input};

#[proc_macro_attribute]
pub fn nothing(_args: TokenStream, _input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn simple_identity(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn parsing_identity_bad(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn parsing_identity_workaround(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = match syn::parse::<ItemFn>(input.clone()) {
        Ok(it) => it,
        Err(err) => {
            let mut result = input;
            result.extend(TokenStream::from(err.into_compile_error()));
            return result;
        }
    };
    TokenStream::from(quote! {
        #input
    })
}

struct ItemFnWithoutParsingBody {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub block: proc_macro2::TokenStream,
}

impl Parse for ItemFnWithoutParsingBody {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        let block = input.parse()?;
        Ok(ItemFnWithoutParsingBody {
            attrs,
            vis,
            sig,
            block,
        })
    }
}

impl quote::ToTokens for ItemFnWithoutParsingBody {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

#[proc_macro_attribute]
pub fn parsing_identity_body_passthrough(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFnWithoutParsingBody);
    TokenStream::from(quote! {
        #input
    })
}
