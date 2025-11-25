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
    kind: u8, // 0: use super, 1: insert_*_rule!, 2: mod, 3: pub use
    rule_kind: Option<RuleKind>,
    file_kind: FileKind,
    normalized_name: String,
    source_type: SourceType, // Changed from source_file_exists
    implementation: Option<LinterImpl>,
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
        let normal = counts[0] + counts[1] + counts[2] + counts[3] + counts[4] == 4;
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

    // After processing all files
    let missing_sources: Vec<_> = all_linters
        .iter()
        // .filter(|l| !l.source_file_exists)
        .filter(|l| l.source_type == SourceType::Missing)
        .map(|l| l.name.as_str())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    if !missing_sources.is_empty() {
        println!(
            "\n\x1b[31mWarning: {} linters have missing source files:\x1b[0m",
            missing_sources.len()
        );
        for name in missing_sources {
            println!("  - {}", name);
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

    let base_path = path.parent().unwrap().parent().unwrap(); // Go up to harper-core/src

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        let _line_display = format!("{}:{}", path.display(), line_num + 1);

        if line.starts_with("use super::")
            && let Some(after_prefix) = line.strip_prefix("use super::")
            && let Some(before_semi) = after_prefix.strip_suffix(';')
        {
            let parts: Vec<&str> = before_semi.split("::").collect();
            if let Some(&last_part) = parts.last()
                && let Some(first_char) = last_part.chars().next()
            {
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
                        let (source_type, implementation) =
                            check_linter_source_exists(last_part, base_path);
                        linters.push(LinterInfo {
                            name: last_part.to_string(),
                            kind: 0, // use super
                            rule_kind: None,
                            file_kind,
                            normalized_name: normalize_linter_name(last_part),
                            source_type, // Changed from source_file_exists
                            implementation,
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
                    let (source_type, implementation) =
                        check_linter_source_exists(linter_name, base_path);
                    linters.push(LinterInfo {
                        name: linter_name.to_string(),
                        kind: 1, // or appropriate kind
                        rule_kind: Some(rule_kind),
                        file_kind,
                        normalized_name: normalize_linter_name(linter_name),
                        source_type, // Changed from source_file_exists
                        implementation,
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
            let (source_type, implementation) = check_linter_source_exists(linter, base_path);
            linters.push(LinterInfo {
                name: linter.to_string(),
                kind: 2, // mod
                rule_kind: None,
                file_kind,
                normalized_name: normalize_linter_name(linter),
                source_type,
                implementation,
            });
        }
        // Match: pub use module_name::LinterName;
        else if let Some(after_prefix) = line.strip_prefix("pub use ")
            && let Some(before_semi) = after_prefix.strip_suffix(';')
        {
            let parts: Vec<&str> = before_semi.split("::").collect();
            if let Some(&last_part) = parts.last()
                && let Some(first_char) = last_part.chars().next()
                && first_char.is_ascii_uppercase()
                && parts.len() >= 2
            {
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
                    let (source_type, implementation) =
                        check_linter_source_exists(last_part, base_path);
                    linters.push(LinterInfo {
                        name: last_part.to_string(),
                        kind: 3, // pub use
                        rule_kind: None,
                        file_kind,
                        normalized_name: normalize_linter_name(last_part),
                        source_type,
                        implementation,
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

    // Debug output
    eprintln!("\nFound {} linters in {}:", linters.len(), path.display());
    for (i, linter) in linters.iter().enumerate() {
        let emoji = match (linter.kind, linter.rule_kind) {
            (0, _) => "🦸",                        // use super
            (1, Some(RuleKind::Expr)) => "📏💬",   // insert_expr_rule!
            (1, Some(RuleKind::Struct)) => "📏🧩", // insert_struct_rule!
            (2, _) => "🧱",                        // mod
            (3, _) => "🍻",                        // pub use
            _ => "❓",
        };

        let warn = if (linter.rule_kind == Some(RuleKind::Expr)
            && linter.implementation == Some(LinterImpl::Linter))
            || (linter.rule_kind == Some(RuleKind::Struct)
                && linter.implementation == Some(LinterImpl::ExprLinter))
            || linter.source_type == SourceType::Both
        // Add this line to warn about both file and dir
        {
            " ❌"
        } else {
            ""
        };

        let source_info = match linter.source_type {
            SourceType::Missing => "\x1b[31m[missing]\x1b[0m".to_string(),
            SourceType::File => "\x1b[32m[file]\x1b[0m".to_string(),
            SourceType::Dir => "\x1b[34m[dir]\x1b[0m".to_string(),
            SourceType::Both => "\x1b[33m[file+dir]\x1b[0m".to_string(),
        };

        let impl_info = match linter.implementation {
            Some(LinterImpl::Linter) => "\x1b[34m[Linter]\x1b[0m",
            Some(LinterImpl::ExprLinter) => "\x1b[35m[ExprLinter]\x1b[0m",
            Some(LinterImpl::Both) => "\x1b[36m[Linter+ExprLinter]\x1b[0m",
            Some(LinterImpl::Neither) => "\x1b[33m[No impl found]\x1b[0m",
            None => "",
        };

        eprintln!(
            "  {:4} {} '{}' (normalized: '{}') {}{}{warn}",
            i, emoji, linter.name, linter.normalized_name, source_info, impl_info
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum LinterImpl {
    Linter,
    ExprLinter,
    Both,
    Neither,
}

fn check_linter_implementation(path: &Path) -> std::io::Result<LinterImpl> {
    use std::fs::File;
    use std::io::{self, BufRead};

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut has_linter = false;
    let mut has_expr_linter = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // Check for impl blocks on a single line
        if !has_linter
            && (trimmed.starts_with("impl Linter for ")
                || (trimmed.contains("impl<") && trimmed.contains("> Linter for ")))
        {
            has_linter = true;
        }
        if !has_expr_linter
            && (trimmed.starts_with("impl ExprLinter for ")
                || (trimmed.contains("impl<") && trimmed.contains("> ExprLinter for ")))
        {
            has_expr_linter = true;
        }

        // Early exit if we've found both
        if has_linter && has_expr_linter {
            break;
        }
    }

    Ok(match (has_linter, has_expr_linter) {
        (true, true) => LinterImpl::Both,
        (true, false) => LinterImpl::Linter,
        (false, true) => LinterImpl::ExprLinter,
        (false, false) => LinterImpl::Neither,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SourceType {
    Missing,
    File,
    Dir,
    Both, // Both file and dir exist (might be an issue)
}

fn check_linter_source_exists(
    linter_name: &str,
    base_path: &Path,
) -> (SourceType, Option<LinterImpl>) {
    // Convert linter name to snake_case for the filename
    let filename = linter_name
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

    // Check for .rs file
    let file_path = base_path.join("linting").join(format!("{}.rs", filename));
    let dir_path = base_path.join("linting").join(&filename);
    let mod_rs_path = dir_path.join("mod.rs");

    let file_exists = file_path.exists();
    let dir_exists = dir_path.is_dir() && mod_rs_path.exists();

    let source_type = match (file_exists, dir_exists) {
        (true, true) => SourceType::Both,
        (true, false) => SourceType::File,
        (false, true) => SourceType::Dir,
        (false, false) => SourceType::Missing,
    };

    // Check implementation if source exists
    let implementation = if source_type != SourceType::Missing {
        let path = if file_exists {
            &file_path
        } else {
            &mod_rs_path
        };
        check_linter_implementation(path).ok()
    } else {
        None
    };

    (source_type, implementation)
}
