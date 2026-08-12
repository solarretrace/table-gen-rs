////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator library.
////////////////////////////////////////////////////////////////////////////////
#![forbid(non_ascii_idents)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(ambiguous_negative_literals)]
#![warn(closure_returning_async_block)]
#![warn(deprecated_in_future)]
#![warn(deprecated_safe_2024)]
#![warn(deref_into_dyn_supertrait)]
#![warn(edition_2024_expr_fragment_specifier)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(ffi_unwind_calls)]
#![warn(if_let_rescope)]
#![warn(impl_trait_overcaptures)]
#![warn(impl_trait_redundant_captures)]
#![warn(keyword_idents_2018)]
#![warn(keyword_idents_2024)]
#![warn(let_underscore_drop)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(missing_unsafe_on_extern)]
#![warn(redundant_imports)]
#![warn(redundant_lifetimes)]
#![warn(rust_2021_incompatible_closure_captures)]
#![warn(rust_2021_incompatible_or_patterns)]
#![warn(rust_2021_prefixes_incompatible_syntax)]
#![warn(rust_2021_prelude_collisions)]
#![warn(rust_2024_guarded_string_incompatible_syntax)]
#![warn(rust_2024_incompatible_pat)]
#![warn(rust_2024_prelude_collisions)]
#![warn(single_use_lifetimes)]
#![warn(tail_expr_drop_order)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unit_bindings)]
#![warn(unnameable_types)]
#![warn(unreachable_pub)]
#![warn(unsafe_attr_outside_unsafe)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unstable_features)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_macro_rules)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

// Clippy groups.
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![allow(clippy::redundant_pub_crate)] // False positives for exports.
#![warn(clippy::pedantic)]

// Clippy restriction lints.
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::create_dir)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exit)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::missing_enforced_import_renames)]
#![warn(clippy::mod_module_files)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::string_add)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::verbose_file_reads)]

// Non-improvement lints.
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::match_bool)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::single_match_else)]
#![allow(clippy::unseparated_literal_suffix)]
#![allow(clippy::uninlined_format_args)]

// Unreliable lints. May be enabled for spot checking.
#![allow(clippy::inline_always)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::shadow_unrelated)] // Does not work correctly.


// Internal modules.
mod aggregate;
mod collate;
mod context;
mod driver;
mod format;
mod render;
mod row;
mod sort;
mod split;

// Internal exports.
pub (in crate) use aggregate::Aggregate;
pub (in crate) use collate::Collate;
pub (in crate) use collate::CollateRow;
pub (in crate) use format::Format;
pub (in crate) use format::FormatRow;
pub (in crate) use sort::Sort;
pub (in crate) use split::Split;
pub (in crate) use split::SplitRow;
pub (in crate) use split::TextRow;

// Public exports.
pub use collate::ColumnDesc;
pub use collate::HorzAlign;
pub use collate::VertAlign;
pub use context::CellContext;
pub use context::RenderContext;
pub use driver::Table;
pub use driver::TableBuilder;
pub use format::DisplayFmt;
pub use format::Sign;
pub use render::Features;
pub use render::Renderer;
pub use row::Cell;
pub use row::Row;
pub use sort::ColumnOrd;
