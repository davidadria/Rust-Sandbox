use std::io::{self, Write};

fn main() {
    println!("NAVE ESPACIAL");

    loop {
        // estados
        // voy a no complicarme con HashMap aun, lista normal
        let lista_estados = ["Motores", "Escudos", "SVB"];

        // menu principal
        print!(concat!(
            "\nCONSOLA\n",
            "Escribe tu orden (salir, estado o despegue): "
        ));

        // para poder usar print normal, le indica al programa que borre el buffer
        io::stdout().flush().unwrap();

        // declaracion de variable y leer valor
        let mut texto_usuario = String::new();
        io::stdin().read_line(&mut texto_usuario).unwrap();

        // match que gestiona lo puesto
        match texto_usuario.as_str().trim() {
            "salir" => break,
            "estado" => {
                println!("Comprobando estado...");
                for estado in lista_estados {
                    println!("{} ACTIVO", estado);
                }
            }
            "despegue" => {
                println!("Despegando...");
                let mut contador = 3;
                while contador > 0 {
                    println!("{}...", contador);
                    contador = contador - 1;
                }

                let combustible = 80;

                if combustible > 50 {
                    println!("Cohete despegado");
                } else {
                    println!("No hay combustible suficiente, el cohete se queda en casa");
                }
            }
            _ => println!("Comando no valido"),
        };
    }
    println!("FIN");
}
