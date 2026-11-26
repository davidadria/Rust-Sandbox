fn main() {
    println!("MERGE SORTED ARRAY");
    /* Notas del ejercicio
     * 1. Numero de elementos inicializados en cada array = m / n
     * 2. nums1 debe tener espacio suficiente para todo
     */

    let mut nums1 = [1, 2, 3, 0, 0, 0]; // m = 3
    let nums2 = [2, 5, 6]; // n = 3

    let mut cont = 0;
    let mut pos = 3;
    println!("INICIAL -> {:?}", nums1);

    loop {
        let n = nums2.get(cont);

        match n {
            Some(&n) => {
                if nums1[pos] == 0 {
                    nums1[pos] = n;
                }
            }
            None => {}
        }

        if cont == nums2.len() {
            break;
        }
        cont += 1;
        pos += 1;
    }

    nums1.sort();
    println!("FIN -> {:?}", nums1);
}
