use super::{KeyWord, Scanner, TokenType};

const FILENAME: &str = "scan-test";
#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn scanner_main() {
    let src = r#"
fn main() {
    return 0;
}
"#;
    let mut scanner = Scanner::new(FILENAME, src);
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::KeyWord(KeyWord::Fn))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("main".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('('))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(')'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('{'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::KeyWord(KeyWord::Return))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Int("0".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(';'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('}'))
    );
}

#[test]
fn scanner_add_func() {
    let src = r#"
fn add(x: u64, y: u64) -> u64 {
    return x + y;
}
fn main() {
    let x = add(123, 321);
    return 0;
}
"#;
    let mut scanner = Scanner::new(FILENAME, src);
    // Add FUNC
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::KeyWord(KeyWord::Fn))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("add".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('('))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("x".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(':'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("u64".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(','))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("y".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(':'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("u64".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(')'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::RArrow)
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("u64".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('{'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::KeyWord(KeyWord::Return))
    );

    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("x".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Op("+".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("y".into()))
    );

    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(';'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('}'))
    );

    // MAIN FUNC
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::KeyWord(KeyWord::Fn))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("main".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('('))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(')'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('{'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::KeyWord(KeyWord::Let))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("x".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('='))
    );

    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Id("add".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('('))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Int("123".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(','))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Int("321".into()))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(')'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(';'))
    );

    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::KeyWord(KeyWord::Return))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Int("0".into()))
    );

    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl(';'))
    );
    assert_eq!(
        scanner.next().ok().map(|t| t.tok_type().clone()),
        Some(TokenType::Ctrl('}'))
    );
}
