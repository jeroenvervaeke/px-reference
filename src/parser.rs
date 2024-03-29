use wasm_bindgen::prelude::*;

const CHAR_CODE_A: u32 = 'A' as u32;
const CHAR_CODE_R: u32 = 'R' as u32;
const CHAR_R_OFFSET: u32 = CHAR_CODE_R - 10;

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
    where
        F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Reference {
    pub reference_type: u32,
    pub company_space_id: Option<u32>,
    pub aggregate_root_id: Option<u32>,
    pub revision: Option<u32>,
    pub object_id: Option<u32>,
}

#[wasm_bindgen]
pub fn parse_reference(input: &str) -> Option<Reference> {
    let reference_parser = create_reference_structure_parser();

    match reference_parser.parse(input) {
        Ok((_, (reference_type, ids))) => Some(Reference {
            reference_type,
            company_space_id: ids.get(0).map(|id| *id),
            aggregate_root_id: ids.get(1).map(|id| *id),
            revision: ids.get(2).map(|id| *id),
            object_id: ids.get(3).map(|id| *id),
        }),
        Err(_) => None,
    }
}

fn create_reference_structure_parser<'a>() -> impl Parser<'a, (u32, Vec<u32>)> {
    move |input: &'a str| left(pair(encoded_group, one_or_more(right(dash, encoded_group))), empty).parse(input)
}

fn dash(input: &str) -> ParseResult<()> {
    match input.chars().next() {
        Some('-') => Ok((&input['-'.len_utf8()..], ())),
        _ => Err(input),
    }
}

fn empty(input: &str) -> ParseResult<()> {
    match input.chars().next() {
        None => Ok((&input, ())),
        _ => Err(input),
    }
}

fn encoded_group(input: &str) -> ParseResult<u32> {
    let (decoded, total_char_length) = input.chars()
        .map(|character| (char_to_number(character), character.len_utf8()))
        .take_while(|(number_result, character_length)| number_result.is_some())
        .map(|(character_option, character_length)| (character_option.unwrap(), character_length))
        .collect::<Vec<(u32, usize)>>()
        .iter()
        .rev()
        .enumerate()
        .fold((0, 0usize), |(decoded, total_char_length), (index, (decoded_value, character_length))| (
            decoded + u32::pow(16, index as u32) * decoded_value,
            total_char_length + character_length),
        );

    if total_char_length == 0 {
        Err(input)
    } else {
        Ok((&input[total_char_length..], decoded))
    }
}

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
    where P1: Parser<'a, R1>,
          P2: Parser<'a, R2>,
{
    move |input: &'a str| match parser1.parse(input) {
        Ok((next_input, result1)) => match parser2.parse(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B
{
    move |input: &'a str| match parser.parse(input) {
        Ok((next_input, result)) => Ok((next_input, map_fn(result))),
        Err(err) => Err(err),
    }
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
    where
        P: Parser<'a, A>,
{
    move |input: &'a str| {
        let mut result = Vec::new();
        let mut next_input = input;

        while let Ok((next, next_item)) = parser.parse(next_input) {
            next_input = next;
            result.push(next_item);
        }

        if result.is_empty() {
            Err(input)
        } else {
            Ok((next_input, result))
        }
    }
}

fn char_to_number(character: char) -> Option<u32> {
    match character {
        c @ 'A'...'J' => Some((c as u32) - CHAR_CODE_A),
        c @ 'R'...'W' => Some((c as u32) - CHAR_R_OFFSET),
        _ => None
    }
}

#[cfg(test)]
mod char_to_number {
    use super::*;

    #[test]
    fn char_to_number_a() {
        assert_eq!(char_to_number('A'), Some(0))
    }

    #[test]
    fn char_to_number_j() {
        assert_eq!(char_to_number('J'), Some(9))
    }

    #[test]
    fn char_to_number_r() {
        assert_eq!(char_to_number('R'), Some(10))
    }

    #[test]
    fn char_to_number_w() {
        assert_eq!(char_to_number('W'), Some(15))
    }

    #[test]
    fn char_to_number_k() {
        assert_eq!(char_to_number('K'), None)
    }

    #[test]
    fn char_to_number_z() {
        assert_eq!(char_to_number('Z'), None)
    }
}

#[cfg(test)]
mod dash {
    use super::*;

    #[test]
    fn dash_dash_aaaa() {
        assert_eq!(dash("-AAAA"), Ok(("AAAA", ())));
    }

    #[test]
    fn dash_abcde_dash() {
        assert_eq!(dash("ABCDE-"), Err("ABCDE-"));
    }
}

#[cfg(test)]
mod empty {
    use super::*;

    #[test]
    fn emtpy() {
        assert_eq!(empty(""), Ok(("", ())));
    }

    #[test]
    fn non_empty() {
        assert_eq!(empty("not empty"), Err("not empty"));
    }
}

#[cfg(test)]
mod encoded_group {
    use super::*;

    #[test]
    fn encoded_group_aaaa() {
        assert_eq!(encoded_group("AAAA"), Ok(("", 0)));
    }

    #[test]
    fn encoded_group_aaaa_dash_aaaab() {
        assert_eq!(encoded_group("AAAA-AAAAB"), Ok(("-AAAAB", 0)));
    }

    #[test]
    fn encoded_group_llaa() {
        assert_eq!(encoded_group("LLAA"), Err("LLAA"));
    }

    #[test]
    fn encoded_group_abll() {
        assert_eq!(encoded_group("ABLL"), Ok(("LL", 1)));
    }

    #[test]
    fn encoded_group_wwwwwwww_dash_123() {
        assert_eq!(encoded_group("WWWWWWWW-123"), Ok(("-123", 4_294_967_295)));
    }

    #[test]
    fn encoded_group_aaaaaacr() {
        assert_eq!(encoded_group("AAAAAACR"), Ok(("", 42)));
    }

    #[test]
    fn encoded_group_bijdrage_dash_nummer_dash_1() {
        assert_eq!(encoded_group("BIJDRAGE-NUMMER-1"), Ok(("-NUMMER-1", 412_328_036)));
    }
}

#[cfg(test)]
mod pair {
    use super::*;

    #[test]
    fn pair_dash_dash() {
        assert_eq!(pair(dash, dash).parse("--"), Ok(("", ((), ()))));
    }

    #[test]
    fn pair_aaaa_dash_aaab_dash_123() {
        assert_eq!(pair(encoded_group, pair(dash, pair(encoded_group, dash))).parse("AAAA-AAAB-123"), Ok(("123", (0, ((), (1, ()))))));
    }
}

#[cfg(test)]
mod map {
    use super::*;

    #[test]
    fn map_aaac_dash_ab() {
        assert_eq!(pair(encoded_group, map(pair(dash, encoded_group), |(result1, result2)| result2)).parse("AAAC-AB"), Ok(("", (2, 1))));
    }

    #[test]
    fn map_aaal_dash_ab() {
        assert_eq!(pair(encoded_group, map(pair(dash, encoded_group), |(result1, result2)| result2)).parse("AAAL-AB"), Err("L-AB"));
    }
}

#[cfg(test)]
mod left_right {
    use super::*;

    #[test]
    fn map_left_aac_dash_123() {
        assert_eq!(left(encoded_group, dash).parse("AAC-123"), Ok(("123", 2)));
    }

    #[test]
    fn map_right_aac_dash() {
        assert_eq!(right(encoded_group, dash).parse("AAC-123"), Ok(("123", ())));
    }
}

#[cfg(test)]
mod one_or_more {
    use super::*;

    #[test]
    fn one_or_more_dash_dash_dash() {
        assert_eq!(one_or_more(dash).parse("---"), Ok(("", vec![(), (), ()])));
    }

    #[test]
    fn one_or_more_no_matches() {
        assert_eq!(one_or_more(dash).parse("AAA-"), Err("AAA-"));
    }
}

#[cfg(test)]
mod parse_reference_structure {
    use super::*;

    #[test]
    fn parse_reference_structure_1() {
        let reference_parser = create_reference_structure_parser();

        assert_eq!(reference_parser.parse("BA-AAAACD-AAAAAEGF"), Ok(("", (16, vec![35, 1125]))));
    }

    #[test]
    fn parse_reference_structure_2() {
        let reference_parser = create_reference_structure_parser();

        assert_eq!(reference_parser.parse("AF-AAAABA"), Ok(("", (5, vec![16]))));
    }

    #[test]
    fn parse_reference_structure_3() {
        let reference_parser = create_reference_structure_parser();

        assert_eq!(reference_parser.parse("GA-AAAABA-AAAGHCIU"), Ok(("", (96, vec![16, 422541]))));
    }
}

#[cfg(test)]
mod parse_reference {
    use super::*;

    #[test]
    fn parse_reference_1() {
        assert_eq!(parse_reference("BA-AAAACD-AAAAAEGF"), Some(Reference {
            reference_type: 16,
            company_space_id: Some(35),
            aggregate_root_id: Some(1125),
            revision: None,
            object_id: None,
        }));
    }

    #[test]
    fn parse_reference_2() {
        assert_eq!(parse_reference("AF-AAAABA"), Some(Reference {
            reference_type: 5,
            company_space_id: Some(16),
            aggregate_root_id: None,
            revision: None,
            object_id: None,
        }));
    }

    #[test]
    fn parse_reference_3() {
        assert_eq!(parse_reference("GA-AAAABA-AAAGHCIU"), Some(Reference {
            reference_type: 96,
            company_space_id: Some(16),
            aggregate_root_id: Some(422541),
            revision: None,
            object_id: None,
        }));
    }

    #[test]
    fn parse_reference_4() {
        assert_eq!(parse_reference("GA-AAAABA-AAAGHCIU-INVALID"), None);
    }

    #[test]
    fn parse_reference_5() {
        assert_eq!(parse_reference("GA-AAAABA-LAAGHCIU"), None);
    }
}