use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileKind {
    ModRs,
    LintGroupRs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleKind {
    Expr,   // For insert_expr_rule!
    Struct, // For insert_struct_rule!
}

#[derive(Debug, Clone)]
struct LinterInfo {
    name: String,
    kind: u8,                    // 0: use super, 1: insert_*_rule!, 2: mod, 3: pub use
    rule_kind: Option<RuleKind>, // Only Some for kind 1 (insert_*_rule!)
    file_kind: FileKind,
    normalized_name: String,
}

// Normalize linter name (convert to lowercase and handle special cases)
fn normalize_linter_name(name: &str) -> String {
    name.chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                vec![' ', c.to_ascii_lowercase()]
            } else if c == '_' {
                vec![' ']
            } else {
                vec![c]
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

pub fn check_linters() -> anyhow::Result<()> {
    let files_to_check = [
        "harper-core/src/linting/mod.rs",
        "harper-core/src/linting/lint_group.rs",
    ];

    let mut all_linters = Vec::new();

    for file in &files_to_check {
        let path = Path::new(file);
        let mut file_linters = extract_linters_from_file(path)?;
        all_linters.append(&mut file_linters);
    }

    // Analyze occurrences
    let occurrences = analyze_linter_occurrences(&all_linters);

    // Report any linters that don't appear exactly 4 times
    for (name, (count, refs)) in &occurrences {
        if *count != 4 {
            println!(
                "Warning: Linter '{}' appears {} times instead of 4",
                name, count
            );
            for linter in refs {
                let kind_desc = match (linter.kind, linter.rule_kind) {
                    (0, _) => "use super".to_string(),
                    (1, Some(RuleKind::Expr)) => "insert_expr_rule!".to_string(),
                    (1, Some(RuleKind::Struct)) => "insert_struct_rule!".to_string(),
                    (2, _) => "mod".to_string(),
                    (3, _) => "pub use".to_string(),
                    _ => "unknown".to_string(),
                };
                println!(
                    "  - In {:?} as {} '{}'",
                    linter.file_kind, kind_desc, linter.name
                );
            }
        }
    }

    // Print summary
    println!("\nLinter Summary:");
    println!(
        "{:<30} {:>8} {:>8} {:>8} {:>8} {:>8}",
        "Linter", "Super", "Expr", "Struct", "Mod", "Pub Use"
    );
    println!("{}", "-".repeat(75));

    let mut sorted: Vec<_> = occurrences.into_iter().collect();
    sorted.sort_by_key(|(name, _)| name.clone());

    for (name, (_, refs)) in sorted {
        let mut counts = [0; 5]; // 0: use, 1: expr, 2: struct, 3: mod, 4: pub use
        for linter in refs {
            match (linter.kind, linter.rule_kind) {
                (0, _) => counts[0] += 1,                      // use super
                (1, Some(RuleKind::Expr)) => counts[1] += 1,   // insert_expr_rule!
                (1, Some(RuleKind::Struct)) => counts[2] += 1, // insert_struct_rule!
                (2, _) => counts[3] += 1,                      // mod
                (3, _) => counts[4] += 1,                      // pub use
                _ => {}
            }
        }
        let normal = counts[0] + counts[1] + counts[2] + counts[3] + counts[4]  == 4;
        let (pre, post) = if !normal {
            ("\x1b[31m", "\x1b[0m")
        } else {
            ("", "")
        };
        if !normal {
            println!(
                "{pre}{:<30} {:>8} {:>8} {:>8} {:>8} {:>8}{post}",
                name, counts[0], counts[1], counts[2], counts[3], counts[4]
            );
        }
    }

    Ok(())
}

fn extract_linters_from_file(path: &Path) -> anyhow::Result<Vec<LinterInfo>> {
    let file_kind = if path.ends_with("mod.rs") {
        FileKind::ModRs
    } else if path.ends_with("lint_group.rs") {
        FileKind::LintGroupRs
    } else {
        anyhow::bail!("Unknown file kind: {}", path.display());
    };

    let content = std::fs::read_to_string(path)?;
    let mut linters = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        let _line_display = format!("{}:{}", path.display(), line_num + 1);

        if line.starts_with("use super::") {
            if let Some(after_prefix) = line.strip_prefix("use super::") {
                if let Some(before_semi) = after_prefix.strip_suffix(';') {
                    let parts: Vec<&str> = before_semi.split("::").collect();
                    if let Some(&last_part) = parts.last() {
                        if let Some(first_char) = last_part.chars().next() {
                            let is_upper = first_char.is_ascii_uppercase();
                            if is_upper && parts.len() >= 2 {
                                // The module name should be the second-to-last part
                                let module_name = parts[parts.len() - 2];
                                // Convert the linter name to snake_case
                                let expected_module_name = last_part
                                    .chars()
                                    .flat_map(|c| {
                                        if c.is_uppercase() {
                                            vec!['_', c.to_ascii_lowercase()]
                                        } else {
                                            vec![c]
                                        }
                                    })
                                    .collect::<String>()
                                    .trim_start_matches('_')
                                    .to_string();

                                if module_name == expected_module_name {
                                    linters.push(LinterInfo {
                                        name: last_part.to_string(),
                                        kind: 0, // use super
                                        rule_kind: None,
                                        file_kind,
                                        normalized_name: normalize_linter_name(last_part),
                                    });
                                } else {
                                    eprintln!(
                                        "Warning: Module name '{}' doesn't match linter name '{}' (expected '{}')",
                                        module_name, last_part, expected_module_name
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        // Match: insert_expr_rule!(LinterName, true/false);
        // Match: insert_struct_rule!(LinterName, true/false);
        else if line.contains("insert_") && line.contains("_rule!(") {
            let (rule_kind, after_prefix) = if let Some(s) = line.strip_prefix("insert_expr_rule!(")
            {
                (Some(RuleKind::Expr), s)
            } else if let Some(s) = line.strip_prefix("insert_struct_rule!(") {
                (Some(RuleKind::Struct), s)
            } else {
                (None, "")
            };

            if let Some(rule_kind) = rule_kind {
                let linter_name = after_prefix.split(',').next().unwrap_or("").trim();
                if !linter_name.is_empty() {
                    linters.push(LinterInfo {
                        name: linter_name.to_string(),
                        kind: 1, // insert_*_rule!
                        rule_kind: Some(rule_kind),
                        file_kind,
                        normalized_name: normalize_linter_name(linter_name),
                    });
                }
            }
        }
        // Match: mod linter_name;
        else if let Some(linter) = line
            .strip_prefix("mod ")
            .and_then(|s| s.strip_suffix(';'))
            .filter(|s| s.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
        {
            linters.push(LinterInfo {
                name: linter.to_string(),
                kind: 2, // mod
                rule_kind: None,
                file_kind,
                normalized_name: normalize_linter_name(linter),
            });
        }
        // Match: pub use module_name::LinterName;
        else if let Some(after_prefix) = line.strip_prefix("pub use ") {
            if let Some(before_semi) = after_prefix.strip_suffix(';') {
                let parts: Vec<&str> = before_semi.split("::").collect();
                if let Some(&last_part) = parts.last() {
                    if let Some(first_char) = last_part.chars().next() {
                        if first_char.is_ascii_uppercase() && parts.len() >= 2 {
                            // The module name should be the second-to-last part
                            let module_name = parts[parts.len() - 2];
                            // Convert the linter name to snake_case
                            let expected_module_name = last_part
                                .chars()
                                .flat_map(|c| {
                                    if c.is_uppercase() {
                                        vec!['_', c.to_ascii_lowercase()]
                                    } else {
                                        vec![c]
                                    }
                                })
                                .collect::<String>()
                                .trim_start_matches('_')
                                .to_string();

                            if module_name == expected_module_name {
                                linters.push(LinterInfo {
                                    name: last_part.to_string(),
                                    kind: 3, // pub use
                                    rule_kind: None,
                                    file_kind,
                                    normalized_name: normalize_linter_name(last_part),
                                });
                            } else {
                                eprintln!(
                                    "Warning: In pub use, module name '{}' doesn't match linter name '{}' (expected '{}')",
                                    module_name, last_part, expected_module_name
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // Debug output
    eprintln!("\nFound {} linters in {}:", linters.len(), path.display());
    for (i, linter) in linters.iter().enumerate() {
        let emoji = match (linter.kind, linter.rule_kind) {
            (0, _) => "🦸",                      // use super
            (1, Some(RuleKind::Expr)) => "📏💬",   // insert_expr_rule!
            (1, Some(RuleKind::Struct)) => "📏🧩", // insert_struct_rule!
            (2, _) => "🧱",                      // mod
            (3, _) => "🍻",                      // pub use
            _ => "❓",
        };
        eprintln!(
            "  {:4} {} '{}' (normalized: '{}')",
            i, emoji, linter.name, linter.normalized_name
        );
    }

    Ok(linters)
}

fn analyze_linter_occurrences(
    linters: &[LinterInfo],
) -> HashMap<String, (usize, Vec<&LinterInfo>)> {
    let mut occurrences: HashMap<String, (usize, Vec<&LinterInfo>)> = HashMap::new();

    for linter in linters {
        let entry = occurrences
            .entry(linter.normalized_name.clone())
            .or_insert((0, Vec::new()));
        entry.0 += 1;
        entry.1.push(linter);
    }

    occurrences
}
