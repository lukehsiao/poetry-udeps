use phf::phf_map;

/// This list represents the mapping between package name you install and the module name you
/// import.
///
/// This is required because there is not always a programatic way to determine this mapping [[1]].
///
/// If you would like to add or improve this list, please file a PR:
/// <https://github.com/lukehsiao/poetry-udeps>.
///
/// Please try to keep this list sorted lexicographically and wrapped to 79
/// columns (inclusive).
///
/// [1]: https://stackoverflow.com/a/54853084
#[rustfmt::skip]
pub static KNOWN_NAMES: phf::Map<&str, &str> = phf_map! {
    "argon2-cffi" => "argon2",
};

#[cfg(test)]
mod tests {
    use super::KNOWN_NAMES;

    #[test]
    fn known_names_are_sorted() {
        let mut names = KNOWN_NAMES.entries().map(|(name, alias)| name);

        let Some(mut previous_name) = names.next() else { return; };

        for name in names {
            assert!(
                name > previous_name,
                r#""{}" should be sorted before "{}" in `KNOWN_NAMES`"#,
                name,
                previous_name
            );

            previous_name = name;
        }
    }
}
