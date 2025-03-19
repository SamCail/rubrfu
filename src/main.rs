use std::fs::File;
use std::io::{self, Read, Write};
use std::collections::VecDeque;

#[derive(Default)]
struct Interpreter {
    tape: VecDeque<u8>,
    pointer: usize,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            tape: VecDeque::new(),
            pointer: 0,
        }
    }

    fn run(&mut self, program: &str) {
        let mut loop_stack = Vec::new();
        let program = program.chars().collect::<Vec<char>>();
        let mut pc = 0; // Program counter
        
        while pc < program.len() {
            match program[pc] {
                '>' => self.pointer += 1,
                '<' => self.pointer = self.pointer.saturating_sub(1),
                '+' => {
                    if self.tape.len() <= self.pointer {
                        self.tape.push_back(0);
                    }
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
                }
                '-' => {
                    if self.tape.len() <= self.pointer {
                        self.tape.push_back(0);
                    }
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
                }
                '.' => {
                    print!("{}", *self.tape.get(self.pointer).unwrap_or(&0) as char);
                    io::stdout().flush().unwrap();
                }
                ',' => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if let Some(first_char) = input.chars().next() {
                        self.tape[self.pointer] = first_char as u8;
                    }
                }
                '[' => {
                    if self.tape.get(self.pointer).unwrap_or(&0) == &0 {
                        let mut depth = 1;
                        while depth > 0 {
                            pc += 1;
                            if program[pc] == '[' {
                                depth += 1;
                            } else if program[pc] == ']' {
                                depth -= 1;
                            }
                        }
                    } else {
                        loop_stack.push(pc);
                    }
                }
                ']' => {
                    if self.tape.get(self.pointer).unwrap_or(&0) != &0 {
                        pc = *loop_stack.last().unwrap();
                    } else {
                        loop_stack.pop();
                    }
                }
                _ => {} // Ignore all other characters
            }
            pc += 1;
        }
    }
}

fn run_from_file(filename: &str) {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut program = String::new();
    file.read_to_string(&mut program).expect("Failed to read file");

    let mut interpreter = Interpreter::new();
    interpreter.run(&program);
}

fn repl() {
    let mut interpreter = Interpreter::new();
    println!("Brainfuck REPL. Type 'exit' to quit.");
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" {
            break;
        }

        interpreter.run(input);
    }
}

fn main() {
    println!("Select an option: ");
    println!("1. Run Brainfuck program from file");
    println!("2. Start Brainfuck REPL");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => {
            println!("Enter the file name: ");
            let mut filename = String::new();
            io::stdin().read_line(&mut filename).unwrap();
            let filename = filename.trim();
            run_from_file(filename);
        }
        "2" => repl(),
        _ => println!("Invalid choice!"),
    }
}
