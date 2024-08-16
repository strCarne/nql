use crate::{
    combinators::exact,
    primitives::character,
    Parser, ParsingResult,
};

use super::{number, Number};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }
}

// DATE ::= DAY:MONTH:YEAR
// DAY ::= INT_NUMBER
// 1 <= DAY <= MAX_DAYS_IN(MONTH, YEAR)
// MONTH ::= INT_NUMBER
// 1 <= MONTH <= 12
// YEAR ::= INT_NUMBER
pub fn date(input: &str) -> ParsingResult<Date> {

    // TODO [refactor]: Don't really like how it came out in the code
    let result = exact(number.wrap_after(character(':')), 2)
        .and_then(|res| number
            .map(move |num| (res[0].clone(), res[1].clone(), num))
        )
        .parse(input);

    if let Err(_) =result {
        return Err(input);
    }

    let (next_input, (day, month, year)) = result.unwrap();
        

    let day = match day {
        Number::Integer(i) if 1 <= i && i <= 31 => i as u8,
        _ => return Err(input),
    };

    let month = match month {
        Number::Integer(i) if 1 <= i && i <= 12 => i as u8,
        _ => return Err(input),
    };

    let year = match year {
        Number::Integer(i) if i32::MIN as i64 <= i && i <= i32::MAX as i64 => i as i32,
        _ => return Err(input),
    };

    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    if day > month_days[month as usize - 1] {
        return Err(input);
    }

    let timestamp = Date { year, month, day };
    Ok((next_input, timestamp))
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn date_parsing() {
        let input_data = vec![
            "28:12:2004",
            "1:1:2000",
            "01:01:2000",
            "01:2000 next tokens",
            "31:13:2024",
            "32:12:2024",
            "29:02:2023",
            "29:02:2024",
            "31:9:2024",
        ]
        .into_iter();

        let expected_results = vec![
            Ok(("", Date::new(2004, 12, 28))),
            Ok(("", Date::new(2000, 1, 1))),
            Ok(("", Date::new(2000, 1, 1))),
            Err("01:2000 next tokens"),
            Err("31:13:2024"),
            Err("32:12:2024"),
            Err("29:02:2023"),
            Ok(("", Date::new(2024, 2, 29))),
            Err("31:9:2024"),
        ];

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            let result = date(input);
            assert_eq!(result, expected);
        }
    }
}
