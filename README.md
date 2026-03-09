# zed-unicode

A Zed extension for typing Unicode symbols using LaTeX-style prefixes. Type `lambda` and get `λ`, type `->` and get `→`, type `forall` and get `∀`.

Forked from [aripiprazole/zed-unicode](https://github.com/aripiprazole/zed-unicode) with rewritten symbol mappings based on the [unicode-math](https://ctan.org/pkg/unicode-math) LaTeX package.

## Usage

Install the extension, then start typing a symbol name in any supported file. Completions appear automatically in Zed's completion menu.

| You type | You get | Name source |
|----------|---------|-------------|
| `rightarrow` | → | unicode-math |
| `->` | → | alias |
| `forall` | ∀ | unicode-math |
| `lambda` | λ | alias (traditional LaTeX) |
| `muplambda` | λ | unicode-math (canonical) |
| `vdash` | ⊢ | unicode-math |
| `\|-` | ⊢ | alias |
| `coloneq` | ≔ | unicode-math |
| `:=` | ≔ | alias |
| `BbbN` | ℕ | unicode-math |
| `bbN` | ℕ | alias |

## Symbol naming

Symbol names follow the [unicode-math](https://github.com/latex3/unicode-math) package conventions. These are the canonical names from `unicode-math-table.tex`, the standard symbol table for XeLaTeX/LuaLaTeX mathematical typesetting.

Common alternative names are provided as **aliases**:

- **Traditional LaTeX** &mdash; `alpha`, `lambda`, `Gamma` (aliases for unicode-math's `mupalpha`, `muplambda`, `mupGamma`)
- **ASCII shorthands** &mdash; `->`, `=>`, `|->`, `|-`, `|=`, `:=`, `<=`, `>=`
- **Synonyms** &mdash; `land`/`lor` (for `wedge`/`vee`), `lnot` (for `neg`), `emptyset` (for `varnothing`)

## Symbols

The default set (~220 symbols) covers:

- **Greek letters** &mdash; all lowercase and uppercase, plus variants
- **Arrows** &mdash; `rightarrow`, `Rightarrow`, `hookrightarrow`, `mapsto`, `twoheadrightarrow`, ...
- **Logic & proof** &mdash; `forall`, `exists`, `neg`, `wedge`, `vee`, `top`, `bot`, `vdash`, `vDash`, ...
- **Relations** &mdash; `equiv`, `neq`, `leq`, `geq`, `approx`, `sim`, `cong`, `prec`, `succ`, `coloneq`, ...
- **Set theory** &mdash; `in`, `notin`, `subset`, `cup`, `cap`, `varnothing`, ...
- **Operators** &mdash; `times`, `cdotp`, `oplus`, `otimes`, `pm`, `circ`, ...
- **Blackboard bold** &mdash; `BbbN`, `BbbZ`, `BbbQ`, `BbbR`, `BbbC`, ...
- **Sub/superscripts** &mdash; `_0` .. `_9`, `^0` .. `^9`, `^-1`, `_n`, `^n`, ...
- **Misc** &mdash; `infty`, `partial`, `nabla`, `sum`, `prod`, `int`, `ldots`, `ell`, `aleph`, ...
- **Delimiters** &mdash; `langle`, `rangle`, `lceil`, `rceil`, `lfloor`, `rfloor`, `llbracket`, `rrbracket`

### Full unicode-math mode

To enable all 2,400+ symbols from the unicode-math table plus additional aliases from [unicode-latex](https://github.com/ViktorQvarfordt/unicode-latex), add to your Zed `settings.json`:

```json
{
  "lsp": {
    "unicode": {
      "settings": {
        "include_all_symbols": true
      }
    }
  }
}
```

## Supported languages

Completions are available in 70 languages including Lambdapi, LaTeX, Lean, Haskell, OCaml, Julia, Rust, Python, and more. See `extension.toml` for the full list.

## Architecture

The extension has two parts:

- **Zed extension** (`src/lib.rs`) &mdash; WASM module that manages the LSP binary lifecycle
- **LSP server** (`unicode-ls/`) &mdash; standalone Rust binary built on [simple-completion-language-server](https://github.com/zed-industries/simple-completion-language-server) that serves Unicode completions

## Development

```bash
cargo build -p unicode-ls --release    # build the LSP server
cp target/release/unicode-ls ~/.local/bin/   # install locally
```

The extension finds `unicode-ls` via `$PATH` before attempting to download from GitHub releases.

To install the extension for development in Zed: `zed: install dev extension` and select this directory.

## License

MIT
