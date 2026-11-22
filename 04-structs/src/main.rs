// declaracion
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Punto(i32, i32, i32);

// unit struct
struct AlwaysEqual;

fn main() {
    // instanciar
    let user1 = User {
        active: true,
        username: String::from("Admin123"),
        email: String::from("admin@email.com"),
        sign_in_count: 1,
    };

    // usar .to_string() se soluciona con impl mas adelante
    // por el tipo de dato &str / String
    // String slice (tam "estatico") Vs String (tam "dinamico")
    let user2 = build_user("email@email.com".to_string(), "User".to_string());

    // extraer valores
    println!("Nombre de User 1 -> {}", user1.username);

    // update structs
    let user3 = User {
        username: String::from("NewUser"),
        ..user2 // traspasa el resto de datos
    };

    println!("Email de User 3 -> {}", user3.email); // mismo el definido de 2

    // uso de structs de tuplas
    let black = Color(0, 0, 0);
    let origin = Punto(0, 0, 0);

    // unit struct
    let subjet = AlwaysEqual;
}

// instanciar de forma corta
// sin repetir email: email por ejemplo
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 2,
    }
}
