use nom::*;
use nom::branch::{alt};
use nom::bytes::complete::{is_not, tag, take_till, take_while, take_while1};
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1};
use nom::combinator::{map, map_res};
use nom::error::{context, Error};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::multi::{many0, separated_list0};
use super::{DataArray, ParseDTAError, RootData, DataString};

const WS_CHARACTERS: &[u8] = b" \t\r\n\x0c";
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

pub struct DTAParser<'a> {
    pub depth: usize,
    pub line_number: usize,
    pub char_index: usize,
    pub data_array: Vec<(DataArray, usize)>, // Includes char start index
    pub remaining: &'a [u8]
}

impl<'a> DTAParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            depth: 0,
            line_number: 0,
            char_index: 0,
            data_array: Vec::new(),
            remaining: data
        }
    }

    pub fn parse(mut self) {
        self.parse_array();
    }

    fn parse_array(&mut self) -> Vec<(DataArray, usize)> {
        let mut data = Vec::new();

        self.consume_whitespace();
        // TODO: Consume comments

        data
    }

    fn consume_whitespace(&mut self) {
        let mut chars_moved = 0;
        let mut lines_moved = 0;

        for c in self.remaining.iter() {
            match c {
                b'\n' => {
                    chars_moved += 1;
                    lines_moved += 1;
                },
                c if c.is_ascii_whitespace() => {
                    chars_moved += 1;
                }
                _ => {
                    break;
                }
            }
        }

        // Update self
        self.char_index += chars_moved;
        self.line_number += lines_moved;
        self.remaining = &self.remaining[chars_moved..];
    }
}

/*pub(crate) fn parse_dta_str<'a>(text: &'a [u8]) -> Vec<DataArray> {
    //let mut parse_info = 

    Vec::new()
}*/

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
    context(
        "comment",
        preceded(
            tag(COMMENT_CHARACTER),
            preceded(take_until_newline, take_newline)
        )
    )(text)
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
    context(
        "string",
        delimited(
            tag(QUOTE_CHARACTER),
            take_till(move |c| QUOTE_CHARACTER.contains(&c)),
            tag(QUOTE_CHARACTER),
        )
    )(text)
}

fn take_node<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    let (mut r1, _) = take_ws_or_comment(text)?; // Whitespace

    // Read open bracket
    let open_bracket_res: IResult<&'a [u8], &'a [u8]> = tag(OPEN_BRACKET)(r1);
    if open_bracket_res.is_ok() {
        // Nested data...
        r1 = &r1[1..];

        let (nested_r1, _) = take_node(r1)?;
        r1 = nested_r1;
    }

    // Read until close bracket
    let mut close_bracket_res: IResult<&'a [u8], &'a [u8]>;

    loop {
        close_bracket_res = tag(CLOSE_BRACKET)(r1);

        if r1.is_empty() {
            break;
        } else if close_bracket_res.is_ok() {
            r1 = &r1[1..];
            break;
        }


    }

    /*while let Ok((nested_r1, _)) = close_bracket_res {
        /*let comment_result = take_comment(text);

        if comment_result.is_err() {
            break;
        }

        let (t1, _) = comment_result.unwrap();
        ws_result = take_ws(t1);*/
    }*/

    /*let tt: IResult<&'a [u8], &'a [u8]> = alt((
        map(take_quoted_string, |_| (r1, r2))
    ))(r1);*/

    //return Ok((r1, r2));

    //return tt;

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

fn take_root_node<'a>(text: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
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

fn parse_string<'a>(text: &'a [u8]) -> IResult<&'a [u8], DataArray> {
    delimited(
        tag(b"\""),
        map(
            take_while(|c: u8| c.ne(&b"\""[0])),
            |s: &'a [u8]| DataArray::String(DataString::from_vec(s.to_owned()))
        ),
        tag(b"\"")
    )(text)
}

fn parse_symbol<'a>(text: &'a [u8]) -> IResult<&'a [u8], DataArray> {
    // TODO: Suport wider range of chars instead of alphanumeric
    map(
        alt((
            tuple((
                alpha1,
                alphanumeric0
            )),
            delimited(
                tag(b"'"),
                tuple((
                    alpha1,
                    alphanumeric0
                )),
                tag(b"'")
            )
        )),
        |(pre, post): (&'a [u8], &'a [u8])| DataArray::Symbol(DataString::from_vec(
            pre
                .iter()
                .chain(post.iter())
                .map(|c| *c)
                .collect()
        ))
    )(text)
}

fn parse_data_array<'a>(text: &'a [u8]) -> IResult<&'a [u8], Vec<DataArray>> {
    many0(
        preceded(
            take_ws_or_comment,
            alt((
                // String
                parse_string,
                // Symbol
                parse_symbol,
                // Array
                delimited(
                    tag(OPEN_BRACKET),
                    map(
                        parse_data_array,
                        |items: Vec<DataArray>| DataArray::Array(items)
                    ),
                    preceded(
                        take_ws_or_comment,
                        tag(CLOSE_BRACKET)
                    )
                )
            ))
        )
    )(text)

    /*preceded(
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
    )(text)*/
}

pub fn parse_dta<'a>(dta: &'a[u8]) -> Result<Vec<ParsedSong>, ParseDTAError> {
    let (r1, r2) = take_root_node(dta)
        .map_err(|_| ParseDTAError::UnknownDTAParseError)?;

    let r1_str = std::str::from_utf8(r1).unwrap();
    let r2_str = std::str::from_utf8(r2).unwrap();

    println!("r1: {r1_str}");
    println!("r2: {r2_str}");

    let (_, items) = parse_data_array(dta).unwrap();

    println!("\n\n{}", String::from_utf8(dta.to_owned()).unwrap());
    println!("{items:#?}");

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
    #[case(b";whatever \n (wildhoneypie)\n(temporarysecretary\n   (name \"Temporary Secretary\")\n)", 2)] // b"(wildhoneypie)\n(temporarysecretary\n   (name \"Temporary Secretary\")\n)")
    fn parse_dta_test<const N: usize>(#[case] dta: &[u8; N], #[case] expected_count: usize) {
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