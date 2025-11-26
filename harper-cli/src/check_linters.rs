use std::collections::BTreeSet;
use std::path::Path;

#[derive(Clone, PartialEq)]
struct ModLine {
    snake_norm: Vec<String>,
}
#[derive(Clone, PartialEq)]
struct PubUseLine {
    snake_norm: Vec<String>,
    pascal_norm: Vec<String>,
}

#[derive(Clone, PartialEq)]
struct UseSuperLine {
    snake_norm: Vec<String>,
    pascal_norm: Vec<String>,
}

#[derive(Clone, PartialEq)]
enum RuleKind {
    Expr,
    Struct,
}

#[derive(Clone, PartialEq)]
struct InsertRuleLine {
    kind: RuleKind,
    pascal_norm: Vec<String>,
    default_config: bool,
}

pub fn check_linters(_verbose: bool) -> anyhow::Result<()> {
    let (mods, pub_uses) = scan_mod_rs()?;

    // 1. Check if snake case entries in mod lines are in alphabetical order
    let mut sorted_mods = mods.clone();
    sorted_mods.sort_by(|a, b| a.snake_norm.join("_").cmp(&b.snake_norm.join("_")));
    if mods != sorted_mods {
        eprintln!("❌ Mod lines are not in alphabetical order");
        // Show first few out-of-order items
        for (i, (a, b)) in mods.iter().zip(sorted_mods.iter()).take(5).enumerate() {
            if a.snake_norm != b.snake_norm {
                eprintln!(
                    "  {}. Found: {}, Expected: {}",
                    i + 1,
                    a.snake_norm.join("_"),
                    b.snake_norm.join("_")
                );
            }
        }
    } else {
        println!("✅ Mod lines are in alphabetical order");
    }

    // 2. Check if snake case and pascal case entries in pub use lines are in alphabetical order
    let mut sorted_pub_uses = pub_uses.clone();
    sorted_pub_uses.sort_by(|a, b| {
        a.snake_norm
            .join("_")
            .cmp(&b.snake_norm.join("_"))
            .then_with(|| a.pascal_norm.join("").cmp(&b.pascal_norm.join("")))
    });

    if pub_uses != sorted_pub_uses {
        eprintln!("\n❌ Pub use lines are not in alphabetical order");
        // Show first few out-of-order items
        for (i, (a, b)) in pub_uses
            .iter()
            .zip(sorted_pub_uses.iter())
            .take(5)
            .enumerate()
        {
            if a.snake_norm != b.snake_norm {
                eprintln!(
                    "  {}. Found: {}::{}",
                    i + 1,
                    a.snake_norm.join("_"),
                    a.pascal_norm.join("")
                );
                eprintln!(
                    "     Expected: {}::{}",
                    b.snake_norm.join("_"),
                    b.pascal_norm.join("")
                );
            }
        }
    } else {
        println!("\n✅ Pub use lines are in alphabetical order");
    }

    // 3. Check if number of mod lines matches number of pub uses
    if mods.len() != pub_uses.len() {
        eprintln!(
            "\n❌ Mismatch in number of items: {} mods vs {} pub uses",
            mods.len(),
            pub_uses.len()
        );
    } else {
        println!(
            "\n✅ Number of mods matches number of pub uses: {}",
            mods.len()
        );
    }

    // 4. List items that are in one but not the other
    let mod_set: BTreeSet<_> = mods.iter().map(|m| m.snake_norm.join("_")).collect();
    let pub_set: BTreeSet<_> = pub_uses.iter().map(|p| p.snake_norm.join("_")).collect();

    let missing_in_pub: Vec<_> = mod_set.difference(&pub_set).collect();
    let extra_in_pub: Vec<_> = pub_set.difference(&mod_set).collect();

    if !missing_in_pub.is_empty() || !extra_in_pub.is_empty() {
        eprintln!("\n❌ Mismatch between mods and pub uses:");
        if !missing_in_pub.is_empty() {
            eprintln!("  Missing in pub uses ({}):", missing_in_pub.len());
            for item in missing_in_pub {
                eprintln!("    - {}", item);
            }
        }
        if !extra_in_pub.is_empty() {
            eprintln!("\n  Extra in pub uses ({}):", extra_in_pub.len());
            for item in extra_in_pub {
                eprintln!("    - {}", item);
            }
        }
    } else {
        println!("\n✅ All mods have corresponding pub uses and vice versa");
    }

    let (use_supers, insert_rules) = scan_lint_group_rs()?;

    // 5. Check if snake case and pascal case entries in use super lines are in alphabetical order
    let mut sorted_use_supers = use_supers.clone();
    sorted_use_supers.sort_by(|a, b| {
        a.snake_norm
            .join("_")
            .cmp(&b.snake_norm.join("_"))
            .then_with(|| a.pascal_norm.join("").cmp(&b.pascal_norm.join("")))
    });

    if use_supers != sorted_use_supers {
        eprintln!("\n❌ Use super lines are not in alphabetical order");
        // Show first few out-of-order items
        for (i, (a, b)) in use_supers
            .iter()
            .zip(sorted_use_supers.iter())
            .take(5)
            .enumerate()
        {
            if a.snake_norm != b.snake_norm {
                eprintln!(
                    "  {}. Found: {}::{}",
                    i + 1,
                    a.snake_norm.join("_"),
                    a.pascal_norm.join("")
                );
                eprintln!(
                    "     Expected: {}::{}",
                    b.snake_norm.join("_"),
                    b.pascal_norm.join("")
                );
            }
        }
    } else {
        println!("\n✅ Use super lines are in alphabetical order");
    }

    // 6. Check if snake case entries in insert rule lines are in alphabetical order
    let mut sorted_insert_rules = insert_rules.clone();
    sorted_insert_rules.sort_by(|a, b| a.pascal_norm.join("_").cmp(&b.pascal_norm.join("_")));
    if insert_rules != sorted_insert_rules {
        eprintln!("❌ Insert rule lines are not in alphabetical order");
        // Show first few out-of-order items
        for (i, (a, b)) in insert_rules
            .iter()
            .zip(sorted_insert_rules.iter())
            .take(5)
            .enumerate()
        {
            if a.pascal_norm != b.pascal_norm {
                eprintln!(
                    "  {}. Found: {}, Expected: {}",
                    i + 1,
                    a.pascal_norm.join("_"),
                    b.pascal_norm.join("_")
                );
            }
        }
    } else {
        println!("✅ Insert rule lines are in alphabetical order");
    }

    // 7. Check if number of use super lines matches number of insert rule lines
    if use_supers.len() != insert_rules.len() {
        eprintln!(
            "\n❌ Mismatch in number of items: {} use supers vs {} insert rules",
            use_supers.len(),
            insert_rules.len()
        );
    } else {
        println!(
            "\n✅ Number of use supers matches number of insert rules: {}",
            use_supers.len()
        );
    }

    // Build a complete set of all linters mentioned
    fn join_parts(parts: &[String]) -> String {
        parts
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
            .join("_")
    }

    // Then collect into a BTreeSet to maintain order and remove duplicates
    let mut all_linters = BTreeSet::new();

    // Add mod linters (snake_case)
    for linter in &mods {
        all_linters.insert(join_parts(&linter.snake_norm));
    }

    // Add pub use linters (snake_case)
    for linter in &pub_uses {
        all_linters.insert(join_parts(&linter.snake_norm));
    }

    // Add use super linters (PascalCase)
    for linter in &use_supers {
        all_linters.insert(join_parts(&linter.pascal_norm));
    }

    // Add insert rule linters (PascalCase)
    for linter in &insert_rules {
        all_linters.insert(join_parts(&linter.pascal_norm));
    }

    // Find the longest linter name length
    let max_len = all_linters.iter().map(String::len).max().unwrap_or(0);

    let linting_dir = Path::new("harper-core/src/linting");

    // Now print with aligned columns
    for linter in &all_linters {
        let in_mod_mod = mods.iter().any(|m| join_parts(&m.snake_norm) == *linter);
        let in_mod_pub_use = pub_uses
            .iter()
            .any(|p| join_parts(&p.snake_norm) == *linter);
        let in_lint_group_use_super = use_supers
            .iter()
            .any(|u| join_parts(&u.pascal_norm) == *linter);
        let rule_info = insert_rules.iter().find_map(|r| {
            if join_parts(&r.pascal_norm) == *linter {
                Some((r.kind.clone(), r.default_config))
            } else {
                None
            }
        });

        let module_path = linting_dir.join(format!("{}.rs", linter));
        let dir_path = linting_dir.join(linter);

        let file_icon = if module_path.exists() {
            "📄" // File exists
        } else if dir_path.exists() {
            "📁" // Directory exists
        } else {
            "  " // Neither exists
        };

        let mod_kind = if let Ok(module) = std::fs::read_to_string(&module_path) {
            let mut kind = "  "; // Default to nothing
            for line in module.lines() {
                if line.contains("impl Linter for") {
                    kind = "🧩"; // Struct linter
                    break;
                } else if line.contains("impl ExprLinter for") {
                    kind = "💬"; // Expression linter
                    break;
                }
            }
            kind
        } else {
            "  " // File doesn't exist or can't be read
        };

        let excl = if let Some((kind, _)) = &rule_info {
            match kind {
                RuleKind::Expr if mod_kind == "🧩" => "‼️", // Mismatch: Rule says Expr but file implements Linter
                RuleKind::Struct if mod_kind == "💬" => "‼️", // Mismatch: Rule says Struct but file implements ExprLinter
                _ => "",
            }
        } else {
            ""
        };

        println!(
            "{:<width$} | mod.rs: {}{} | lint_group.rs: {}{} | file: {}{} {}",
            linter,
            if in_mod_mod { "📦" } else { "  " },
            if in_mod_pub_use { "🍻" } else { "  " },
            if in_lint_group_use_super {
                "🦸"
            } else {
                "  "
            },
            if let Some((kind, default)) = rule_info {
                match kind {
                    RuleKind::Expr => "💬",
                    RuleKind::Struct => "🧩",
                }
                .to_string()
                    + if default { "✅" } else { "❌" }
            } else {
                "    ".to_string() // 4 spaces to match the emoji + symbol width
            },
            file_icon,
            mod_kind,
            excl,
            width = max_len
        );
    }

    Ok(())
}

// enum ModRsState {
//     BeforeMods,
//     InMods,
//     BetweenModsAndPubUses,
//     InPubUses,
//     AfterPubUses,
// }

fn scan_mod_rs() -> anyhow::Result<(Vec<ModLine>, Vec<PubUseLine>)> {
    let mut mods = Vec::new();
    let mut pub_uses = Vec::new();
    // let mut state = ModRsState::BeforeMods;

    let content = std::fs::read_to_string("harper-core/src/linting/mod.rs")?;

    for line in content.lines() {
        // starts with "mod " / ends with ';' / snake_case only
        if line.starts_with("mod ")
            && let Some(after) = line.strip_prefix("mod ")
            && let Some(linter) = after.strip_suffix(';')
        {
            let linter_norm_split = split_snake_case(linter);
            // eprintln!("🧱 '{linter}' -> {:?}", linter_norm_split);

            mods.push(ModLine {
                snake_norm: linter_norm_split,
            });
        }
        // starts with "pub use " / ends with ';' / snake_case `::` PascalCase
        if line.starts_with("pub use ")
            && let Some(after) = line.strip_prefix("pub use ")
            && let Some(snake_pascal) = after.strip_suffix(';')
            && let Some((snake, pascal)) = snake_pascal.split_once("::")
        {
            let (snake_norm, pascal_norm) = (split_snake_case(snake), split_pascal_case(pascal));

            if snake_norm.len() != pascal_norm.len()
                || !snake_norm
                    .iter()
                    .zip(&pascal_norm)
                    .all(|(s, p)| s.eq_ignore_ascii_case(p))
            {
                eprintln!(
                    "🍻 '{snake}' / '{pascal}' -> {:?} / {:?}",
                    snake_norm, pascal_norm
                );
            } else {
                pub_uses.push(PubUseLine {
                    snake_norm,
                    pascal_norm,
                });
            }
        }
    }

    Ok((mods, pub_uses))
}

// enum LintGroupRsState {
//     BeforeUseSupers,
//     InUseSupers,
//     BetweenUseSupersAndRules,
//     InRules,
//     AfterRules,
// }

fn scan_lint_group_rs() -> anyhow::Result<(Vec<UseSuperLine>, Vec<InsertRuleLine>)> {
    let mut use_supers = Vec::new();
    let mut insert_rules = Vec::new();
    // let mut state = LintGroupRsState::BeforeUseSupers;

    let content = std::fs::read_to_string("harper-core/src/linting/lint_group.rs")?;

    for line in content.lines() {
        // starts with "use super::" / ends with ';' / snake_case `::` PascalCase
        if line.starts_with("use super::")
            && let Some(after) = line.strip_prefix("use super::")
            && let Some(snake_pascal) = after.strip_suffix(';')
            && let Some((snake, pascal)) = snake_pascal.split_once("::")
        {
            let (snake_norm, pascal_norm) = (split_snake_case(snake), split_pascal_case(pascal));

            if snake_norm.len() != pascal_norm.len()
                || !snake_norm
                    .iter()
                    .zip(&pascal_norm)
                    .all(|(s, p)| s.eq_ignore_ascii_case(p))
            {
                eprintln!(
                    "🦸 '{snake}' / '{pascal}' -> {:?} / {:?}",
                    snake_norm, pascal_norm
                );
            } else {
                use_supers.push(UseSuperLine {
                    snake_norm,
                    pascal_norm,
                });
            }
        }

        if line.starts_with("        insert_")
            && let Some(after) = line.strip_prefix("        insert_")
            && let Some(stuff) = after.strip_suffix(");")
            && let Some((kind, rule_default_config)) = stuff.split_once("_rule!(")
            && let Some((rule, default_config)) = rule_default_config.split_once(", ")
        {
            if (kind == "expr" || kind == "struct")
                && (default_config == "true" || default_config == "false")
            {
                let pascal_norm = split_pascal_case(rule);
                insert_rules.push(InsertRuleLine {
                    kind: if kind == "expr" {
                        RuleKind::Expr
                    } else {
                        RuleKind::Struct
                    },
                    pascal_norm,
                    default_config: default_config == "true",
                });
            } else {
                eprintln!("❌ insert rule: '{kind}' // '{rule}' // '{default_config}'");
            }
        }
    }

    Ok((use_supers, insert_rules))
}

fn split_snake_case(s: &str) -> Vec<String> {
    s.split('_').map(String::from).collect()
}

fn split_pascal_case(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for c in s.chars() {
        if c.is_uppercase() && !current.is_empty() {
            result.push(current);
            current = String::new();
        }
        current.push(c);
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}
