// estructura que va a implementar
struct Rectangle {
    width: u32,
    height: u32,
}

// bloque impl, para poner metodos a estructuras
// se pueden tener varios bloques por struct
impl Rectangle {
    // &self para no recibir ownership del struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // se pueden añadir varios métodos por bloque
    fn width(&self) -> bool {
        self.width > 0
    }

    fn cabe(&self, otro: &Rectangle) -> bool {
        self.width > otro.width && self.height > otro.height
    }

    // devolver un struct de tipo Rectangle, algo asi como un constructor
    fn cuadrado(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangulo1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rectangulo2 = Rectangle {
        width: 60,
        height: 90,
    };

    println!("El area del rectangulo es: {}", rectangulo1.area());

    if rectangulo1.width() {
        println!("El ancho del rectangulo es > 0: {}", rectangulo1.width);
    }

    println!(
        "Puede el rectangulo1 contener al 2?: {}",
        rectangulo1.cabe(&rectangulo2)
    );
    println!(
        "Puede el rectangulo2 contener al 1?: {}",
        rectangulo2.cabe(&rectangulo1)
    );

    // crear uns instancia de cuadrado, que tenga los metodos de rectangulo
    let cuadrado = Rectangle::cuadrado(10);
    println!("Area del cuadrado: {}", cuadrado.area());
}
