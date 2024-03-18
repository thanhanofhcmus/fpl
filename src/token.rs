use logos::Logos;

pub type TokenResult = Result<Token, String>;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"\s", error = String)]
pub enum Token {
    #[token("and")]
    And,
    #[token("or")]
    Or,

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    #[regex(r#""[a-zA-Z]+""#, |l| l.slice().to_owned() )]
    Str(String),

    #[regex(r"\d+", |l| l.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[token("if")]
    If,
    #[token("then")]
    Then,
    #[token("else")]
    Else,
    #[token("end")]
    End,

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
