#[derive(Clone, PartialEq)]
struct ModLine {
    snake_norm: Vec<String>,
}
#[derive(Clone, PartialEq)]
struct PubUseLine {
    snake_norm: Vec<String>,
    pascal_norm: Vec<String>,
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

    // 2. Check if snake case and pascal case entries in pub uses are in alphabetical order
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
    use std::collections::BTreeSet;

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

    let lint_group_rs = scan_lint_group_rs()?;
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
    // collect all `mod` lines
    // collect all `pub use` lines

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

    // Will be implemented in next steps
    Ok((mods, pub_uses))
}

struct UseSuperLine;
struct InsertRuleLine;

// enum LintGroupRsState {
//     BeforeUseSupers,
//     InUseSupers,
//     BetweenUseSupersAndRules,
//     InRules,
//     AfterRules,
// }

fn scan_lint_group_rs() -> anyhow::Result<(Vec<UseSuperLine>, Vec<InsertRuleLine>)> {
    // collect all `user super::` lines
    // collect all `insert_struct_rule`/`insert_expr_rule` lines

    let mut use_supers = Vec::new();
    let mut insert_rules = Vec::new();
    // let mut state = LintGroupRsState::BeforeUseSupers;

    let content = std::fs::read_to_string("harper-core/src/linting/lint_group.rs")?;

    for line in content.lines() {
        // starts with "use super::" / ends with ';' / snake_case `::` PascalCase
        // starts with "        insert_expr_rule!(" || "        insert_struct_rule!(" / ends with ';' / PascalCase ", " (true/false)
    }

    // Will be implemented in next steps
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
