# zed-unicode

A Zed extension for typing Unicode symbols using LaTeX-style prefixes. Type `lambda` and get `λ`, type `->` and get `→`, type `forall` and get `∀`.

Forked from [aripiprazole/zed-unicode](https://github.com/aripiprazole/zed-unicode) with rewritten symbol mappings, LaTeX-convention prefixes, and support for proof assistants.

## Usage

Install the extension, then start typing a symbol name in any supported file. Completions appear automatically in Zed's completion menu.

| You type | You get |
|----------|---------|
| `alpha`  | α       |
| `lambda` | λ       |
| `->`     | →       |
| `forall` | ∀       |
| `vdash`  | ⊢       |
| `:=`     | ≔       |
| `bbN`    | ℕ       |
| `_0`     | ₀       |
| `^-1`    | ⁻¹      |

## Symbols

The default set (~170 symbols) covers:

- **Greek letters** &mdash; `alpha` .. `omega`, `Gamma` .. `Omega`, variants (`varepsilon`, `varphi`, ...)
- **Arrows** &mdash; `->`, `<-`, `=>`, `<=>`, `|->`, `hookrightarrow`, `->>`, ...
- **Logic & proof** &mdash; `forall`, `exists`, `neg`, `land`, `lor`, `top`, `bot`, `vdash`, `turnstile`, ...
- **Relations** &mdash; `equiv`, `neq`, `leq`, `geq`, `approx`, `sim`, `cong`, `prec`, `succ`, ...
- **Set theory** &mdash; `in`, `notin`, `subset`, `cup`, `cap`, `emptyset`, ...
- **Operators** &mdash; `times`, `cdot`, `oplus`, `otimes`, `pm`, `circ`, ...
- **Blackboard bold** &mdash; `bbN`, `bbZ`, `bbQ`, `bbR`, `bbC`, `bbP`, `bbH`, `bbB`
- **Sub/superscripts** &mdash; `_0` .. `_9`, `^0` .. `^9`, `^-1`, `_n`, `^n`, ...
- **Misc** &mdash; `infty`, `partial`, `nabla`, `sum`, `prod`, `int`, `ldots`, `ell`, `aleph`, ...
- **Delimiters** &mdash; `langle`, `rangle`, `lceil`, `rceil`, `lfloor`, `rfloor`, `llbracket`, `rrbracket`

### Full Unicode mode

To enable all 2,500+ symbols from [unicode-latex](https://github.com/ViktorQvarfordt/unicode-latex) (covering nearly all of LaTeX's math symbol commands), add to your Zed `settings.json`:

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
