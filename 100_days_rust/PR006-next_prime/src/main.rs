fn main() {
    println!("NEXT PRIME");
    println!("{}", next_prime(12));
}

// averiguar si un num es primo
fn is_prime(num: i32) -> bool {
    // si n es menor o igual a 1 no puede ser primo
    if num <= 1 {
        return false;
    }
    // ver si es primo
    for a in 2..num {
        if num % a == 0 {
            return false;
        }
    }
    // si no es divisible, es primo
    true
}

fn next_prime(num: i32) -> i32 {
    // si recibe un primo se devuelve a si mismo
    if is_prime(num) {
        return num;
    }

    // parar al encontrar el siguiente num primo
    let mut flag = false;
    let mut final_number: i32 = 0;
    while !flag {
        let new_num = num + 1;
        if is_prime(new_num) {
            flag = true;
            final_number = new_num;
        }
    }

    return final_number;
}

/* SOLUCION OPTIMA
fn next_prime(num: i32) -> i32 {
    if is_prime(num) {
        return num;
    }

    // numero actual incrementado
    let mut current = num + 1;

    // comprobar con el numero actual si es primo
    while !is_prime(current) {
        // si no lo es se incrementa
        current += 1;
    }

    // al encontrar un primo sale del bucle y lo devuelve
    current
}
*/
