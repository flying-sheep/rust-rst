use pest::{RuleType, iterators::{Pair, Pairs}};
use serde::{
    Serialize,
    Serializer,
    ser::{
        SerializeSeq,
        SerializeStruct,
    },
};

pub struct PairsWrap<'i, R>(pub Pairs<'i, R>);
pub struct PairWrap <'i, R>(pub Pair <'i, R>);

impl<'i, R> Serialize for PairsWrap<'i, R> where R: RuleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut seq = serializer.serialize_seq(None)?;
        for pair in self.0.clone() {
            seq.serialize_element(&PairWrap(pair))?;
        }
        seq.end()
    }
}

impl<'i, R> Serialize for PairWrap<'i, R> where R: RuleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Pair", 3)?;
        state.serialize_field("rule", &format!("{:?}", self.0.as_rule()))?;
        let tokens: Vec<_> = self.0.clone().tokens().collect();
        if tokens.len() > 2 {
            state.serialize_field("inner", &PairsWrap(self.0.clone().into_inner()))?;
        } else {
            state.serialize_field("content", self.0.as_str())?;
        }
        state.end()
    }
}
