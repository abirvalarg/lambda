pub fn tokenize(mut input: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    input = input.trim();
    if let Some((data, _)) = input.split_once('#') {
        input = data.trim();
    }
    let input = input.chars();
    let mut token_buffer = String::new();
    let mut raw_tokens = Vec::new();
    let mut result = Vec::new();
    for ch in input {
        if ch.is_ascii_whitespace() && ch != '\n' && !token_buffer.is_empty() {
            raw_tokens.push(token_buffer.clone());
            token_buffer.clear();
        } else {
            match ch {
                _ => token_buffer.push(ch)
            }
        }
    }
    Ok(result)
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Directive(Directive),
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
pub enum Directive {
    Prompt(String)
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Literal {
    Unsigned(u32)
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Operator {
    Lambda,
    Arrow,
    TypeCast,
    Bracket(char)
}
