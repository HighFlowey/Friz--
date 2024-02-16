use std::{fs::read_to_string, ops::Index, path::PathBuf};
use clap::Parser as ClapParser;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
enum UserType {
    String(String),
    Int(i32),
}

#[derive(Debug, PartialEq)]
enum TokenType {
    UserType(UserType),
    Print,
    Let,
    To,
    OpenParen,
    CloseParen,
    Comma,
}

enum ParserError <'a> {
    Ok,
    Err(&'a str)
}

#[derive(Debug)]
enum StmtType <'a> {
    Print(Vec<UserType>),
    Let {
        key: &'a TokenType,
        value: &'a TokenType,
    },
}

struct Tokenizer <'a> {
    content: &'a String,
    tokens: Vec<TokenType>,
    index: usize,
}

impl Tokenizer <'_> {
    fn new(content: &String) -> Tokenizer {
        return Tokenizer {
            content: content,
            tokens: Vec::new(),
            index: 0,
        }
    }

    fn run(self: &mut Self) {
        let buffer = &mut String::new();

        while self.peek(None).is_some() {
            if self.peek(None).unwrap().is_alphabetic() {
                self.accept_to_buffer(buffer);

                while self.peek(None).is_some() && self.peek(None).unwrap().is_alphanumeric() {
                    self.accept_to_buffer(buffer);
                }

                if buffer == "print" {
                    self.tokens.push(TokenType::Print);
                } else if buffer == "let" {
                    self.tokens.push(TokenType::Let);
                } else if buffer == "to" {
                    self.tokens.push(TokenType::To);
                } else {
                    self.tokens.push(TokenType::UserType(UserType::String(buffer.to_string())))
                }

                buffer.clear();
            } else if self.peek(None).unwrap().is_numeric() {
                self.accept_to_buffer(buffer);

                while self.peek(None).is_some() && self.peek(None).unwrap().is_numeric() {
                    self.accept_to_buffer(buffer);
                }

                let user_type = UserType::Int(buffer.parse().unwrap());
                self.tokens.push(TokenType::UserType(user_type));
                buffer.clear();
            } else if self.peek(None).unwrap() == '(' {
                self.accept();

                self.tokens.push(TokenType::OpenParen);
            } else if self.peek(None).unwrap() == ')' {
                self.accept();

                self.tokens.push(TokenType::CloseParen);
            } else if self.peek(None).unwrap() == ',' {
                self.accept();

                self.tokens.push(TokenType::Comma);
            } else {
                self.accept();
            }
        }
    }

    fn peek(self: &Self, _offset: Option<usize>) -> Option<char> {
        let offset = if _offset.is_some() { _offset.unwrap() } else { 0 };

        if self.index + offset < self.content.len() {
            let str = &self.content[(self.index + offset)..(self.index + offset + 1)];
            return str.chars().last();
        } else {
            return None;
        }
    }

    fn accept(self: &mut Self) -> Option<char> {
        self.index += 1;
        return self.content[self.index - 1 .. self.index].chars().last();
    }

    fn accept_to_buffer(self: &mut Self, buffer: &mut String) {
        buffer.push(self.accept().unwrap());
    }
}

struct Parser <'a> {
    tokens: &'a Vec<TokenType>,
    stmts: Vec<StmtType<'a>>,
    index: usize,
}

impl Parser <'_> {
    fn parse_stmt(self: &mut Self) -> ParserError {
        if TokenType::Print == *self.peek(None).unwrap() {
            self.accept();

            if self.peek(None).is_some() && TokenType::OpenParen == *self.peek(None).unwrap() {
                let mut user_types: Vec<UserType> = Vec::new();

                self.accept();
                
                while self.peek(None).is_some() && matches!(self.peek(None).unwrap(), TokenType::UserType(_)) {
                    let TokenType::UserType(x) = self.peek(None).unwrap() else {
                        return ParserError::Err("Expected type Int inside print statement");
                    };

                    user_types.push(x.clone());

                    self.accept();

                    if self.peek(None).is_some() && TokenType::Comma == *self.peek(None).unwrap() {
                        // found comma, gonna keep looking for values
                        self.accept();
                    } else {
                        // didn't find comma, gonna stop now
                        break;
                    }
                }

                if self.peek(None).is_some() && TokenType::CloseParen == *self.peek(None).unwrap() {
                    self.accept();

                    // println!("{:?}", user_types);

                    self.stmts.push(StmtType::Print(user_types));
                } else {
                    return ParserError::Err("Expected ')' to end print statement")
                }
            } else {
                return ParserError::Err("Expected '(' to start print statement")
            }
        } else if TokenType::Let == *self.peek(None).unwrap() {
            self.accept();

            if self.peek(None).is_some() && matches!(self.peek(None).unwrap(), TokenType::UserType(UserType::String(_))) {
                let key_token = self.tokens.index(self.index);
                self.accept();

                if self.peek(None).is_some() && TokenType::To == *self.peek(None).unwrap() {
                    self.accept();

                    if self.peek(None).is_some() && matches!(self.peek(None).unwrap(), TokenType::UserType(_)) {
                        let value_token = self.tokens.index(self.index);
                        self.accept();
                        
                        let stmt = StmtType::Let { key: key_token, value: value_token };
                        self.stmts.push(stmt);
                    } else {
                        return ParserError::Err("Expected value after 'to'")
                    }
                } else {
                    return ParserError::Err("Expected 'to' after variable name")
                }
            } else {
                return ParserError::Err("Expected variable name after 'let'")
            }
        } else {
            self.accept();
        }

        return ParserError::Ok;
    }

    fn run(self: &mut Self) -> ParserError {
        while self.peek(None).is_some() {
            match self.parse_stmt() {
                ParserError::Ok => { continue; }
                ParserError::Err(err) => { println!("!!! -> Erorr while parsing: {}", err) }   
            }
        }

        return ParserError::Ok;
    }

    fn peek(self: &Self, _offset: Option<usize>) -> Option<&TokenType> {
        let offset = if _offset.is_some() { _offset.unwrap() } else { 0 };

        if self.index + offset < self.tokens.len() {
            let token = self.tokens.index(self.index);
            return Some(token);
        } else {
            return None;
        }
    }

    fn accept(self: &mut Self) {
        self.index += 1;
    }
}

struct Generator <'a> {
    stmts: &'a Vec<StmtType<'a>>,
    index: usize,
}

fn include(mut includes: String, include: &str) -> String {
    if !includes.contains(&String::from(include)) {
        includes += "#include ";
        includes += include;
        includes += "\n";
    }

    return includes;
}

impl Generator <'_> {
    fn generate(&mut self) -> String {
        let mut includes = String::new();
        let mut src = String::new();
        src += "int main() {\n";

        while self.peek(None).is_some() {
            let stmt = self.peek(None).unwrap();

            match stmt {
                StmtType::Print(user_types) => {
                    includes = include(includes, "<iostream>");
                    src += "std::cout<<";

                    for user_type in user_types.into_iter() {
                        match user_type {
                            UserType::Int(x) => {
                                src += x.to_string().as_str();
                                src += "<<";
                            }
                            UserType::String(x) => {
                                src += x.as_str();
                                src += "<<";
                            }
                        }
                    }

                    src += "std::endl;\n";
                }
                StmtType::Let { key, value } => {
                    match key {
                        TokenType::UserType(key_ut) => {
                            match key_ut {
                                UserType::String(key_string) => {
                                    match value {
                                        TokenType::UserType(value_ut) => {
                                            match value_ut {
                                                UserType::String(value_string) => {
                                                    includes = include(includes, "<string>");
                                                    src += "std::string ";
                                                    src += key_string;
                                                    src += "=";
                                                    src += value_string;
                                                    src += ";\n";
                                                }
                                                UserType::Int(value_int) => {
                                                    src += "int ";
                                                    src += key_string;
                                                    src += "=";
                                                    src += value_int.to_string().as_str();
                                                    src += ";\n";
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }

            self.accept();
        }
        
        src += "}";

        return format!("{}\n{}", includes, src);
    }

    fn peek(self: &Self, _offset: Option<usize>) -> Option<&StmtType> {
        let offset = if _offset.is_some() { _offset.unwrap() } else { 0 };

        if self.index + offset < self.stmts.len() {
            let stmt = self.stmts.index(self.index);
            return Some(stmt);
        } else {
            return None;
        }
    }

    fn accept(self: &mut Self) -> &StmtType {
        self.index += 1;
        return self.stmts.index(self.index - 1);
    }
}

#[derive(ClapParser)]
struct Cli {
    file: PathBuf,
}

fn main() {
    #[allow(unused_assignments)]
    let mut content = String::new();
    let mut cli = Cli::parse();

    if cli.file.is_file() {
        content = read_to_string(cli.file).unwrap();
    } else {
        cli.file.push("init.zynk");
        content = read_to_string(cli.file).unwrap();
    }

    let mut tokenizer = Tokenizer::new(&content);
    tokenizer.run();

    let mut parser = Parser { tokens: &tokenizer.tokens, index: 0, stmts: Vec::new() };
    let parser_result = parser.run();

    match parser_result {
        ParserError::Ok => {
            println!("");
            println!("          ⇊     User input   ⇊");
            println!("----- Zynk ----------------------");
            println!("{}", content);
            println!("----- Zynk ----------------------");

            let mut generator = Generator { stmts: &parser.stmts, index: 0 };
            println!("");
            println!("          ⇊ Compiler results ⇊");
            println!("----- C++ -----------------------");
            println!("{}", generator.generate());
            println!("----- C++ -----------------------");
        }
        ParserError::Err(err) => {
            println!("!!! -> Error: {}", err);
        }
    }
}
