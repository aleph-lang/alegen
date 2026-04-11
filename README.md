# alegen

Generates Aleph source code from an [`AlephTree`](https://github.com/aleph-lang/aleph-syntax-tree).
The native round-trip generator: parse Aleph → transform → emit Aleph.

## Installation

```toml
[dependencies]
alegen = "0.1"
```

## Usage

```rust
let code = alegen::generate(ast);
```

## Example

```rust
let ast = AlephTree::Add {
    number_expr1: Box::new(AlephTree::Int { value: "3".into() }),
    number_expr2: Box::new(AlephTree::Int { value: "4".into() }),
};
let code = alegen::generate(ast);
// produces: "3 + 4"
```

## Related

- [`aleph-syntax-tree`](https://github.com/aleph-lang/aleph-syntax-tree) — AST definition
- [`aleparser`](https://github.com/aleph-lang/aleparser) — Aleph parser
- [`alephc`](https://github.com/aleph-lang/aleph) — uses this generator with `--features ale_gen`
