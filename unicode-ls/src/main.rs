use std::collections::HashMap;

use clap::Parser;
use simple_completion_language_server::*;

#[derive(Parser)]
#[clap(version, long_about = None, about = "Unicode language server")]
struct Cli {
    /// Include all 2,400+ symbols from unicode-math (not just the curated set)
    #[arg(short, long)]
    include_all_symbols: bool,
}

/// Curated set of commonly needed Unicode symbols.
///
/// Names are the canonical names from the unicode-math LaTeX package.
/// Returns (canonical_name, character) pairs.
fn curated_symbols() -> Vec<(&'static str, &'static str)> {
    vec![
        // ── Greek lowercase (unicode-math: \mupalpha etc.) ───────────
        ("mupalpha", "α"),
        ("mupbeta", "β"),
        ("mupgamma", "γ"),
        ("mupdelta", "δ"),
        ("mupvarepsilon", "ε"),
        ("mupzeta", "ζ"),
        ("mupeta", "η"),
        ("muptheta", "θ"),
        ("mupvartheta", "ϑ"),
        ("mupiota", "ι"),
        ("mupkappa", "κ"),
        ("muplambda", "λ"),
        ("mupmu", "μ"),
        ("mupnu", "ν"),
        ("mupxi", "ξ"),
        ("muppi", "π"),
        ("muprho", "ρ"),
        ("mupsigma", "σ"),
        ("mupvarsigma", "ς"),
        ("muptau", "τ"),
        ("mupupsilon", "υ"),
        ("mupvarphi", "φ"),
        ("mupphi", "ϕ"),
        ("mupchi", "χ"),
        ("muppsi", "ψ"),
        ("mupomega", "ω"),
        ("mupvarepsilon", "ε"),
        // ── Greek uppercase (unicode-math: \mupGamma etc.) ───────────
        ("mupGamma", "Γ"),
        ("mupDelta", "Δ"),
        ("mupTheta", "Θ"),
        ("mupLambda", "Λ"),
        ("mupXi", "Ξ"),
        ("mupPi", "Π"),
        ("mupSigma", "Σ"),
        ("mupPhi", "Φ"),
        ("mupPsi", "Ψ"),
        ("mupOmega", "Ω"),
        // ── Arrows ───────────────────────────────────────────────────
        ("rightarrow", "→"),
        ("leftarrow", "←"),
        ("leftrightarrow", "↔"),
        ("Rightarrow", "⇒"),
        ("Leftarrow", "⇐"),
        ("Leftrightarrow", "⇔"),
        ("mapsto", "↦"),
        ("hookrightarrow", "↪"),
        ("hookleftarrow", "↩"),
        ("twoheadrightarrow", "↠"),
        ("twoheadleftarrow", "↞"),
        ("uparrow", "↑"),
        ("downarrow", "↓"),
        ("updownarrow", "↕"),
        ("Uparrow", "⇑"),
        ("Downarrow", "⇓"),
        ("Longrightarrow", "⟹"),
        ("Longleftarrow", "⟸"),
        ("Longleftrightarrow", "⟺"),
        ("longrightarrow", "⟶"),
        ("longleftarrow", "⟵"),
        ("longmapsto", "⟼"),
        ("rightarrowtail", "↣"),
        ("leftarrowtail", "↢"),
        ("rightsquigarrow", "⇝"),
        ("nrightarrow", "↛"),
        ("nleftarrow", "↚"),
        ("nRightarrow", "⇏"),
        ("nLeftarrow", "⇍"),
        // ── Logic & proof ────────────────────────────────────────────
        ("forall", "∀"),
        ("exists", "∃"),
        ("nexists", "∄"),
        ("neg", "¬"),
        ("wedge", "∧"),
        ("vee", "∨"),
        ("top", "⊤"),
        ("bot", "⊥"),
        ("vdash", "⊢"),
        ("dashv", "⊣"),
        ("vDash", "⊨"),
        ("Vdash", "⊩"),
        ("Vvdash", "⊪"),
        ("nvdash", "⊬"),
        ("nvDash", "⊭"),
        ("models", "⊧"),
        ("therefore", "∴"),
        ("because", "∵"),
        // ── Relations ────────────────────────────────────────────────
        ("equiv", "≡"),
        ("nequiv", "≢"),
        ("neq", "≠"),
        ("leq", "≤"),
        ("geq", "≥"),
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
        ("coloneq", "≔"),
        ("eqcolon", "≕"),
        // ── Set theory ───────────────────────────────────────────────
        ("in", "∈"),
        ("notin", "∉"),
        ("ni", "∋"),
        ("subset", "⊂"),
        ("supset", "⊃"),
        ("subseteq", "⊆"),
        ("supseteq", "⊇"),
        ("nsubseteq", "⊈"),
        ("nsupseteq", "⊉"),
        ("subsetneq", "⊊"),
        ("supsetneq", "⊋"),
        ("cup", "∪"),
        ("cap", "∩"),
        ("bigcup", "⋃"),
        ("bigcap", "⋂"),
        ("varnothing", "∅"),
        ("setminus", "∖"),
        // ── Operators ────────────────────────────────────────────────
        ("times", "×"),
        ("div", "÷"),
        ("cdotp", "·"),
        ("circ", "∘"),
        ("oplus", "⊕"),
        ("ominus", "⊖"),
        ("otimes", "⊗"),
        ("odot", "⊙"),
        ("pm", "±"),
        ("mp", "∓"),
        ("star", "⋆"),
        ("dagger", "†"),
        ("ddagger", "‡"),
        ("amalg", "⨿"),
        ("wr", "≀"),
        // ── Big operators ────────────────────────────────────────────
        ("sum", "∑"),
        ("prod", "∏"),
        ("coprod", "∐"),
        ("bigwedge", "⋀"),
        ("bigvee", "⋁"),
        ("bigoplus", "⨁"),
        ("bigotimes", "⨂"),
        // ── Integrals ────────────────────────────────────────────────
        ("int", "∫"),
        ("iint", "∬"),
        ("iiint", "∭"),
        ("oint", "∮"),
        // ── Blackboard bold ──────────────────────────────────────────
        ("BbbA", "𝔸"),
        ("BbbB", "𝔹"),
        ("BbbC", "ℂ"),
        ("BbbD", "𝔻"),
        ("BbbE", "𝔼"),
        ("BbbF", "𝔽"),
        ("BbbG", "𝔾"),
        ("BbbH", "ℍ"),
        ("BbbK", "𝕂"),
        ("BbbN", "ℕ"),
        ("BbbP", "ℙ"),
        ("BbbQ", "ℚ"),
        ("BbbR", "ℝ"),
        ("BbbS", "𝕊"),
        ("BbbZ", "ℤ"),
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
        ("partial", "∂"),
        ("nabla", "∇"),
        ("sqrt", "√"),
        ("cbrt", "∛"),
        ("ldots", "…"),
        ("cdots", "⋯"),
        ("vdots", "⋮"),
        ("ddots", "⋱"),
        ("prime", "′"),
        ("dprime", "″"),
        ("ell", "ℓ"),
        ("hbar", "ℏ"),
        ("aleph", "ℵ"),
        ("wp", "℘"),
        ("Re", "ℜ"),
        ("Im", "ℑ"),
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
        // ── Misc symbols ────────────────────────────────────────────
        ("Box", "□"),
        ("Diamond", "◇"),
        ("lozenge", "◊"),
        ("checkmark", "✓"),
    ]
}

/// Aliases: shorthand or alternative names that map to the same character
/// as a canonical unicode-math symbol. These are LaTeX synonyms, ASCII
/// shorthands, or names from other conventions (e.g. Lambdapi, Lean, Agda).
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
    ]
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Build the unicode input map (prefix -> character)
    let mut unicode_input: HashMap<String, String> = HashMap::new();

    // Load curated symbols (canonical unicode-math names)
    for (name, symbol) in curated_symbols() {
        unicode_input.entry(name.to_string()).or_insert_with(|| symbol.to_string());
    }

    // Load aliases (traditional LaTeX names, ASCII shorthands)
    for (alias, symbol) in aliases() {
        unicode_input.entry(alias.to_string()).or_insert_with(|| symbol.to_string());
    }

    // Optionally load the full unicode-math symbol table
    if cli.include_all_symbols {
        let json_str = include_str!("unicode-math-symbols.json");
        if let Ok(all_symbols) = serde_json::from_str::<HashMap<String, String>>(json_str) {
            for (name, symbol) in all_symbols {
                if !name.is_empty() && !symbol.is_empty() {
                    unicode_input.entry(name).or_insert(symbol);
                }
            }
        }

        // Also load latex-unicode.json for additional aliases
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
        Vec::new(),       // no snippet-style completions
        unicode_input,
        etcetera::home_dir().unwrap().to_str().unwrap().into(),
    )
    .await;
}
