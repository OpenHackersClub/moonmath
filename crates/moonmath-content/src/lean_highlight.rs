/// Lean4 syntax highlighting for build-time use.
///
/// Produces HTML spans with CSS classes for syntax coloring:
/// - `lean-doc-comment`, `lean-comment` — comments
/// - `lean-string` — string literals
/// - `lean-symbol` — unicode math symbols
/// - `lean-kw` — keywords
/// - `lean-tactic` — tactic names
/// - `lean-type` — capitalized identifiers
/// - `lean-number` — numeric literals

pub fn highlight_lean(code: &str) -> String {
    let mut out = String::with_capacity(code.len() * 2);
    let chars: Vec<char> = code.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Doc comment: /-- ... -/
        if i + 2 < len && chars[i] == '/' && chars[i + 1] == '-' && chars[i + 2] == '-' {
            let start = i;
            i += 3;
            while i + 1 < len && !(chars[i] == '-' && chars[i + 1] == '/') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            }
            let text: String = chars[start..i].iter().collect();
            out.push_str(&format!(
                "<span class=\"lean-doc-comment\">{}</span>",
                escape_html(&text)
            ));
            continue;
        }

        // Line comment: -- ...
        if i + 1 < len && chars[i] == '-' && chars[i + 1] == '-' {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            out.push_str(&format!(
                "<span class=\"lean-comment\">{}</span>",
                escape_html(&text)
            ));
            continue;
        }

        // String literal
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            if i < len {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            out.push_str(&format!(
                "<span class=\"lean-string\">{}</span>",
                escape_html(&text)
            ));
            continue;
        }

        // Unicode math symbols
        if is_math_symbol(chars[i]) {
            out.push_str(&format!("<span class=\"lean-symbol\">{}</span>", chars[i]));
            i += 1;
            continue;
        }

        // Words (identifiers / keywords)
        if chars[i].is_alphanumeric() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let cls = classify_word(&word);
            if let Some(c) = cls {
                out.push_str(&format!(
                    "<span class=\"{}\">{}</span>",
                    c,
                    escape_html(&word)
                ));
            } else {
                out.push_str(&escape_html(&word));
            }
            continue;
        }

        // Other characters
        out.push_str(&escape_html(&chars[i].to_string()));
        i += 1;
    }

    out
}

fn is_math_symbol(c: char) -> bool {
    matches!(
        c,
        '∀' | '∃'
            | '∧'
            | '∨'
            | '¬'
            | '∣'
            | '→'
            | '←'
            | '↔'
            | '≤'
            | '≥'
            | '≠'
            | '∈'
            | '∉'
            | '⊂'
            | '⊆'
            | '∅'
            | '×'
            | '⟨'
            | '⟩'
            | 'λ'
    )
}

const KEYWORDS: &[&str] = &[
    "def",
    "theorem",
    "lemma",
    "by",
    "have",
    "let",
    "obtain",
    "suffices",
    "intro",
    "notation",
    "where",
    "if",
    "then",
    "else",
    "match",
    "with",
    "fun",
    "do",
    "return",
    "import",
    "open",
    "namespace",
    "end",
    "structure",
    "class",
    "instance",
    "deriving",
    "section",
    "variable",
    "example",
    "noncomputable",
    "private",
    "protected",
    "mutual",
    "inductive",
    "axiom",
    "abbrev",
    "set_option",
    "sorry",
];

const TACTICS: &[&str] = &[
    "grind",
    "simp",
    "simp_all",
    "by_cases",
    "induction",
    "exact",
    "apply",
    "rfl",
    "ring",
    "omega",
    "norm_num",
    "decide",
    "trivial",
    "constructor",
    "cases",
    "rcases",
    "rintro",
    "assumption",
    "contradiction",
    "linarith",
    "positivity",
    "field_simp",
    "push_neg",
    "use",
];

fn classify_word(word: &str) -> Option<&'static str> {
    if KEYWORDS.contains(&word) {
        return Some("lean-kw");
    }
    if TACTICS.contains(&word) {
        return Some("lean-tactic");
    }
    // Types: capitalized words
    if word
        .chars()
        .next()
        .map_or(false, |c| c.is_uppercase())
    {
        return Some("lean-type");
    }
    // Numbers
    if word.chars().all(|c| c.is_ascii_digit()) {
        return Some("lean-number");
    }
    None
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Extract fenced ```lean4 code blocks from markdown source.
pub fn extract_lean4_blocks(source: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed == "```lean4" || trimmed == "```lean" {
            let mut block = String::new();
            for inner_line in lines.by_ref() {
                if inner_line.trim() == "```" {
                    break;
                }
                if !block.is_empty() {
                    block.push('\n');
                }
                block.push_str(inner_line);
            }
            if !block.is_empty() {
                blocks.push(block);
            }
        }
    }

    blocks
}
