fn tokenize(mut input: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    input = input.trim();
    if let Some((data, _)) = input.split_once('#') {
        input = data.trim();
    }
    let input = input.chars();
    let mut token_buffer = String::new();
    let mut result = Vec::new();
    for ch in input {
        if "{[()]}".contains(ch) {
            result.push(Token::Operator(Operator::Bracket(ch)));
        } else if ch == '\\' {
            result.push(Token::Operator(Operator::Lambda));
        } else if ch == '-' {
            token_buffer.push(ch);
        } else if ch == '>' && token_buffer == "-" {
            result.push(Token::Operator(Operator::Arrow));
            token_buffer.clear();
        }
    }
    Ok(result)
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Macro(Macro),
    Literal(Literal),
    Variable(String),
    Operator(Operator)
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Keyword {
    Input,
    Output
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Macro {
    Prompt(String)
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Literal {
    Unsigned(u32),
    String(String)
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Operator {
    Lambda,
    Arrow,
    TypeCast,
    Bracket(char)
}
