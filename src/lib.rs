pub mod core;
pub mod discover;
pub mod hooks;
pub mod parser;

/// Apply the built-in TOML filter for `command` to `output`.
///
/// Returns the filtered string if a matching filter exists, or `None` if no
/// filter is registered for this command (caller should handle truncation
/// themselves, e.g. via [`parser::truncate_output`]).
pub fn filter_output(command: &str, output: &str) -> Option<String> {
    let filter = core::toml_filter::find_matching_filter(command)?;
    Some(core::toml_filter::apply_filter(filter, output))
}
