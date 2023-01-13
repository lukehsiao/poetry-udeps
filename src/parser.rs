use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{alpha1, alphanumeric1, anychar, char, multispace0, multispace1, space1},
    combinator::{all_consuming, map, recognize, value},
    multi::{many0, many0_count},
    sequence::{pair, tuple},
    IResult,
};

/// Parsing identifiers that may start with a letter (or underscore) and may contain underscores,
/// letters, and numbers.
fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

/// Consume an entire import line, extracting the package
fn parse_import(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("import ")(input)?;
    let (input, package) = identifier(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = multispace1(input)?;

    Ok((input, package))
}

/// Consume the important bits of a from block, extracting the package
///
/// Specifically, we also consume the "import", so we don't get confused.
fn parse_from(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("from ")(input)?;
    let (input, package) = identifier(input)?;
    let (input, _) = many0(pair(tag("."), identifier))(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("import")(input)?;
    let (input, _) = multispace1(input)?;

    Ok((input, package))
}

fn inline_comment(input: &str) -> IResult<&str, ()> {
    value((), pair(char('#'), is_not("\n\r")))(input)
}

fn multiline_comment(input: &str) -> IResult<&str, ()> {
    value(
        (),
        tuple((tag("\"\"\""), take_until("\"\"\""), tag("\"\"\""))),
    )(input)
}
fn parse_block(input: &str) -> IResult<&str, Option<&str>> {
    alt((
        map(parse_from, Some),
        map(inline_comment, |_| None),
        map(parse_import, Some),
        map(multiline_comment, |_| None),
        // Consume everything else
        map(anychar, |_| None),
    ))(input)
}

fn parse_file(input: &str) -> IResult<&str, Vec<&str>> {
    let (i, v) = all_consuming(many0(parse_block))(input)?;
    Ok((i, v.into_iter().flatten().collect()))
}

pub fn parse_python_file(input: &str) -> Result<Vec<&str>> {
    let (_, v) = parse_file(input).map_err(|e| e.to_owned())?;
    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_identifier() {
        assert_eq!(
            identifier("_something_for_nothing"),
            Ok(("", "_something_for_nothing"))
        );
        assert_eq!(identifier("argon2"), Ok(("", "argon2")));
        assert_eq!(identifier("pprint"), Ok(("", "pprint")));
        assert_eq!(identifier("google.cloud"), Ok((".cloud", "google")));
    }
    #[test]
    fn test_simple_import() {
        assert_eq!(parse_import("import numpy as np\n").unwrap().1, "numpy");
        assert_eq!(parse_import("import google.cloud\n").unwrap().1, "google");
    }
    #[test]
    fn test_simple_from() {
        assert_eq!(
            parse_from("from google.cloud import bigquery").unwrap().1,
            "google"
        );
        assert_eq!(parse_from("from pprint import pprint").unwrap().1, "pprint");
        assert_eq!(
            parse_from(
                "from torchnlp.encoders.text.default_reserved_tokens import DEFAULT_COPY_TOKEN"
            )
            .unwrap()
            .1,
            "torchnlp"
        );
    }
    #[test]
    fn test_multiline_comment() {
        assert_eq!(
            multiline_comment(r#""""from google.cloud import bigquery""""#),
            Ok(("", ()))
        );
        assert_eq!(
            multiline_comment(
                r#""""Some function docstring.

    Some docstrings are really long.

    Args:
        something: a list of stuff

    Example:
        import pprint
        pprint("hello world")
    """"#
            ),
            Ok(("", ()))
        );
    }
    #[test]
    fn test_inline_comment() {
        assert_eq!(inline_comment("# something else"), Ok(("", ())));
        assert_eq!(inline_comment("# ##### other stuff"), Ok(("", ())));
    }

    #[test]
    fn test_parse_file() {
        let file = r#"
from pprint import pprint
import numpy as np
from torchnlp.encoders.text.default_reserved_tokens import DEFAULT_COPY_TOKEN

def run() -> None:
    pprint(np.ones(10))

if __name__ == "__main__":
    run()

        "#;
        assert_eq!(
            parse_file(file).unwrap().1,
            vec!["pprint", "numpy", "torchnlp"]
        );
    }
}
