A transform plugin for [Blades](https://getblades.org) that renders formulas
using [KaTeX](https://katex.org).
Formulas delimited by `$` are rendered in inline mode and by `$$` in display mode.
Formulas are cached, so they don't have to be rendered multiple times.

This plugin can be installed as
```bash
cargo install blades-katex
```

Then, it can be used in Blades as
```toml
[plugins]
transform = ["blades-katex"]
```
