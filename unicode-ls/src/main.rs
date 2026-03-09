use std::collections::HashMap;

use clap::Parser;
use simple_completion_language_server::*;

#[derive(Parser)]
#[clap(version, long_about = None, about = "Unicode language server")]
struct Cli {
    /// Also load aliases from latex-unicode.json (backslash-less traditional
    /// LaTeX names that aren't in unicode-math-table.tex)
    #[arg(short, long)]
    include_latex_aliases: bool,
}

/// Aliases: shorthand or alternative names that map to the same character
/// as a canonical unicode-math symbol. These are traditional LaTeX synonyms,
/// ASCII shorthands, or names from other conventions.
fn aliases() -> Vec<(&'static str, &'static str)> {
    vec![
        // ── Greek aliases (traditional LaTeX names) ──────────────────
        ("alpha", "α"),
        ("beta", "β"),
        ("gamma", "γ"),
        ("delta", "δ"),
        ("epsilon", "ε"),
        ("varepsilon", "ε"),
        ("zeta", "ζ"),
        ("eta", "η"),
        ("theta", "θ"),
        ("vartheta", "ϑ"),
        ("iota", "ι"),
        ("kappa", "κ"),
        ("lambda", "λ"),
        ("mu", "μ"),
        ("nu", "ν"),
        ("xi", "ξ"),
        ("pi", "π"),
        ("rho", "ρ"),
        ("sigma", "σ"),
        ("varsigma", "ς"),
        ("tau", "τ"),
        ("upsilon", "υ"),
        ("phi", "ϕ"),
        ("varphi", "φ"),
        ("chi", "χ"),
        ("psi", "ψ"),
        ("omega", "ω"),
        ("Gamma", "Γ"),
        ("Delta", "Δ"),
        ("Theta", "Θ"),
        ("Lambda", "Λ"),
        ("Xi", "Ξ"),
        ("Pi", "Π"),
        ("Sigma", "Σ"),
        ("Phi", "Φ"),
        ("Psi", "Ψ"),
        ("Omega", "Ω"),
        // ── Arrow ASCII shorthands ───────────────────────────────────
        ("->", "→"),
        ("to", "→"),
        ("<-", "←"),
        ("gets", "←"),
        ("<->", "↔"),
        ("=>", "⇒"),
        ("<=>", "⇔"),
        ("|->", "↦"),
        ("->>", "↠"),
        ("implies", "⟹"),
        ("iff", "⟺"),
        // ── Logic ASCII shorthands ───────────────────────────────────
        ("lnot", "¬"),
        ("land", "∧"),
        ("lor", "∨"),
        ("|-", "⊢"),
        ("|=", "⊨"),
        ("turnstile", "⊢"),
        // ── Relation ASCII shorthands ────────────────────────────────
        ("<=", "≤"),
        (">=", "≥"),
        (":=", "≔"),
        ("assign", "≔"),
        ("~~", "≈"),
        // ── Set aliases ──────────────────────────────────────────────
        ("emptyset", "∅"),
        // ── Operator aliases ─────────────────────────────────────────
        ("cdot", "·"),
        ("infinity", "∞"),
        // ── Blackboard bold aliases ──────────────────────────────────
        ("bbA", "𝔸"),
        ("bbB", "𝔹"),
        ("bbC", "ℂ"),
        ("bbD", "𝔻"),
        ("bbE", "𝔼"),
        ("bbF", "𝔽"),
        ("bbG", "𝔾"),
        ("bbH", "ℍ"),
        ("bbK", "𝕂"),
        ("bbN", "ℕ"),
        ("bbP", "ℙ"),
        ("bbQ", "ℚ"),
        ("bbR", "ℝ"),
        ("bbS", "𝕊"),
        ("bbZ", "ℤ"),
        // ── Sub/superscripts (not in unicode-math-table.tex) ─────────
        ("_0", "₀"),
        ("_1", "₁"),
        ("_2", "₂"),
        ("_3", "₃"),
        ("_4", "₄"),
        ("_5", "₅"),
        ("_6", "₆"),
        ("_7", "₇"),
        ("_8", "₈"),
        ("_9", "₉"),
        ("_+", "₊"),
        ("_-", "₋"),
        ("_=", "₌"),
        ("_(", "₍"),
        ("_)", "₎"),
        ("_a", "ₐ"),
        ("_e", "ₑ"),
        ("_i", "ᵢ"),
        ("_o", "ₒ"),
        ("_r", "ᵣ"),
        ("_u", "ᵤ"),
        ("_n", "ₙ"),
        ("^0", "⁰"),
        ("^1", "¹"),
        ("^2", "²"),
        ("^3", "³"),
        ("^4", "⁴"),
        ("^5", "⁵"),
        ("^6", "⁶"),
        ("^7", "⁷"),
        ("^8", "⁸"),
        ("^9", "⁹"),
        ("^+", "⁺"),
        ("^-", "⁻"),
        ("^=", "⁼"),
        ("^(", "⁽"),
        ("^)", "⁾"),
        ("^n", "ⁿ"),
        ("^i", "ⁱ"),
        ("^-1", "⁻¹"),
    ]
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let mut unicode_input: HashMap<String, String> = HashMap::new();

    // Load all canonical symbols from unicode-math-table.tex
    let json_str = include_str!("unicode-math-symbols.json");
    if let Ok(symbols) = serde_json::from_str::<HashMap<String, String>>(json_str) {
        for (name, symbol) in symbols {
            if !name.is_empty() && !symbol.is_empty() {
                unicode_input.insert(name, symbol);
            }
        }
    }

    // Load aliases (traditional LaTeX names, ASCII shorthands, sub/superscripts)
    for (alias, symbol) in aliases() {
        unicode_input.entry(alias.to_string()).or_insert_with(|| symbol.to_string());
    }

    // Optionally load additional aliases from latex-unicode.json
    if cli.include_latex_aliases {
        let latex_json = include_str!("latex-unicode.json");
        if let Ok(all_symbols) = serde_json::from_str::<HashMap<String, String>>(latex_json) {
            for (latex_cmd, symbol) in all_symbols {
                let prefix = latex_cmd.trim_start_matches('\\').to_string();
                if !prefix.is_empty() && !symbol.is_empty() {
                    unicode_input.entry(prefix).or_insert(symbol);
                }
            }
        }
    }

    server::start(
        stdin,
        stdout,
        Vec::new(),
        unicode_input,
        etcetera::home_dir().unwrap().to_str().unwrap().into(),
    )
    .await;
}
