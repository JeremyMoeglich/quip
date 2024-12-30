use fst::VariableCreation;
use parser_core::*;

use crate::{
    destructure::parse_mutable_destructure,
    utils::{opt, ws1},
};

pub fn parse_variable_creation<'a>(input: Span<'a>) -> ParserResult<'a, VariableCreation> {
    (
        (opt((parse_mut, ws1).tuple()), parse_ident)
            .tuple()
            .map(|(mutable, ident)| VariableCreation::Identifier {
                name: ident.to_string(),
                mutable: mutable.is_some(),
            }),
        parse_mutable_destructure.map(|destructure| VariableCreation::Destructure { destructure }),
    )
        .alt()(input)
}
