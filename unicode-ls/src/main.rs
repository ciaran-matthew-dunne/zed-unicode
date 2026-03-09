use std::collections::HashMap;

use clap::Parser;
use simple_completion_language_server::*;

#[derive(Parser)]
#[clap(version, long_about = None, about = "Unicode language server")]
struct Cli {
    /// Include all symbols from latex-unicode.json (not just the curated set)
    #[arg(short, long)]
    include_all_symbols: bool,
}

/// Curated set of LaTeX-style Unicode completions.
///
/// These are the most commonly needed symbols for math, logic, and proof
/// assistants. Prefixes follow LaTeX conventions (without the backslash).
/// ASCII shorthand aliases are provided for common operators.
fn curated_symbols() -> Vec<(&'static str, &'static str)> {
    vec![
        // ── Greek lowercase ──────────────────────────────────────────
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
        ("phi", "φ"),
        ("varphi", "ϕ"),
        ("chi", "χ"),
        ("psi", "ψ"),
        ("omega", "ω"),
        // ── Greek uppercase ──────────────────────────────────────────
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
        // ── Arrows ───────────────────────────────────────────────────
        ("to", "→"),
        ("rightarrow", "→"),
        ("->", "→"),
        ("leftarrow", "←"),
        ("<-", "←"),
        ("leftrightarrow", "↔"),
        ("<->", "↔"),
        ("Rightarrow", "⇒"),
        ("=>", "⇒"),
        ("Leftarrow", "⇐"),
        ("Leftrightarrow", "⇔"),
        ("<=>", "⇔"),
        ("implies", "⟹"),
        ("iff", "⟺"),
        ("mapsto", "↦"),
        ("|->", "↦"),
        ("hookrightarrow", "↪"),
        ("hookleftarrow", "↩"),
        ("twoheadrightarrow", "↠"),
        ("->>", "↠"),
        ("uparrow", "↑"),
        ("downarrow", "↓"),
        ("longrightarrow", "⟶"),
        ("longleftarrow", "⟵"),
        ("longmapsto", "⟼"),
        // ── Logic & proof ────────────────────────────────────────────
        ("forall", "∀"),
        ("exists", "∃"),
        ("nexists", "∄"),
        ("neg", "¬"),
        ("lnot", "¬"),
        ("land", "∧"),
        ("wedge", "∧"),
        ("lor", "∨"),
        ("vee", "∨"),
        ("top", "⊤"),
        ("bot", "⊥"),
        ("vdash", "⊢"),
        ("|-", "⊢"),
        ("vDash", "⊨"),
        ("|=", "⊨"),
        ("nvdash", "⊬"),
        ("nvDash", "⊭"),
        ("turnstile", "⊢"),
        ("models", "⊧"),
        ("therefore", "∴"),
        ("because", "∵"),
        // ── Relations ────────────────────────────────────────────────
        ("equiv", "≡"),
        ("neq", "≠"),
        ("leq", "≤"),
        ("<=", "≤"),
        ("geq", "≥"),
        (">=", "≥"),
        ("ll", "≪"),
        ("gg", "≫"),
        ("prec", "≺"),
        ("succ", "≻"),
        ("preceq", "⪯"),
        ("succeq", "⪰"),
        ("sim", "∼"),
        ("simeq", "≃"),
        ("cong", "≅"),
        ("approx", "≈"),
        ("propto", "∝"),
        // ── Set theory ───────────────────────────────────────────────
        ("in", "∈"),
        ("notin", "∉"),
        ("ni", "∋"),
        ("subset", "⊂"),
        ("supset", "⊃"),
        ("subseteq", "⊆"),
        ("supseteq", "⊇"),
        ("cup", "∪"),
        ("cap", "∩"),
        ("emptyset", "∅"),
        ("varnothing", "∅"),
        // ── Operators ────────────────────────────────────────────────
        ("times", "×"),
        ("div", "÷"),
        ("cdot", "·"),
        ("circ", "∘"),
        ("oplus", "⊕"),
        ("otimes", "⊗"),
        ("pm", "±"),
        ("mp", "∓"),
        ("star", "⋆"),
        ("dagger", "†"),
        ("ddagger", "‡"),
        // ── Blackboard bold ──────────────────────────────────────────
        ("bbN", "ℕ"),
        ("bbZ", "ℤ"),
        ("bbQ", "ℚ"),
        ("bbR", "ℝ"),
        ("bbC", "ℂ"),
        ("bbP", "ℙ"),
        ("bbH", "ℍ"),
        ("bbB", "𝔹"),
        // ── Subscripts ───────────────────────────────────────────────
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
        // ── Superscripts ─────────────────────────────────────────────
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
        // ── Misc math ────────────────────────────────────────────────
        ("infty", "∞"),
        ("infinity", "∞"),
        ("partial", "∂"),
        ("nabla", "∇"),
        ("sqrt", "√"),
        ("cbrt", "∛"),
        ("sum", "∑"),
        ("prod", "∏"),
        ("int", "∫"),
        ("iint", "∬"),
        ("oint", "∮"),
        ("ldots", "…"),
        ("cdots", "⋯"),
        ("vdots", "⋮"),
        ("ddots", "⋱"),
        ("prime", "′"),
        ("dprime", "″"),
        ("ell", "ℓ"),
        ("hbar", "ℏ"),
        ("aleph", "ℵ"),
        // ── Delimiters & brackets ────────────────────────────────────
        ("langle", "⟨"),
        ("rangle", "⟩"),
        ("lceil", "⌈"),
        ("rceil", "⌉"),
        ("lfloor", "⌊"),
        ("rfloor", "⌋"),
        ("llbracket", "⟦"),
        ("rrbracket", "⟧"),
        ("Vert", "‖"),
        // ── Lambdapi-specific ────────────────────────────────────────
        // These are the ASCII alternatives used in Lambdapi syntax
        (":=", "≔"),
        ("coloneq", "≔"),
        ("assign", "≔"),
        ("hookarrow", "↪"),     // Lambdapi's rewrite arrow (↪)
    ]
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Build the unicode input map (prefix -> character)
    let mut unicode_input: HashMap<String, String> = HashMap::new();

    // Load curated symbols
    for (prefix, symbol) in curated_symbols() {
        unicode_input.entry(prefix.to_string()).or_insert_with(|| symbol.to_string());
    }

    // Optionally load the full latex-unicode.json dataset
    if cli.include_all_symbols {
        let json_str = include_str!("latex-unicode.json");
        if let Ok(all_symbols) = serde_json::from_str::<HashMap<String, String>>(json_str) {
            for (latex_cmd, symbol) in all_symbols {
                // Strip the leading backslash from LaTeX commands
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
        Vec::new(),       // no snippet-style completions
        unicode_input,
        etcetera::home_dir().unwrap().to_str().unwrap().into(),
    )
    .await;
}
