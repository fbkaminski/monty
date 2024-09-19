use lex::scanner::Scanner;

fn main() {
    //let code = "string s = 'hello world';\nif s.lenght() > 4 {\n print('hello');\n} else {\n print('ola');\n}\n";
    let code = "(2 + 2) / 3 = ?\n";
    let mut s = Scanner::new(code, 0);
    s.init();
    let mut token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
    token = s.next();
    println!("scan_token: {}", token);
}
