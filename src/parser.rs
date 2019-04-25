const CHAR_CODE_A: u32 = 'A' as u32;
const CHAR_CODE_R: u32 = 'R' as u32;
const CHAR_R_OFFSET: u32 = CHAR_CODE_R - 10;

fn dash(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('-') => Ok((&input['-'.len_utf8()..], ())),
        _ => Err(input),
    }
}

fn encoded_group(input: &str) -> Result<(&str, u32), &str> {
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

fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
    where P1: Fn(&str) -> Result<(&str, R1), &str>,
          P2: Fn(&str) -> Result<(&str, R2), &str>,
{
    move |input: &str| match parser1(input) {
        Ok((next_input, result1)) => match parser2(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&str) -> Result<(&str, B), &str>
    where
        P: Fn(&str) -> Result<(&str, A), &str>,
        F: Fn(A) -> B
{
    move |input: &str| match parser(input) {
        Ok((next_input, result)) => Ok((next_input, map_fn(result))),
        Err(err) => Err(err),
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
        assert_eq!(pair(dash, dash)("--"), Ok(("", ((), ()))));
    }

    #[test]
    fn pair_aaaa_dash_aaab_dash_123() {
        assert_eq!(pair(encoded_group, pair(dash, pair(encoded_group, dash)))("AAAA-AAAB-123"), Ok(("123", (0, ((), (1, ()))))));
    }
}

#[cfg(test)]
mod map {
    use super::*;

    #[test]
    fn map_aaac_dash_ab() {
        assert_eq!(pair(encoded_group, map(pair(dash, encoded_group), |(result1, result2)| result2))("AAAC-AB"), Ok(("", (2, 1))));
    }

    #[test]
    fn map_aaal_dash_ab() {
        assert_eq!(pair(encoded_group, map(pair(dash, encoded_group), |(result1, result2)| result2))("AAAL-AB"), Err("L-AB"));
    }
}