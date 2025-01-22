# parser-helper

A parser helper.
Often, we need to parse a string (or other sequence of bytes) when processing packets or commands.
This crate provides a convenient trait that allows you to do that with custom errors.

Consider using a lexer like [Logos](https://github.com/maciejhirsz/logos), which constructs a very efficient tree for tokenization.

Here are some reasons you would not use such a lexer for parsing data:
- tokenization is not useful (variable length captures not based on regex)
- tokenization is not practical (lots of implementations that differ)
- would be ill-suited (binary data instead of text)

## Cargo

I haven't pushed this to crates, so you can use the git repo instead:
```toml
[dependencies]
parser-helper = { git = "https://github.com/rapidrecast/parser-helper.git" }
```

## Usage

```rust
use parser_helper::ParserHelper;

fn main() {
    let input = "Some sequence of bytes";
    
    // Exact subsequence
    let (found, remaining) = input.take_until(" ".as_bytes()).unwrap();
    assert_eq!(found, "Some".as_bytes());
    assert_eq!(remaining, " sequence of bytes".as_bytes());
    
    // Exact size
    let (_space, remaining) = remaining.take_exact(1).unwrap();
    let (found, remaining) = remaining.take_expect("sequence".as_bytes()).unwrap();
    assert_eq!(found, "sequence".as_bytes());
    assert_eq!(remaining, " of bytes".as_bytes());
    
    // Variable size based on condition
    let func_contains_two_spaces = |b: &[u8]| b.iter().filter(|&&b| b == b' ').count() == 2;
    let (smallest, not_caught) = remaining.take_smallest_err(func_contains_two_spaces, "some custom error").unwrap();
    assert_eq!(smallest, " of ".as_bytes());
    assert_eq!(not_caught, "bytes".as_bytes());
    let (largest, not_caught) = remaining.take_largest_err(func_contains_two_spaces, "some custom error").unwrap();
    assert_eq!(largest, " of bytes".as_bytes());
    assert_eq!(not_caught, &[]);
}
```