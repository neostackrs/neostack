# NML Language Support for VS Code

Syntax highlighting, snippets, and language configuration for **NeoStack Markup Language** (`.nml`).

## Features

- Full syntax highlighting for all NML sigils (`@`, `$`, `%`, `&`, `=`, `{}`)
- All v1 directives highlighted: `=macro`, `=set`, `=assign`, `=include`, `=extends`, `=block`
- All reserved post-v1 directives recognized (control flow, error handling, state, assets, i18n, caching, dev tooling)
- String interpolation `${}` highlighted inside single, double, and backtick strings
- Text content interpolation `{variable}` highlighted
- Auto-indent on block directives (`=macro`, `=block`, `=if`, etc.)
- Code folding between block directive openers and `=end`
- Snippets for all v1 directives and element forms
- Comment toggling with `//`

## Installation

Install via the VS Code Marketplace (coming soon) or clone the NeoStack monorepo and run:

```bash
cd packages/nml-vscode
npm install
npm run package
code --install-extension nml-language-0.1.0.vsix
```

## Snippets

| Prefix     | Output                          |
|------------|---------------------------------|
| `=macro`   | Macro definition block          |
| `=set`     | Inline set                      |
| `=setb`    | Block set                       |
| `=assign`  | Inline assign                   |
| `=assignb` | Block assign                    |
| `=include` | Include partial (dot notation)  |
| `=extends` | Extends layout (path notation)  |
| `=block`   | Block content slot              |
| `@tag`     | Inline element with content     |
| `@empty`   | Empty paired element            |
| `$void`    | Void/self-closing element       |
| `$block`   | Block element with open/close   |
| `$style`   | Style block                     |
| `$script`  | Script block                    |
| `&macro`   | Macro call                      |
| `{`        | Text interpolation              |
