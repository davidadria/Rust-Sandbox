enum Mensaje {
    Apagado,
    Reproduciendo(String),
    Pausado,
    CambiarVolumen(u8), // 0 - 100
}

fn main() {
    println!("REPRODUCTOR MULTIMEDIA");

    let msg1 = Mensaje::Reproduciendo(String::from("Crystal Mountains"));
    let msg2 = Mensaje::CambiarVolumen(80);
    let msg3 = Mensaje::Pausado;
    let msg4 = Mensaje::Apagado;

    let array = [msg1, msg2, msg3, msg4];

    for mensaje in array {
        procesar_mensaje(mensaje);
    }
}

fn procesar_mensaje(msg: Mensaje) {
    match msg {
        Mensaje::Apagado => println!("El reproductor esta apagado"),
        Mensaje::Reproduciendo(cancion) => println!("Reproduciendo cancion {}", cancion),
        Mensaje::Pausado => println!("Cancion pausada"),
        Mensaje::CambiarVolumen(vol) => println!("Mostrando el nivel de volumen {}", vol),
        _ => println!("Valor no valido encontrado"),
    }
}
