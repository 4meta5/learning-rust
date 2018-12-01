# Macro_Rules

> this is really just a way to better organize my notes from the Little Macro Book (instead of keeping all notes in that README.md)

`macro_rules!` has the following form

```
macro_rules! $name {
    $rule0 ;
    $rule1 ;
    // ...
    $ruleN ;
}
```

Each rule looks like: `($pattern) => {$expansion}`

When a macro is invoked, the `macro_rules!` interpreter goes through the rules one at a time, in lexical order. For each rule, it tries to match the contents of the input token tree against that rule's `pattern`.

## Captures

Captures are written as a `$` followed by an identifier, a color `:` and the kind of capture, which must be one of the following:
* `item`: an item ie function, struct module, etc
* `block`: a block of statements and/pr an expression, surrounded by braces
* `stmt`: a statement
* `pat`: a pattern
* `expr`: an expression
* `ty`: a type
* `ident`: an identifier
* `path`: a path (e.g. `foo`, `::std::mem::replace, transmute::<_, int>`, ...)
* `meta`: a meta iteml the things that go inside `#[...]` and `#[...]` attributes
* `tt`: a single token tree

> Much like macro expansion, captures are substituted as complete AST nodes. Therefore, regardless of what sequence of tokens are captured, it will be interpreted as a single, complete expression

Remember that each rule looks like: `($pattern) => {$expansion}`. Likewise, each rule can contain *repetitions* that allow a sequence of tokens to be matched. Repetitions have the form `$ ( ... ) sep rep` such that `( ... )` is the paren-grouped pattern being matched, `sep` is the *optional* separator token (`,` or `;`), and `rep` is the *required* repeat control (EITHER `*` for zero or more repeats OR `+` for one or more repeats).

*Repetitions* can contain any valid pattern, including literal token trees, captures, and other repetitions.

`macro_rules!` restricts what can follow various captures...as of Rust 1.3
* `item`: anything
* `block`: anything
* `stmt`: `=>`, `,`, `;`
* `pat`: `=>`, `=` `if` `in`
* `expr`: `=>`, `,`, `;`
* `ty`: `,`, `=>` `:` `=` `>` `;` `as`
* `ident`: anything
* `path`: `,` `=>` `:` `=` `>` `;` `as`
* `meta`: anything
* `tt`: anything