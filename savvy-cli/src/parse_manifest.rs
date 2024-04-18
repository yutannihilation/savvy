use std::path::Path;

use crate::canonicalize;

pub(crate) struct Manifest {
    pub(crate) crate_name: String,
    pub(crate) dependencies: toml::Table,
}

impl Manifest {
    pub(crate) fn new(path: &Path, features: &[String]) -> Self {
        Self::from_str(
            &savvy_bindgen::read_file(path),
            path.parent().unwrap_or(Path::new(".")),
            features,
        )
    }

    pub(crate) fn from_str(content: &str, base_dir: &Path, features: &[String]) -> Self {
        let mut parsed = content
            .parse::<toml::Table>()
            .expect("Failed to parse Cargo.toml");

        let crate_name = parsed
            .get("package")
            .expect("Cargo.toml doesn't have a [package] section")
            .get("name")
            .expect("Cargo.toml doesn't have the `name` key in the [package] section")
            .as_str()
            .expect("Cargo.toml have an invalid `name` key in the [package] section")
            .to_string();

        let deps = parsed.get_mut("dependencies").map(|d| {
            d.as_table()
                .expect("Cargo.toml has an invalid [dependencies] section")
                .clone()
        });
        let dev_deps = parsed.get_mut("dev-dependencies").map(|d| {
            d.as_table()
                .expect("Cargo.toml has an invalid [dev-dependencies] section")
                .clone()
        });

        let mut dependency_list = match (deps, dev_deps) {
            (None, None) => toml::Table::new(),
            (None, Some(dev_deps)) => dev_deps,
            (Some(deps), None) => deps,
            (Some(mut deps), Some(dev_deps)) => {
                // overwrite the value if it's specified in [dev-dependencies]
                dev_deps.into_iter().for_each(|(k, v)| {
                    deps.insert(k, v);
                });
                deps
            }
        };

        // Add savvy as the dependency

        if crate_name != "savvy" {
            dependency_list.insert("savvy".to_string(), toml::Value::String("*".to_string()));
        }

        // Add the crate itself as a dependency

        let mut self_crate_spec = toml::Table::new();
        let self_path = canonicalize(base_dir).expect("Failed to canonicalize path");
        self_crate_spec.insert("path".to_string(), toml::Value::String(self_path));

        if !features.is_empty() {
            self_crate_spec.insert(
                "features".to_string(),
                toml::Value::Array(
                    features
                        .iter()
                        .map(|f| toml::Value::String(f.clone()))
                        .collect::<Vec<toml::Value>>(),
                ),
            );
        }

        dependency_list.insert(crate_name.clone(), toml::Value::Table(self_crate_spec));

        // Tweak path dependencies
        for (_, v) in dependency_list.iter_mut() {
            if let toml::Value::Table(ref mut spec) = v {
                if let Some(toml::Value::String(ref mut path)) = spec.get_mut("path") {
                    *path =
                        canonicalize(&base_dir.join(&path)).expect("Failed to canonicalize path");
                }
            }
        }

        let mut dependencies = toml::Table::new();
        dependencies.insert(
            "dependencies".to_string(),
            toml::Value::Table(dependency_list),
        );

        Self {
            crate_name,
            dependencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use toml::Table;

    use super::*;

    fn assert_manifest(
        actual: &str,
        base_dir: &Path,
        expected_crate_name: &str,
        expected_dependencies: toml::Table,
    ) {
        let actual = Manifest::from_str(actual, base_dir, &[]);
        assert_eq!(actual.crate_name, expected_crate_name);
        assert_eq!(actual.dependencies, expected_dependencies);
    }

    #[test]
    fn test_parse_manifest() {
        assert_manifest(
            r#"
[package]
name = "test"

[dependencies]
dep1 = "1.2.3"
        "#,
            Path::new("."),
            "test",
            {
                let mut expected = Table::new();
                expected.insert("dep1".to_string(), toml::Value::String("1.2.3".to_string()));
                expected
            },
        );

        assert_manifest(
            r#"
[package]
name = "test"

[dependencies]
dep1 = "1.2.3"

[dev-dependencies]
dep2 = "4.5.6"
        "#,
            Path::new("."),
            "test",
            {
                let mut expected = Table::new();
                expected.insert("dep1".to_string(), toml::Value::String("1.2.3".to_string()));
                expected.insert("dep2".to_string(), toml::Value::String("4.5.6".to_string()));
                expected
            },
        );

        assert_manifest(
            r#"
[package]
name = "test"

[dependencies]
dep1 = "1.2.3"
dep2 = { path = "./savvy-cli/src" }
        "#,
            Path::new("../"),
            "test",
            {
                let mut expected = Table::new();
                expected.insert("dep1".to_string(), toml::Value::String("1.2.3".to_string()));
                let mut tbl2 = Table::new();
                tbl2.insert(
                    "path".to_string(),
                    toml::Value::String(canonicalize(Path::new("./src")).unwrap()),
                );
                expected.insert("dep2".to_string(), toml::Value::Table(tbl2));
                expected
            },
        );
    }
}
