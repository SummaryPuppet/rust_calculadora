use std::io;

struct Calculadora {
    num1: u32,
    num2: u32,
}

impl Calculadora {
    fn sumar(&self) -> u32 {
        self.num1 + self.num2
    }

    fn restar(&self) -> u32 {
        self.num1 - self.num2
    }

    fn multiplicar(&self) -> u32 {
        self.num1 * self.num2
    }

    fn dividir(&self) -> u32 {
        self.num1 / self.num2
    }
}

fn main() {
    let stdin = io::stdin();

    let mut operacion = String::new();

    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Hola a mi calculadora");
    println!("Dime el primer numero");

    stdin.read_line(&mut num1).expect("Fail");

    println!("Dime el segundo numero");

    stdin.read_line(&mut num2).expect("Fail");

    let calculadora = Calculadora {
        num1: num1.trim().trim().parse().expect("No integer"),
        num2: num2.trim().trim().parse().expect("No integer"),
    };

    println!("Que operacion desea hacer?");
    println!("1 sumar");
    println!("2 restar");
    println!("3 multiplicar");
    println!("4 dividir");

    stdin.read_line(&mut operacion).expect("Fail");
    operacion = operacion.trim().trim().parse().expect("Fail");

    if operacion == "1" {
        println!("Su suma es: {}", calculadora.sumar());
    } else if operacion == "2" {
        println!("Su resta es: {}", calculadora.restar());
    } else if operacion == "3" {
        println!("Su multiplicacion es: {}", calculadora.multiplicar());
    } else if operacion == "4" {
        println!("Su division es: {}", calculadora.dividir());
    } else {
        println!("No funciono intentelo de nuevo");
    }
}
