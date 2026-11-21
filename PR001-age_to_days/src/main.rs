use std::io;

fn main() {
    println!("Proyecto 001 -> From years to days");
    let mut age_by_text = String::new();

    println!("Introduce un numero: ");
    io::stdin()
        .read_line(&mut age_by_text)
        .expect("Debes insertar un num");

    let age: i32 = match age_by_text.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No has insertado un numero");
            0 // devuelve 0 en caso de error
        }
    };

    let days = age * 365;

    println!(
        "Edad: {}
        \nDias: {}",
        age, days
    );
}
