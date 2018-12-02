use std::str::FromStr;

use pest::iterators::Pair;
use pest::error::{Error,ErrorVariant};


pub trait PairExt<R> where R: pest::RuleType {
    fn parse<T, E>(&self) -> Result<T, Error<R>> where T: FromStr<Err = E>, E: ToString;
}

impl<'l, R> PairExt<R> for Pair<'l, R> where R: pest::RuleType {
    fn parse<T, E>(&self) -> Result<T, Error<R>> where T: FromStr<Err = E>, E: ToString {
        self.as_str().parse().map_err(|e: T::Err| {
            let var: ErrorVariant<R> = ErrorVariant::CustomError { message: e.to_string() };
            Error::new_from_span(var, self.as_span())
        })
    }
}
