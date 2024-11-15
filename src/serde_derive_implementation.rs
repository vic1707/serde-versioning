#![allow(clippy::wildcard_imports, clippy::enum_glob_use)]
#![expect(
    clippy::unreachable,
    clippy::too_many_lines,
    clippy::match_same_arms,
    clippy::single_call_fn,
    clippy::pattern_type_mismatch,
    clippy::unnested_or_patterns,
    clippy::wildcard_enum_match_arm,
    clippy::shadow_reuse,
    clippy::single_char_lifetime_names,
    clippy::min_ident_chars,
    clippy::if_then_some_else_none,
    clippy::equatable_if_let,
    clippy::needless_pass_by_value,
    clippy::as_conversions,
    clippy::option_if_let_else,
    clippy::cast_possible_truncation,
    clippy::indexing_slicing,
    clippy::module_name_repetitions,
    clippy::unused_self,
    clippy::needless_pass_by_ref_mut,
    clippy::iter_on_single_items,
    unexpected_cfgs,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::manual_let_else,
    clippy::use_self,
    clippy::missing_const_for_fn,
    clippy::unwrap_used,
    clippy::uninlined_format_args,
    clippy::arithmetic_side_effects,
    clippy::shadow_unrelated,
    clippy::single_match_else,
    clippy::doc_markdown,
    clippy::derive_partial_eq_without_eq,
    clippy::deref_by_slicing,
    clippy::cognitive_complexity,
    clippy::field_scoped_visibility_modifiers,
    clippy::struct_excessive_bools,
    clippy::else_if_without_else,
    clippy::pub_use,
    clippy::similar_names,
    clippy::separated_literal_suffix,
    clippy::redundant_pub_crate,
    clippy::let_underscore_must_use,
    clippy::panic,
    dead_code,
    clippy::let_underscore_untyped,
    clippy::string_slice,
    clippy::string_add,
    clippy::unwrap_in_result,
    clippy::manual_assert,
    clippy::unseparated_literal_suffix,
    clippy::checked_conversions,
    clippy::missing_assert_message,
    clippy::enum_variant_names,
    clippy::wildcard_in_or_patterns,
    clippy::match_like_matches_macro,
    reason = "clippy::all didn't work."
)]

#[macro_use]
pub mod bound;
#[macro_use]
pub mod fragment;

pub mod de;
pub mod dummy;
pub mod internals;
pub mod pretend;
pub mod ser;
pub mod this;
