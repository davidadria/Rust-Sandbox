fn main() {
    funcion_vacia();
    funcion_parametros(5, 'X');

    // declaracion de datos => instrucciones que NO devuelven valores
    let _variable = 20;
    // let var = (let x = 6);   Esto es valido en C o Ruby, no aqui

    // expresion => instrucciones que SI devuelven valores
    let y = {
        let x = 3;
        x + 1 // sin ; se devuelve el valor de x + 1
    };
    println!("The value of y is: {y}");

    let cinco = devolver5();
    println!("{}", cinco);

    let x = sumar_uno(4);
    println!("{}", x);
}

fn funcion_vacia() {
    println!("Nueva funcion");
}

// se requiere poner el tipo en parametros
fn funcion_parametros(x: i32, letra: char) {
    println!("El valor de los parametros son -> {} y {}", x, letra);
}

// para devolver un valor requiere especificarlo
fn devolver5() -> i32 {
    5
}

// recibir un valor y sumarle 1
fn sumar_uno(x: i32) -> i32 {
    x + 1
}
