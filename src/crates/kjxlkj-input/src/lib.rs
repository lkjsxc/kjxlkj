//! kjxlkj-input: terminal input decoding and keybinding management.

pub mod headless;
pub mod input_decoder;
pub mod keybinding_dsl;
pub mod keybinding_tables;
pub mod layout_acceptance;
pub mod leader_keys;
pub mod mapping;
pub mod ux_coverage;

pub use headless::{
    parse_mode_string, parse_script, parse_script_key, script_step_to_keys, ScriptStep,
};
pub use input_decoder::{decode_event, decode_key, map_key_code, map_modifiers};
pub use keybinding_dsl::{
    parse_key_notation, parse_key_sequence, resolve_special, validate_key_sequence, KeyChord,
    SpecialKey,
};
pub use keybinding_tables::{
    build_normal_table, coverage_stats, ActionCategory, BindingEntry, BindingTable,
};
pub use layout_acceptance::{
    check_coverage, check_cursor_visible, check_min_size, check_no_overlap, run_all_invariants,
    InvariantKind, LayoutRegion,
};
pub use leader_keys::{default_leader_bindings, LeaderBinding, LeaderConfig, LeaderRegistry};
pub use mapping::{
    expand_recursive, parse_map_command, resolve_mapping, MapMode, MappingEntry, MappingStore,
};
pub use ux_coverage::{
    build_insert_coverage, build_normal_coverage, compute_summary, find_undocumented,
    find_untested, keyboard_only_check, CoverageEntry, CoverageSummary,
};
