# Development practices

This article describes some development practices used in the project.

# Code style

## File level

### try to keep size of the source file small

Ideally, the maximum file size is good to have in `[300,500]` range of lines in total.


### use `*` import to avoid long import lines.

Advantages:
* shorter import
* less lines in total

Disadvantages:
* `it is bad for version control`: it’s harder to track what has been added to the local file namespace.
  Although it is valid, I believe it is not a big issue.
* `it can lead to unnecessary naming collisions`.  Can be solved using aliasing (`alias`/`as` keywords)

__NOTE__: on crate level, [preludes](https://doc.rust-lang.org/beta/reference/names/preludes.html) can be used to have a
collection of names that are automatically brought into scope of every module in a crate.

## Function level


### prefer functional style over imperative

- declarative approach which describes `what to do` rather `how to do`
- more readability as code is naturally grouped.

For example, use list comprehensions over loops:
```rust
let mut sum = 0;
for i in 1..11 {
    sum += i;
}
println!("{}", sum);
```
  vs

```rust
println!("{}", (1..11).fold(0, |a, b| a + b));
```

### prefer linear style to multiple one-several lines functions which are called just once

Advantages (personal taste):
- code is easier to follow (fewer jumps here and there over code base)
- call stack is less nested, so debug is easier
- benefit of not making it possible to call the function from other places

However, this is not a hard rule. In some cases, you might prefer to split:
- multiple usages
- separate function provides a good abstraction over complex logic
- make sense to test it separately
- ..

In general, don’t be over-eager to abstract, or offended by a few lines of duplication. Premature abstraction often
ends up coupling code that should not have to evolve together.

Please note, that this is not about proposing a single 1000-lines god function.

Additional resources:
- http://number-none.com/blow/blog/programming/2014/09/26/carmack-on-inlined-code.html


## Code organization level


### prefer directory/file hierarchy over flat structure


### use variable name shadowing

This helps to reduce hassle in some degree by allowing:
- reusing variable names rather than creating unique ones;
- transforming variables without making them mutable;
- converting type without manually creating two variables of different types (compiler does it automatically)


## Comments


### write comments on public api

It is enforced by `#![warn(missing_docs)]`


### comment non trivial logic, use `NOTE` if necessary


### use `TODO` prefix to signalize about missing implementation


# Toolchain

## General

### use code formatter

Cargo formatter can be used:

    cargo fmt

Please note, that the project has some default rules in overridden. Check `.rustfmt.toml` file for details.


### use static code analyzer

Cargo clippy is default tool:

    cargo clippy --all-features -- -D warnings

This command runs clippy tool with the setting which interprets all warning as errors. This should be a default strategy.


### automate some steps on CI

- run unit/component/feature tests
- measure code coverage


### analyze hot-spots

The following command is useful to get a list of most modified files for last two years according to the git history:

    git log --pretty=format: --since="2 years ago" --name-only -- "*.rs" | sed '/^\s*$/d' | sort | uniq -c | sort -rg | head -20

The idea is to look closely at top of the list and refactor the big files to have more fine-grained and isolated abstractions
in separate modules (and files). This will help to reduce amount of changes in the same files, potentially done simultaneously
by the different team members.