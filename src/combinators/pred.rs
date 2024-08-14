use crate::Parser;

pub fn pred<'a, P, Output, F>(parser: P, predicate: F) -> impl Parser<'a, Output>
where
    P: Parser<'a, Output>,
    F: Fn(&Output) -> bool,
{
    move |input| {
        if let Ok((next_input, output)) = parser.parse(input) {
            if predicate(&output) {
                return Ok((next_input, output));
            }
        }
        Err(input)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::primitives;
    use pretty_assertions::assert_eq;

    #[test]
    fn pred_digit_test_case() {
        let input_data = vec![
            "digit",
            "123",
            " 345",
            "456",
            "four",
            "",
            "4 abobas",
            "09 zero",
            "O9 letter o",
        ]
        .into_iter();

        let parser = pred(primitives::any, |c| c.is_ascii_digit());

        let expected_results = vec![
            Err("digit"),
            Ok(("23", '1')),
            Err(" 345"),
            Ok(("56", '4')),
            Err("four"),
            Err(""),
            Ok((" abobas", '4')),
            Ok(("9 zero", '0')),
            Err("O9 letter o"),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(expected, parser.parse(input));
        }
    }

    #[test]
    fn pred_letter_test_case() {
        let input_data = vec![
            "digit",
            "123",
            "Беларусь",
            " 345",
            "\tnice",
            "",
            "😶",
            "<abobas>",
            "09 zero",
            "O9 letter o",
        ]
        .into_iter();

        let parser = pred(primitives::any, |c| c.is_alphabetic());

        let expected_results = vec![
            Ok(("igit", 'd')),
            Err("123"),
            Ok(("еларусь", 'Б')),
            Err(" 345"),
            Err("\tnice"),
            Err(""),
            Err("😶"),
            Err("<abobas>"),
            Err("09 zero"),
            Ok(("9 letter o", 'O')),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(expected, parser.parse(input));
        }
    }
}
