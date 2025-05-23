use phf::phf_ordered_map;

/// This list represents the mapping between package name you install and the module name you
/// import.
///
/// This is required because there is not always a programatic way to determine this mapping [[1]].
///
/// Some entries in the map also exploit the set of aliases we automatically
/// generate (e.g., databricks-sql), to improve the mapping.
///
/// If you would like to add or improve this list, please file a PR:
/// <https://github.com/lukehsiao/poetry-udeps>.
///
/// Please try to keep this list sorted lexicographically and wrapped to 79
/// columns (inclusive).
///
/// [1]: https://stackoverflow.com/a/54853084
#[rustfmt::skip]
pub static KNOWN_NAMES: phf::OrderedMap<&str, &str> = phf_ordered_map! {
    "Flask" => "flask",
    "Markdown" => "markdown",
    "PyYAML" => "yaml",
    "SQLAlchemy" => "sqlalchemy",
    "Wand" => "wand",
    "argon2-cffi" => "argon2",
    "beautifulsoup4" => "bs4",
    "boto3-stubs" => "mypy_boto3_iam",
    "celery-redbeat" => "redbeat",
    "databricks-sdk" => "databricks.sdk",
    "databricks-sql-connector" => "databricks-sql",
    "faiss-cpu" => "faiss",
    "faiss-gpu" => "faiss",
    "google-api-python-client" => "googleapiclient",
    "google-cloud-pubsub" => "google-cloud-pubsub_v1",
    "grpcio" => "grpc",
    "hydra-colorlog" => "colorlog",
    "hydra-core" => "hydra",
    "json-stream" => "json_stream",
    "jupyter" => "IPython",
    "levenshtein" => "Levenshtein",
    "opensearch-py" => "opensearchpy",
    "pdfminer.six" => "pdfminer",
    "protobuf" => "google.protobuf",
    "pyautogen" => "autogen",
    "pybars3" => "pybars",
    "python-jose" => "jose",
    "python-multipart" => "multipart",
    "python-slugify" => "slugify",
    "pytorch-nlp" => "torchnlp",
    "pyvespa" => "vespa",
    "scikit-learn" => "sklearn",
    "snowflake-connector-python" => "snowflake",
    "unicodedata2" => "unicodedata",
    "vl-convert-python" => "vl_convert",
};

#[cfg(test)]
mod tests {
    use super::KNOWN_NAMES;

    #[test]
    fn known_names_are_sorted() {
        let mut names = KNOWN_NAMES.entries().map(|(name, _alias)| name);

        let Some(mut previous_name) = names.next() else {
            return;
        };

        for name in names {
            assert!(
                name > previous_name,
                r#""{name}" should be sorted before "{previous_name}" in `KNOWN_NAMES`"#,
            );

            previous_name = name;
        }
    }
}
