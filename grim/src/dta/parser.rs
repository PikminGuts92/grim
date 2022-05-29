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

fn take_newline<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    take_while(move |c| NEWLINE_CHARACTERS.contains(&c))(text)
}

fn take_ws_or_comment<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    let mut ws_result = take_ws(text);

    while let Ok((text, _)) = ws_result {
        let comment_result = take_comment(text);

        if comment_result.is_err() {
            break;
        }

        let (t1, _) = comment_result.unwrap();
        ws_result = take_ws(t1);
    }

    ws_result

    //alt((take_ws, take_comment))(text)
}

fn take_until_newline<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    take_till(move |c| NEWLINE_CHARACTERS.contains(&c))(text)
}

fn take_comment<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    preceded(tag(COMMENT_CHARACTER), preceded(take_until_newline, take_newline))(text)
}

fn take_section<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    delimited(
        preceded(take_ws, tag(OPEN_BRACKET)),
        tag(b"test"), // alt
        preceded(take_ws, tag(CLOSE_BRACKET)),
    )(text)
}

fn take_quoted_string<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    // TODO: Better support escaped characters?
    preceded(
        tag(QUOTE_CHARACTER),
        take_till(move |c| QUOTE_CHARACTER.contains(&c))
    )(text)
}

fn take_node<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    preceded(
        take_ws_or_comment,
            // string
            // int
            // float
            // node
            delimited(
                tag(OPEN_BRACKET),
                preceded(
                    take_ws_or_comment,
                    take_while(|c| !CLOSE_BRACKET.contains(&c))
                ),
                preceded(
                    take_ws_or_comment,
                    tag(CLOSE_BRACKET)
                )
            )
    )(text)
}

pub fn parse_dta<'a>(dta: &'a[u8]) -> Result<Vec<ParsedSong>, ParseDTAError> {
    let (r1, r2) = preceded(take_ws_or_comment, tag(OPEN_BRACKET))(dta)
        .map_err(|_| ParseDTAError::UnknownDTAParseError)?;

    let r1_str = std::str::from_utf8(r1).unwrap();
    let r2_str = std::str::from_utf8(r2).unwrap();

    println!("r1: {r1_str}");
    println!("r2: {r2_str}");

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
    #[case(2, b";whatever \n (wildhoneypie)\n(temporarysecretary\n   (name \"Temporary Secretary\")\n)")] // b"(wildhoneypie)\n(temporarysecretary\n   (name \"Temporary Secretary\")\n)")
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