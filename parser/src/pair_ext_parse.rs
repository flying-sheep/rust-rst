use std::str::FromStr;

use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use pest::Span;

pub trait PairExt<R>
where
    R: pest::RuleType,
{
    fn parse<T, E>(&self) -> Result<T, Box<Error<R>>>
    where
        T: FromStr<Err = E>,
        E: ToString;
}

impl<R> PairExt<R> for Pair<'_, R>
where
    R: pest::RuleType,
{
    fn parse<T, E>(&self) -> Result<T, Box<Error<R>>>
    where
        T: FromStr<Err = E>,
        E: ToString,
    {
        self.as_str()
            .parse()
            .map_err(|e| to_parse_error(self.as_span(), &e))
    }
}

pub(crate) fn to_parse_error<E, R>(span: Span, e: &E) -> Box<Error<R>>
where
    E: ToString,
    R: pest::RuleType,
{
    let var: ErrorVariant<R> = ErrorVariant::CustomError {
        message: e.to_string(),
    };
    Box::new(Error::new_from_span(var, span))
}
