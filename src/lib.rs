//! Execute shell scripts embedded within text.
//!
//! # Example
//!
//! Let's say you have a blog page in HTML that you want to keep updated with
//! the total number of posts and links to all posts.  Let's call that page
//! `archive.html`, as seen below.
//!
//! ```html
//! <!-- archive.html -->
//! <h1>My Blog Archive</h1>
//! <p>Contains !((ls pages/*.html | wc -l))! post(s).</p>
//! <ul>
//! !((
//! for PAGE in $(ls pages/*.html)
//! do
//!   PAGENAME="$(basename $PAGE .html)"  # name of file minus .html extension
//!   echo "<li><a href=\"$PAGE\">${PAGENAME//'-'/' '}</a></li>"
//! done
//! ))!
//! </ul>
//! ```
//!
//! This file can be processed from the command line using this project's
//! binary:
//!
//!     corre -i archive.html -o www/archive.html
//!
//! You can also use the `run_embedded_scripts(text, opening_tag, closing_tag)`
//! function provided by this project's library:
//!
//! ```rust
//! // Load `archive.html` into the `String` `input_text`
//! let shouldRecur = false;  // Don't operate on script outputs
//! let output_text = corre::run_embedded_scripts(input_text, "!((", "))!", shouldRecur)?;
//! // Save the `String` `output_text` to the file `www/archive.html`
//! ```
//!
//! Both will produce the modified text:
//!
//! ```html
//! <!-- www/archive.html -->
//! <h1>My Blog Archive</h1>
//! <p>Contains 3 post(s)</p>
//! <ul>
//! <li><a href="pages/Hydroelectric-Dams.html">Hydroelectric Dams</a></li>
//! <li><a href="pages/The-Finnish-Genitive-Case.html">The Finnish Genitive Case</a></li>
//! <li><a href="pages/Vultee-XP54-Swoose-Goose.html">Vultee XP54 Swoose Goose</a></li>
//! </ul>
//! ```

use regex::Regex;
use subprocess::{Exec, PopenError};

/// Intersperses the given string with backslashes and returns it.
fn regex_safe(not_safe: &str) -> String {
    let mut safe = String::new();

    for char in not_safe.chars() {
        safe.push('\\');
        safe.push(char);
    }

    safe
}

/// Returns the regex pattern used to match shell commands that are between the
/// given opening and closing tags.
fn make_regex_pattern(opening_tag: &str, closing_tag: &str) -> Regex {
    Regex::new(
        format!(
            "(?ms){}(.*?){}",
            regex_safe(opening_tag),
            regex_safe(closing_tag),
        )
        .as_str(),
    )
    .unwrap()
}

/// Executes the given script in the shell and returns the STDOUT.
fn exec_script(script: &str) -> Result<String, PopenError> {
    Ok({ Exec::shell(script) }
        .capture()?
        .stdout_str()
        .trim_ascii()
        .to_owned())
}

/// Runs all scripts that are embedded within the given text.  Scripts are
/// identified using the given opening and closing tags.  Returns the original
/// text, in which the shell scripts have been replaced with their STDOUT.
/// If recur is true, operates on script outputs as they are generated.
pub fn run_embedded_scripts(
    text: String,
    opening_tag: &str,
    closing_tag: &str,
    recur: bool,
) -> Result<String, PopenError> {
    let pattern = make_regex_pattern(opening_tag, closing_tag);

    if let Some(needle) = pattern.find(&text) {
        // Split text into before needle, after needle, and the needle (script)
        let mut before = text[..needle.start()].to_owned();
        let after = &text[needle.end()..];
        let script = &text[needle.start() + opening_tag.len()..needle.end() - closing_tag.len()];

        let mut script_result = exec_script(script)?;

        // text before the needle has been processed, but we handle the rest
        // differently depending on whether or not recur is true
        let rest = if recur {
            // Process both the script result and the text after the needle
            script_result.push_str(after);
            run_embedded_scripts(script_result, opening_tag, closing_tag, recur)?
        } else {
            // Only process the text after the needle
            before.push_str(&script_result);
            run_embedded_scripts(after.to_owned(), opening_tag, closing_tag, recur)?
        };

        before.push_str(&rest);

        Ok(before)
    } else {
        Ok(text)
    }
}
