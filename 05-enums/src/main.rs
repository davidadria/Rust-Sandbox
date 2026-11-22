use std::slice::SplitInclusiveMut;

// definicion basica
enum IpAddrKing {
    V4,
    V6,
}

// enum para meter datos
enum IpAddr {
    V4(String),
    V6(String),
}

// ejemplo mas complejo
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Option, viene creado asi, sustituto de null con mas control
// se puede usar None y Some directamente
/*
enum Option<T> {
    None,
    Some(T),
}
*/

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // uso simple de enums
    let cuatro = IpAddrKing::V4;
    let seis = IpAddrKing::V6;

    // enviar un parametro tipo enum
    route(cuatro);
    route(seis);

    // crear datos a partir del enum
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // OPTION, puede ser algo o nada
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    // pasamos una moneda del enum para recibir su valor
    println!("{}", value_in_cents(Coin::Dime));

    let cinco = Some(5);
    let seis = plus_one(cinco); // devuelve Some(6)
    let ninguno = plus_one(None); // devuelve None
}

// recibir el un tipo enum
fn route(_tipo_ip: IpAddrKing) {}

// funcion con pattern matching
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// match pattern con Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // comprobar si recibe un valor
        None => None,
        Some(i) => Some(i + 1),
    }
}
