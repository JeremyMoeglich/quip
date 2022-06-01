use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"[\r\t\f\v ]+", logos::skip)]
    #[error]
    Error,
    #[token("\n")]
    Newline,
    #[token("fn")]
    Function,
    #[token("mut")]
    Mut,
    #[regex("\"[^\"]*\"")]
    String,
    #[regex("[0-9]+")]
    Integer,
    #[regex("[0-9]*\\.[0-9]+")]
    Float,
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Exclamation,
    #[token("?")]
    Question,
    #[token("!?")]
    Panic,
    #[token("..")]
    Range,
    #[token("...")]
    Spread,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[regex("//[^\n]*", priority = 99)]
    Comment
}



fn main() {
    let test_content = r#"
        x = 5
        lst = [1, 2, 3]

        fn add_5(n) {
            return n + 5
        }

        // This is a comment
        fn add_1_to_each(lst) {
            for i in lst {
                i = i + 1
            }
        }

        println(add_5(x), lst)
        add_1_to_each(lst)
        println(lst, lst.sum())
    "#;
    print!("{}", test_content);
    let mut lexer = Token::lexer(test_content);
    while let Some(token) = lexer.next() {
        println!("{:?} - {}", token, lexer.slice().replace("\n", "\\n"));
    }
}
