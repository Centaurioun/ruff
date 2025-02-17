//! Rules from [flake8-pie](https://pypi.org/project/flake8-pie/).
mod fixes;
pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use insta::assert_yaml_snapshot;
    use test_case::test_case;

    use crate::registry::Rule;
    use crate::settings;
    use crate::test::test_path;

    #[test_case(Rule::DupeClassFieldDefinitions, Path::new("PIE794.py"); "PIE794")]
    #[test_case(Rule::UnnecessaryDictKwargs, Path::new("PIE804.py"); "PIE804")]
    #[test_case(Rule::SingleStartsEndsWith, Path::new("PIE810.py"); "PIE810")]
    #[test_case(Rule::UnnecessaryPass, Path::new("PIE790.py"); "PIE790")]
    #[test_case(Rule::UnnecessarySpread, Path::new("PIE800.py"); "PIE800")]
    #[test_case(Rule::PreferListBuiltin, Path::new("PIE807.py"); "PIE807")]
    #[test_case(Rule::PreferUniqueEnums, Path::new("PIE796.py"); "PIE796")]
    #[test_case(Rule::UnnecessaryComprehensionAnyAll, Path::new("PIE802.py"); "PIE802")]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.noqa_code(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("flake8_pie").join(path).as_path(),
            &settings::Settings::for_rule(rule_code),
        )?;
        assert_yaml_snapshot!(snapshot, diagnostics);
        Ok(())
    }
}
