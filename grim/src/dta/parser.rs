use nom::*;
use nom::branch::{alt};
use nom::bytes::complete::{is_not, tag, take_till, take_while};
use nom::combinator::{map};
use nom::error::Error;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::multi::{many0};
use super::ParseDTAError;

const WS_CHARACTERS: &[u8] = b" \t\r\n";
const SPACE_CHARACTERS: &[u8] = b" \t";
const NEWLINE_CHARACTERS: &[u8] = b"\r\n";

const OPEN_BRACKET: &[u8] = b"(";
const CLOSE_BRACKET:&[u8] = b")";

const QUOTE_CHARACTER: &[u8] = b"\"";
const COMMENT_CHARACTER: &[u8] = b";";

pub struct ParsedSong<'a> {
    pub start: usize,
    pub size: usize,
    pub data: &'a [u8],
}

fn take_ws<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    take_while(move |c| WS_CHARACTERS.contains(&c))(text)
}

fn take_until_newline<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    take_till(move |c| NEWLINE_CHARACTERS.contains(&c))(text)
}

/*fn take_comment<'a>(test: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {

}*/

fn take_section<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    delimited(
        preceded(take_ws, tag(OPEN_BRACKET)),
        tag(b"test"), // alt
        preceded(take_ws, tag(CLOSE_BRACKET)),
    )(text)
}

pub fn parse_dta<'a>(dta: &'a[u8]) -> Result<Vec<ParsedSong>, ParseDTAError> {
    let (r1, r2) = preceded(take_ws, tag(OPEN_BRACKET))(dta)
        .map_err(|_| ParseDTAError::UnknownDTAParseError)?;

    //let result = take_section(dta).map_err(|_| ParseDTAError::UnknownDTAParseError)?;

    /*let result = map(
        many0(take_section),
        |songs| songs
    )(dta)
        .map_err(|_| ParseDTAError::UnknownDTAParseError)?;*/

    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    //const DTA_PATH: Option<&str> = option_env!("GRIM_TEST_DTA_PATH");

    #[rstest]
    #[case(2, b"(wildhoneypie)\n(temporarysecretary\n   (name \"Temporary Secretary\")\n)")] // b"(wildhoneypie)\n(temporarysecretary\n   (name \"Temporary Secretary\")\n)")
    fn parse_dta_test<const N: usize>(#[case] expected_count: usize, #[case] dta: &[u8; N]) {
        //let dta_path = DTA_PATH;
        /*match DTA_PATH {
            Some(dta_path) => println!("DTA path is \"{dta_path}\""),
            None => println!("DTA path not found!"),
        };*/

        //let dta_str = std::str::from_utf8(dta).unwrap();

        let result = parse_dta(dta);
        assert!(result.is_ok());

        if let Ok(songs) = result {
            assert_eq!(expected_count, songs.len());
        }
    }
}