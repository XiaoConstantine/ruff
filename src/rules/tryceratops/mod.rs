//! Rules from [tryceratops](https://pypi.org/project/tryceratops/1.1.0/).
pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::convert::AsRef;
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::linter::test_path;
    use crate::registry::Rule;
    use crate::settings;

    #[test_case(Rule::PreferTypeError, Path::new("TRY004.py"); "TRY004")]
    #[test_case(Rule::ReraiseNoCause, Path::new("TRY200.py"); "TRY200")]
    #[test_case(Rule::VerboseRaise, Path::new("TRY201.py"); "TRY201")]
    #[test_case(Rule::TryConsiderElse, Path::new("TRY300.py"); "TRY300")]
    #[test_case(Rule::RaiseWithinTry , Path::new("TRY301.py"); "TRY301")]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.as_ref(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("./resources/test/fixtures/tryceratops")
                .join(path)
                .as_path(),
            &settings::Settings::for_rule(rule_code),
        )?;
        insta::assert_yaml_snapshot!(snapshot, diagnostics);
        Ok(())
    }
}
