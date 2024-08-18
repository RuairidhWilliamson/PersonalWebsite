use proc_macro::TokenStream;
use quote::quote;

#[allow(clippy::missing_panics_doc)]
#[proc_macro_attribute]
pub fn job(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item).expect("fn item");

    let vis = &ast.vis;
    let sig = &ast.sig;
    let block = &ast.block;
    let name = &ast.sig.ident.to_string();
    let args = ast.sig.inputs.iter().filter_map(|arg| {
        let syn::FnArg::Typed(pat) = arg else {
            return None;
        };
        Some(&pat.pat)
    });

    quote! {
        #vis #sig {
            let id = jobber::JobIdBuilder::new(#name)#(.arg(&#args))*.build();
            let f = |ctx: &mut jobber::JobCtx<'_>| #block;
            ctx.job(id, f)
        }
    }
    .into()
}
