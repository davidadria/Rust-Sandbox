use std::io;

fn main() {
    // declaracion de variables
    // lee solo String
    let mut entrada = String::new();

    println!("CALCULADORA");

    // importante: si pones print en lugar de println
    // el programa se retrasa y no lo carga hasta tarde
    println!("Inserta un numero: ");

    // leer la entrada, unwrap hace que el sistema "paniquee"
    // y se cierre de golpe con un mensaje generico
    io::stdin().read_line(&mut entrada).unwrap();

    // trim() -> quitar espacios
    // parse() -> cambiar el valor de String al indicado en la variable (i32 aqui)
    // expect() -> mensaje de error personalizado si algo va mal
    let num1: i32 = entrada.trim().parse().expect("Introduce un num valido");

    // limpiar el valor de la entrada para reutilizarla
    entrada.clear();

    println!("Inserta otro numero: ");
    io::stdin().read_line(&mut entrada); // unwrap quitado para comprobar el mensaje de error

    let num2: i32 = entrada.trim().parse().expect("Introduce un num valido");

    // mostrar numeros introducidos
    println!("\nNum 1 ->{}\tNum 2 ->{}", num1, num2);

    let suma = num1 + num2;
    let resta = num1 - num2;
    let mult = num1 * num2;
    let divi = num1/num2;

    println!(
        "\nRESULTADOS: 
        \nSuma -> {}
        \nResta -> {}
        \nMultiplicacion -> {}
        \nDivision -> {}",
    suma, resta, mult, divi)
}
