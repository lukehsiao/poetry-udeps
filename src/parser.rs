use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{alpha1, alphanumeric1, anychar, char, space1},
    combinator::{all_consuming, map, recognize, value},
    multi::{many0, many0_count},
    sequence::{pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct ImportStatement {
    pub package: String,
    pub module: String,
}

/// Parsing identifiers that may start with a letter (or underscore) and may contain underscores,
/// letters, numbers, and periods.
fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"), tag(".")))),
    ))(input)
}

fn from_package_import(input: &str) -> IResult<&str, ImportStatement> {
    let (input, (_, _, package, _, _, _, module)) = tuple((
        tag("from"),
        space1,
        identifier,
        space1,
        tag("import"),
        space1,
        identifier,
    ))(input)?;
    let statement = ImportStatement {
        module: module.to_owned(),
        package: package.to_owned(),
    };
    Ok((input, statement))
}

fn simple_import(input: &str) -> IResult<&str, ImportStatement> {
    let (input, (_, _, package)) = tuple((tag("import"), space1, identifier))(input)?;
    let statement = ImportStatement {
        module: String::new(),
        package: package.to_owned(),
    };
    Ok((input, statement))
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

fn parse_block(input: &str) -> IResult<&str, Option<ImportStatement>> {
    alt((
        map(from_package_import, Some),
        map(simple_import, Some),
        map(inline_comment, |()| None),
        map(multiline_comment, |()| None),
        // Consume everything else
        map(anychar, |_| None),
    ))(input)
}

fn parse_file(input: &str) -> IResult<&str, Vec<ImportStatement>> {
    let (i, v) = all_consuming(many0(parse_block))(input)?;
    Ok((i, v.into_iter().flatten().collect()))
}

pub fn parse_python_file(input: &str) -> Result<Vec<ImportStatement>> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    let (_, v) = parse_file(input).map_err(|e| e.to_owned())?;
    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_identifier() {
        assert_eq!(
            identifier("_something_for_nothing"),
            Ok(("", "_something_for_nothing"))
        );
        assert_eq!(identifier("argon2"), Ok(("", "argon2")));
        assert_eq!(identifier("pprint"), Ok(("", "pprint")));
        assert_eq!(identifier("google.cloud"), Ok(("", "google.cloud")));
    }
    #[test]
    fn test_simple_import() {
        assert_eq!(simple_import("import numpy").unwrap().1.package, "numpy");
        assert_eq!(
            simple_import("import google.cloud").unwrap().1.package,
            "google.cloud"
        );
        assert_eq!(
            simple_import("import snowflake.connector")
                .unwrap()
                .1
                .package,
            "snowflake.connector"
        );
    }
    #[test]
    fn test_simple_from() {
        let import = from_package_import("from google.cloud import bigquery")
            .unwrap()
            .1;
        assert_eq!(import.package, "google.cloud");
        assert_eq!(import.module, "bigquery");
        let import = from_package_import("from pprint import pprint").unwrap().1;
        assert_eq!(import.package, "pprint");
        assert_eq!(import.module, "pprint");
        let import = from_package_import(
            "from torchnlp.encoders.text.default_reserved_tokens import DEFAULT_COPY_TOKEN",
        )
        .unwrap()
        .1;
        assert_eq!(
            import.package,
            "torchnlp.encoders.text.default_reserved_tokens"
        );
        assert_eq!(import.module, "DEFAULT_COPY_TOKEN");
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
        let imports = parse_file(file).unwrap().1;
        assert_eq!(
            imports.iter().map(|i| &i.package).collect::<Vec<_>>(),
            [
                "pprint",
                "numpy",
                "torchnlp.encoders.text.default_reserved_tokens"
            ]
        );
        assert_eq!(
            imports.iter().map(|i| &i.module).collect::<Vec<_>>(),
            ["pprint", "", "DEFAULT_COPY_TOKEN"]
        );
    }
}
