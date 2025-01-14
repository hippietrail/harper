use super::merge_linters::merge_linters;

mod avoid_contraction;
mod should_contract;

use avoid_contraction::AvoidContraction;
use should_contract::ShouldContract;

merge_linters! {PronounContraction => ShouldContract, AvoidContraction => "Choosing when to contract pronouns is a challenging art. This rule looks for faults." }
