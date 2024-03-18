use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
#[logos(skip r"\s", error = String)]
pub enum Token {
    #[token("and")]
    And,
    #[token("or")]
    Or,

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    #[regex("[a-zA-Z]+", |l| l.slice().to_owned() )]
    Str(String),

    #[regex(r"\d+", |l| l.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token("\n", priority = 3)]
    NewLine,
}
