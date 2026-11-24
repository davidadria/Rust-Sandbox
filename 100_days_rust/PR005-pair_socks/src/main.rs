fn main() {
    println!("PAIR OF SOCKS");

    let socks1 = "AA"; // 1 par
    println!("{}", sock_pairs(socks1));

    let socks2 = "ABABC"; // 2 pares
    println!("{}", sock_pairs(socks2));

    let socks3 = "CABBACCC"; // 4 pares
    println!("{}", sock_pairs(socks3));
}

// me he adelantado para usar Vec
// https://doc.rust-lang.org/beta/std/vec/struct.Vec.html
// recibe una referencia para ver, no tiene permisos de modificar
fn sock_pairs(cadena: &str) -> i32 {
    // crear variables a usar, por defecto debe devolver 0
    let mut pares = 0;
    let mut sueltos: Vec<char> = Vec::new();

    // para cada caracter
    for letra in cadena.chars() {
        // crear estado, encontrado o no
        let mut encontrado = false;
        // indice de la letra encontrada,
        // inicializado para ahorrarme match-option
        let mut index = 0;

        // recorrer calcetines del vector sueltos para obtener el indice
        // mas adelante detalles de programacion funcional
        for (indice, &calcetin) in sueltos.iter().enumerate() {
            if calcetin == letra {
                // letra actual == alguna letra del vector
                encontrado = true; // se marca como encontrado
                index = indice; // se guarda el indice
                break; // rompe el bucle
            }
        }

        if encontrado {
            sueltos.remove(index); // quitar par de calcetin encontrado
            pares += 1;
        } else {
            sueltos.push(letra); // añadir calcetin suelto al vector
        }
    }

    pares
}
