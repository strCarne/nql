#![allow(unused)]

use crate::{
    basic_types::{self, Date, Number},
    combinators, primitives, Parser, ParsingResult,
};

#[derive(Debug)]
pub struct Range {
    pub bounds: RangeBounds,
    pub op: RangeOp,
}

#[derive(Debug)]
pub enum RangeBounds {
    DateRange(Date, Date),
    NumberRange(Number, Number),
}

impl RangeBounds {
    pub fn to_strings(&self) -> (String, String) {
        match self {
            RangeBounds::DateRange(date_1, date_2) => (
                format!("'{}'", date_1.to_string()),
                format!("'{}'", date_2.to_string()),
            ),
            RangeBounds::NumberRange(number_1, number_2) => {
                (number_1.to_string(), number_2.to_string())
            }
        }
    }
}

// Last example fails, because it will be recognized as a number at first, so number
// will return '-01-01' as the next input tokens. That will be an error for range
// parser, because it will expect an token of type 'range_op'
//
// RANGE ::= DATE_RANGE | NUMBER_RANGE
// DATE_RANGE ::= DATE RANGE_OP DATE
// NUMBER_RANGE ::= NUMBER RANGE_OP NUMBER
//
// RANGE_OP >> see grammar::value::range::range_op.rs
// NUMBER >> see grammar::basic_types::number::number.rs
// DATE >> see grammar::basic_types::date.rs
pub fn range(input: &str) -> ParsingResult<Range> {
    combinators::single_of(vec![
        basic_types::date
            .whitespace_wrap()
            .and_then(|date_1| range_op.whitespace_wrap().map(move |op| (date_1, op)))
            .and_then(|(date_1, op)| {
                basic_types::date
                    .whitespace_wrap()
                    .map(move |date_2| Range {
                        bounds: RangeBounds::DateRange(date_1, date_2),
                        op,
                    })
            }),
        basic_types::number
            .whitespace_wrap()
            .and_then(|number_1| range_op.whitespace_wrap().map(move |op| (number_1, op)))
            .and_then(|(number_1, op)| {
                basic_types::number
                    .whitespace_wrap()
                    .map(move |number_2| Range {
                        bounds: RangeBounds::NumberRange(number_1, number_2),
                        op,
                    })
            }),
    ])
    .parse(input)
}

// RANGE_OP ::= II | IE | EI | EE
// II := ==
// IE := =\.
// EI := \.=
// EE := \.\.
// NOTE: I - inclusion, E - exclusion
#[derive(Clone, Copy, Debug)]
pub enum RangeOp {
    II,
    IE,
    EI,
    EE,
}

pub fn range_op(input: &str) -> ParsingResult<RangeOp> {
    combinators::single_of(vec![
        primitives::literal("==").map(|_| RangeOp::II),
        primitives::literal("=.").map(|_| RangeOp::IE),
        primitives::literal(".=").map(|_| RangeOp::EI),
        primitives::literal("..").map(|_| RangeOp::EE),
    ])
    .parse(input)
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "Not implemented yet"]
    fn range_test() {
        todo!("Make unit test")
    }
}
