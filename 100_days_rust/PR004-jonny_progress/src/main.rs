fn main() {
    println!("JONNY'S PROGRESS");

    let array_miles1 = [3, 4, 1, 2]; // 2
    let array_miles2 = [10, 11, 12, 9, 10]; // 3
    let array_miles3 = [6, 5, 4, 3, 2, 9]; // 1

    println!("{}", progress_days(&array_miles1));
    println!("{}", progress_days(&array_miles2));
    println!("{}", progress_days(&array_miles3));
}

// paso un vistazo al array, por lo que lo recorre segun sus datos
// hecho con while en lugar de for por practicar
fn progress_days(array_miles: &[i32]) -> i32 {
    let mut n1 = 0;
    let mut n2 = 1;

    let mut contador_progreso = 0;

    while n2 < array_miles.len() {
        if array_miles[n2] > array_miles[n1] {
            contador_progreso += 1;
        }
        n2 += 1;
        n1 += 1;
    }
    contador_progreso
}
