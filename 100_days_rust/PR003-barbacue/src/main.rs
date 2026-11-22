fn main() {
    println!("BARBACUE");

    // x -> meat
    // o -> vegetarian
    let cadena = [
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ];

    // pasamos una vista de la cadena
    vegetarian(&cadena);
}

// recibe un array de la vista
fn vegetarian(cadena: &[&str]) {
    let mut cont1 = 0; // carne
    let mut cont2 = 0; // vegetal

    for elemento in cadena {
        if elemento.contains('x') {
            cont1 += 1;
        } else {
            cont2 += 1;
        }
    }

    // nCarne, nVegetal
    let contador = [cont1, cont2];
    println!("{:?}", contador);
}
