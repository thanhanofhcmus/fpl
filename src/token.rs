use logos::Logos;

pub type TokenResult = Result<Token, String>;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"\s", error = String)]
pub enum Token {
    #[token("and")]
    And,
    #[token("or")]
    Or,

    #[token("nil")]
    Nil,

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    #[regex(r"\d+", |l| l.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[regex("[a-zA-Z_$][a-zA-Z_$0-9]*", |l| l.slice().to_owned())]
    Identifier(String),

    #[regex(r#""([^"\\]|\\.|"")*""#, |l| l.slice().to_owned(), priority=10)]
    Str(String),

    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("do")]
    Do,
    #[token("end")]
    End,

    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    BangEqual,

    #[token("\n", priority = 3)]
    NewLine,
}
