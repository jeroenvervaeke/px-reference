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