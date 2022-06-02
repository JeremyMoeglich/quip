use logos::Logos;
mod lexer;
mod ast;
mod pest_parser;
#[macro_use]
extern crate pest_derive;

fn main() {
    let test_content = r#"
// This is a comment
fn get_current_ip() -> Result<ip.Ip, TypeException<"Ip must be string"> | NetworkException<"Connection timed out" | "Not connected"> | FormatException<"Ip has wrong format"> | EnvError<"http request not available">> {
    response: Response = fetch('https://ip.moeglich.dev')?
    ip_string: JsonValue = json.parse(response.body)?
    if !(ip_string is string) {
        throw TypeException("Ip must be string")
    }
    ip: ip.Ip = ip.parse(ip_string)?
    return ip
}
    "#;
    print!("{}", test_content);
    let mut lexer = lexer::Token::lexer(test_content);
    let tokens = lexer.collect::<Vec<lexer::Token>>();
    for token in tokens {
        println!("{:?}", token);
    }
}
