/// Parser for `:command` definition strings.
use crate::user_commands::{CompletionType, Nargs, RangeMode, UserCommand};

/// Parse a `:command` definition string into a UserCommand.
/// Format: `:command [-flags ...] Name replacement`
pub fn parse_command_def(
    input: &str,
) -> Result<(UserCommand, bool), String> {
    let input = input.trim();
    let mut nargs = Nargs::Zero;
    let mut range = RangeMode::None;
    let mut bang_flag = false;
    let mut complete = None;
    let mut overwrite = false;

    let mut rest = input;

    // Check for ! (overwrite)
    if let Some(r) = rest.strip_prefix('!') {
        overwrite = true;
        rest = r.trim_start();
    }

    // Parse flags
    while rest.starts_with('-') {
        let (flag, remainder) = match rest.find(char::is_whitespace) {
            Some(i) => (&rest[..i], rest[i..].trim_start()),
            None => return Err("E471: Missing command name".to_string()),
        };

        if let Some(val) = flag.strip_prefix("-nargs=") {
            nargs = match val {
                "0" => Nargs::Zero,
                "1" => Nargs::One,
                "*" => Nargs::Any,
                "?" => Nargs::Optional,
                "+" => Nargs::OneOrMore,
                _ => {
                    return Err(format!(
                        "E176: Invalid -nargs value: {val}"
                    ))
                }
            };
        } else if flag == "-range" {
            range = RangeMode::CurrentLine;
        } else if flag == "-range=%" {
            range = RangeMode::WholeFile;
        } else if let Some(val) = flag.strip_prefix("-count=") {
            if let Ok(n) = val.parse::<usize>() {
                range = RangeMode::Count(n);
            }
        } else if flag == "-bang" {
            bang_flag = true;
        } else if let Some(val) = flag.strip_prefix("-complete=") {
            complete = Some(match val {
                "file" => CompletionType::File,
                "dir" => CompletionType::Dir,
                "buffer" => CompletionType::Buffer,
                "command" => CompletionType::Command,
                "option" => CompletionType::Option,
                "color" => CompletionType::Color,
                "help" => CompletionType::Help,
                other => CompletionType::Custom(other.to_string()),
            });
        }

        rest = remainder;
    }

    // Parse command name and replacement
    let (name, replacement) = match rest.find(char::is_whitespace) {
        Some(i) => (&rest[..i], rest[i..].trim_start()),
        None => {
            if rest.is_empty() {
                return Err("E471: Missing command name".to_string());
            }
            (rest, "")
        }
    };

    Ok((
        UserCommand {
            name: name.to_string(),
            replacement: replacement.to_string(),
            nargs,
            range,
            bang: bang_flag,
            complete,
        },
        overwrite,
    ))
}
