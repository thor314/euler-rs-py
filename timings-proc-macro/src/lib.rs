use proc_macro::TokenStream;

/// ```
/// #[timings]
/// fn f(){
/// ...blah...
/// }
/// // generates:
/// fn f(){
///     let now = std::time::Instant::now();
///     ...blah...
///     println!("time: {:?}", now.elapsed());
/// }
/// ```
#[proc_macro_attribute]
pub fn timings(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::ItemFn);
    handle_timings_macro(&ast)
}

fn handle_timings_macro(ast: &syn::ItemFn) -> TokenStream {
    let vis = &ast.vis;
    let sig = &ast.sig;
    let block_stmts = &ast.block.stmts;
    let gen = quote::quote! {
      #vis #sig
      {
        let now = std::time::Instant::now();
        #(#block_stmts)*
        println!("time: {:?}", now.elapsed());
      }
    };
    gen.into()
}
