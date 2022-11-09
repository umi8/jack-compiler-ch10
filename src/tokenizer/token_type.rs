#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}
