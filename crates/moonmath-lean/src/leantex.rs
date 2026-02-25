// LeanTeX: Convert Lean4 source to LaTeX mathematical notation.
//
// Extracts def/theorem/lemma declarations and converts their type
// signatures to LaTeX. This is a best-effort syntactic conversion
// for common Lean4 patterns — not a full Lean4 parser.

/// A parsed Lean4 declaration.
struct LeanItem {
    kind: &'static str,
    name: String,
    type_sig: String,
}

/// Extract mathematical statements from Lean4 source and convert to LaTeX.
///
/// Returns a LaTeX string with each declaration rendered as a labeled formula.
/// Multiple declarations are separated by `\\[6pt]` for display mode rendering.
pub fn lean_to_latex(source: &str) -> String {
    let items = extract_items(source);
    if items.is_empty() {
        return String::new();
    }

    items
        .iter()
        .map(|item| {
            let name_latex = escape_latex_text(&item.name);
            let sig_latex = lean_type_to_latex(&item.type_sig);
            format!(
                "\\textbf{{{}}} \\; \\text{{{}}} : {}",
                item.kind, name_latex, sig_latex
            )
        })
        .collect::<Vec<_>>()
        .join(" \\\\[6pt]\n")
}

/// Extract def/theorem/lemma items from Lean4 source.
fn extract_items(source: &str) -> Vec<LeanItem> {
    let mut items = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Match declaration keywords at the start of a line
        let (kind, rest) = if let Some(rest) = strip_decl_prefix(trimmed, "theorem") {
            ("Theorem", rest)
        } else if let Some(rest) = strip_decl_prefix(trimmed, "def") {
            ("Definition", rest)
        } else if let Some(rest) = strip_decl_prefix(trimmed, "lemma") {
            ("Lemma", rest)
        } else {
            i += 1;
            continue;
        };

        // Extract the name (first word after keyword)
        let rest = rest.trim();
        let name_end = rest
            .find(|c: char| c.is_whitespace() || c == '(' || c == ':' || c == '{')
            .unwrap_or(rest.len());
        let name = rest[..name_end].to_string();
        let after_name = &rest[name_end..];

        // Collect the full declaration (may span multiple lines)
        let mut full_decl = after_name.to_string();
        let mut j = i + 1;
        while j < lines.len() {
            let next = lines[j].trim();
            // Stop at next top-level declaration or blank line after `:= by`
            if next.is_empty()
                || strip_decl_prefix(next, "theorem").is_some()
                || strip_decl_prefix(next, "def").is_some()
                || strip_decl_prefix(next, "lemma").is_some()
                || strip_decl_prefix(next, "import").is_some()
                || next.starts_with("/--")
            {
                break;
            }
            full_decl.push(' ');
            full_decl.push_str(next);
            j += 1;
        }

        // Extract type signature: everything between the outermost `:` and `:= by` / `:=`
        if let Some(type_sig) = extract_type_signature(&full_decl) {
            items.push(LeanItem {
                kind,
                name,
                type_sig,
            });
        }

        i = j;
    }

    items
}

fn strip_decl_prefix<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let stripped = line.strip_prefix(keyword)?;
    // Must be followed by whitespace (not part of a longer word)
    if stripped.starts_with(|c: char| c.is_whitespace()) {
        Some(stripped)
    } else {
        None
    }
}

/// Extract the type signature from a declaration's body.
/// Looks for the pattern: `(params) : TypeSig := by ...` or `: TypeSig := ...`
fn extract_type_signature(decl: &str) -> Option<String> {
    // Find the colon that starts the return type (after all parameters).
    // Parameters are enclosed in `()` so we skip those.
    let mut depth = 0;
    let chars: Vec<char> = decl.chars().collect();
    let mut colon_pos = None;

    for (idx, &c) in chars.iter().enumerate() {
        match c {
            '(' | '{' | '[' => depth += 1,
            ')' | '}' | ']' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            ':' if depth == 0 => {
                // Check it's not `:=`
                if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    continue;
                }
                colon_pos = Some(idx);
                // Take the last top-level colon before `:=`
            }
            _ => {}
        }
    }

    let colon_pos = colon_pos?;
    let after_colon: String = chars[colon_pos + 1..].iter().collect();

    // Find `:= by` or `:=` to trim the body
    let type_sig = if let Some(idx) = after_colon.find(":= by") {
        &after_colon[..idx]
    } else if let Some(idx) = after_colon.find(":=") {
        &after_colon[..idx]
    } else {
        &after_colon
    };

    let trimmed = type_sig.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Convert Lean4 type-level syntax to LaTeX.
fn lean_type_to_latex(lean: &str) -> String {
    let mut s = lean.to_string();

    // Multi-word replacements first
    s = s.replace("Nat.factorial", "\\operatorname{factorial}");

    // Lean4 unicode → LaTeX commands
    s = s.replace('∀', "\\forall\\,");
    s = s.replace('∃', "\\exists\\,");
    s = s.replace('∧', "\\land");
    s = s.replace('∨', "\\lor");
    s = s.replace('¬', "\\lnot ");
    s = s.replace('→', "\\to");
    s = s.replace('←', "\\leftarrow");
    s = s.replace('↔', "\\leftrightarrow");
    s = s.replace('≥', "\\ge");
    s = s.replace('≤', "\\le");
    s = s.replace('≠', "\\ne");
    s = s.replace('∣', "\\mid");
    s = s.replace('∈', "\\in");
    s = s.replace('∉', "\\notin");
    s = s.replace('⊂', "\\subset");
    s = s.replace('⊆', "\\subseteq");
    s = s.replace('∅', "\\emptyset");
    s = s.replace('×', "\\times");
    s = s.replace('⟨', "\\langle");
    s = s.replace('⟩', "\\rangle");
    s = s.replace('λ', "\\lambda\\,");

    // Common Lean types → LaTeX
    s = s.replace("Nat", "\\mathbb{N}");
    s = s.replace("Int", "\\mathbb{Z}");
    s = s.replace("Prop", "\\text{Prop}");

    // Known identifiers as text
    let known_idents = ["IsPrime", "IsEven", "IsOdd"];
    for ident in &known_idents {
        let replacement = format!("\\text{{{}}}", ident);
        s = s.replace(ident, &replacement);
    }

    s
}

/// Escape a name for use in LaTeX \text{} blocks.
fn escape_latex_text(name: &str) -> String {
    name.replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_def() {
        let source = "def IsPrime (p : Nat) : Prop :=\n  p ≥ 2 ∧ ∀ m : Nat, m ∣ p → m = 1 ∨ m = p\n";
        let latex = lean_to_latex(source);
        assert!(latex.contains("Definition"));
        assert!(latex.contains("IsPrime"));
    }

    #[test]
    fn test_extract_theorem() {
        let source = "theorem factorial_pos (n : Nat) : Nat.factorial n ≥ 1 := by\n  induction n with\n  | zero => simp\n";
        let latex = lean_to_latex(source);
        assert!(latex.contains("Theorem"));
        assert!(latex.contains("factorial\\_pos"));
        assert!(latex.contains("\\ge"));
    }

    #[test]
    fn test_unicode_conversion() {
        let result = lean_type_to_latex("∀ n, ∃ p, p > n ∧ IsPrime p");
        assert!(result.contains("\\forall"));
        assert!(result.contains("\\exists"));
        assert!(result.contains("\\land"));
    }

    #[test]
    fn test_full_showcase_source() {
        let source = r#"import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Data.Nat.Factorial.Basic

def IsPrime (p : Nat) : Prop :=
  p ≥ 2 ∧ ∀ m : Nat, m ∣ p → m = 1 ∨ m = p

theorem exists_prime_factor (n : Nat) (hn : n ≥ 2) :
    ∃ p, IsPrime p ∧ p ∣ n := by
  grind

theorem factorial_pos (n : Nat) : Nat.factorial n ≥ 1 := by
  induction n with
  | zero => simp [Nat.factorial]
  | succ k ih => simp [Nat.factorial]; omega

theorem InfinitudeOfPrimes (n : Nat) :
    ∃ p, p > n ∧ IsPrime p := by
  have h1 : Nat.factorial n + 1 ≥ 2 := by omega
  obtain ⟨p, hp, hdvd⟩ := exists_prime_factor _ h1
  use p
"#;
        let latex = lean_to_latex(source);
        // Should extract 4 items: IsPrime def, exists_prime_factor, factorial_pos, InfinitudeOfPrimes
        assert!(latex.contains("Definition"));
        assert!(latex.contains("\\text{IsPrime}"));
        assert!(latex.contains("exists\\_prime\\_factor"));
        assert!(latex.contains("factorial\\_pos"));
        assert!(latex.contains("InfinitudeOfPrimes"));
    }
}
