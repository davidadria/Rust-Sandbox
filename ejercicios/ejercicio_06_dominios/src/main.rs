fn main() {
    println!("DOMINIOS");

    // PARTE 1
    let cadena = String::from("usuario@rustlang.es");

    let domain = extraer_dominio(&cadena);

    println!("{}", domain);

    // PARTE 2
    let mut cadena = String::from("usuario@rustlang.es");

    let domain = extraer_dominio(&cadena);

    // no se puede modificar algo mientras tiene un puntero inmutable
    // comentado para que compile
    // cadena.push_str(".com");

    println!("{}", domain);
}

// debe devolver rustlang.es en este caso
fn extraer_dominio(cadena: &String) -> &str {
    let parte = cadena.find('@');

    match parte {
        Some(v) => {
            let my_slice = cadena.trim();
            let dominio = &my_slice[v + 1..];

            &dominio[..]
        }
        None => {
            let error = "ERROR EN EL CORREO";
            error
        }
    }
}

/* Codigo mas optimo
fn extraer_dominio(cadena: &String) -> &str {
    match cadena.find('@') { // hace la comprobacion directamente
        Some(v) => {
            &cadena[v + 1..]
        }
        None => "ERROR EN EL CORREO",
    }
}
*/
