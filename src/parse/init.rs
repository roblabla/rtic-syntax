use proc_macro2::{Span, TokenStream as TokenStream2};

use syn::{parse, ItemFn};

use crate::{
    ast::{Init, InitArgs, Local},
    parse::util,
    Settings,
};

impl InitArgs {
    pub(crate) fn parse(
        cores: u8,
        tokens: TokenStream2,
        settings: &Settings,
        span: Span,
    ) -> parse::Result<Self> {
        crate::parse::init_idle_args(tokens, cores, settings, cores != 1, span)
    }
}

impl Init {
    pub(crate) fn parse(args: InitArgs, item: ItemFn) -> parse::Result<Self> {
        let valid_signature = util::check_fn_signature(&item) && item.decl.inputs.len() == 1;

        let span = item.ident.span();

        let name = item.ident.to_string();

        if valid_signature {
            if let Ok(returns_late_resources) = util::type_is_late_resources(&item.decl.output) {
                if let Some((context, Ok(rest))) = util::parse_inputs(item.decl.inputs, &name) {
                    if rest.is_empty() {
                        if !returns_late_resources && !args.late.is_empty() {
                            return Err(parse::Error::new(
                                span,
                                "the `late` argument can only be used on functions that \
                                 return `LateResources`",
                            ));
                        }

                        let (locals, stmts) = util::extract_locals(item.block.stmts)?;

                        return Ok(Init {
                            args,
                            attrs: item.attrs,
                            context,
                            locals: Local::parse(locals)?,
                            name: item.ident,
                            returns_late_resources,
                            stmts,
                            _extensible: (),
                        });
                    }
                }
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this `#[init]` must have type signature `fn({}::Context) [-> {0}::LateResources]`",
                name
            ),
        ))
    }
}
