use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        BinOp, Expr, ExprBinary, Ident, ItemFn,
        parse::{Parse, ParseStream},
        parse_macro_input,
        visit_mut::VisitMut,
    },
};

pub(super) fn expand_mathinator2000(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let mode = if attr.is_empty() {
        OverflowMode::Checked
    } else {
        match syn::parse::<OverflowMode>(attr) {
            Ok(mode) => mode,
            Err(e) => return e.to_compile_error().into(),
        }
    };

    let mut transformer = Mathinator2000 { mode };
    transformer.visit_item_fn_mut(&mut input);

    TokenStream::from(quote! { #input })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OverflowMode {
    Checked,
    Saturating,
    Wrapping,
}

impl Parse for OverflowMode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "checked" => Ok(OverflowMode::Checked),
            "saturating" => Ok(OverflowMode::Saturating),
            "wrapping" => Ok(OverflowMode::Wrapping),
            _ => Err(syn::Error::new_spanned(
                ident,
                "expected `checked`, `saturating`, or `wrapping`",
            )),
        }
    }
}

struct Mathinator2000 {
    mode: OverflowMode,
}

impl VisitMut for Mathinator2000 {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        syn::visit_mut::visit_expr_mut(self, expr);

        if let Expr::Binary(binary_expr) = expr
            && let Some(new_expr) = self.transform_binary_expr(binary_expr)
        {
            *expr = new_expr;
        }
    }
}

impl Mathinator2000 {
    fn transform_binary_expr(&self, binary: &ExprBinary) -> Option<Expr> {
        let left = &binary.left;
        let right = &binary.right;

        let (base_method, op_symbol) = match binary.op {
            BinOp::Add(_) => ("add", "+"),
            BinOp::Sub(_) => ("sub", "-"),
            BinOp::Mul(_) => ("mul", "*"),
            BinOp::Div(_) => ("div", "/"),
            BinOp::Rem(_) => ("rem", "%"),
            _ => return None,
        };

        match self.mode {
            OverflowMode::Checked => {
                let method = format!("checked_{}", base_method);
                let method_ident = Ident::new(&method, proc_macro2::Span::call_site());

                Some(syn::parse_quote! {
                    {
                        let __left_val = #left;
                        let __right_val = #right;
                        __left_val.#method_ident(__right_val)
                            .expect(&::std::format!(
                                "Arithmetic overflow: {} {} {} exceeds type bounds at {}:{}",
                                __left_val,
                                #op_symbol,
                                __right_val,
                                ::std::file!(),
                                ::std::line!()
                            ))
                    }
                })
            }
            OverflowMode::Saturating => {
                if base_method == "rem" {
                    return None;
                }

                let method = format!("saturating_{}", base_method);
                let method_ident = Ident::new(&method, proc_macro2::Span::call_site());

                Some(syn::parse_quote! {
                    (#left).#method_ident(#right)
                })
            }
            OverflowMode::Wrapping => {
                let method = format!("wrapping_{}", base_method);
                let method_ident = Ident::new(&method, proc_macro2::Span::call_site());

                Some(syn::parse_quote! {
                    (#left).#method_ident(#right)
                })
            }
        }
    }
}
