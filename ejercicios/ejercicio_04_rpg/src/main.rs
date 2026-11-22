// definir struct
struct Personaje {
    nombre: String,
    nivel: u32,
    vida: u32,
    experiencia: u32,
}

fn main() {
    println!("RPG");

    let mut personaje = build_personaje("Elma Goloco");

    mostrar_estado(&personaje); // mas adelante detalles del borrowing

    println!("Vida actual del personaje -> {}", personaje.vida);
    recibir_daño(&mut personaje, 50);
    println!("Vida actual del personaje -> {}", personaje.vida);

    ganar_experiencia(&mut personaje, 130);

    mostrar_estado(&personaje);
}

// crear un personaje rapidamente
fn build_personaje(nombre: &str) -> Personaje {
    Personaje {
        nombre: nombre.to_string(), // mejorable en un futuro con impl
        nivel: 1,
        vida: 100,
        experiencia: 0,
    }
}

// mejorable con iterator a futuro
fn mostrar_estado(personaje: &Personaje) {
    println!(
        "\nPersonaje:
        \nNombre -> {}
        \nNivel -> {}
        \nVida -> {}
        \nExperiencia -> {}
        ",
        personaje.nombre, personaje.nivel, personaje.vida, personaje.experiencia
    );
}

// mas detalles de ownership y borrowing a futuro
// devuelve vida actual
fn recibir_daño(personaje: &mut Personaje, cantidad: u32) {
    // saturating_sub por si llega a 0, se queda en 0 sin dar error
    personaje.vida = personaje.vida.saturating_sub(cantidad);
}

// subir de nivel cada 100 xp
fn ganar_experiencia(personaje: &mut Personaje, exp: u32) {
    personaje.experiencia += exp; // suma el total

    // si debe subir de nivel
    if personaje.experiencia >= 100 {
        personaje.nivel += 1; // se suma
        personaje.experiencia -= 100; // y se guarda la xp restante
    }
}
