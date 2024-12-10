use parse::parser::Parser;
use jit::jit::Jit;

fn main() {
    //let code = "string s = 'hello world';\nif s.lenght() > 4 {\n print('hello');\n} else {\n print('ola');\n}\n";
    //let code = "#comment 1\n#comment 2";
    let code = "12 * 2";
    //let code = "if 1 elif 2 else 3 not good: bool\n";
    //let code = "abc: str = 'this is a string'";
    //let code = "abc: str = 'this is a string'\n  bcd: int = 1234\n    def add(a: int, b: int) -> int\n    return a + b";
//     let code = "
// use abc.dfg

// struct Calculator {
//     a: int
//     b: int
//     mode: str
// }

// impl Calculator {

//     fn new() {
//         self = Calculator{}
//         self.mode = \"simple\"
//         return self
//     }

//     fn sum(self, a: int, b: int) -> int {
//         return a + b
//     }

//     fn mul(self, a: int, b: int) -> int {
//         return a * b
//     }

//     fn div(self, a: int, b: int) -> int {
//         return a / b
//     }

// }

// fn main() {
//     calc = Calculator()
//     x = calc.add(2, 3)
//     print(x)
// }
// ";
    println!("{:?}", code);
    // let scanner = Scanner::new(code);
    // for token in scanner {
    //     match token {
    //         Token::Identifier {ref value} => {println!("{} {:?}", token, value)},
    //         Token::Number {ref value} => {println!("{} {:?}", token, value)},
    //         Token::String {ref value} => {println!("{} {:?}", token, value)},
    //         _ => {println!("{}", token)}
    //     }
    // }
    let mut parser = Parser::new(code);
    let jit = Jit::new();
    let expr = parser.parse().unwrap();
    let func = jit.compile_operation(expr.op);
    let left = expr.left.value.parse::<i32>().unwrap();
    let right = expr.right.value.parse::<i32>().unwrap();
    let result = func(left, right);
    println!("result: {} {} {} => {}", left, expr.op, right, result);
}
