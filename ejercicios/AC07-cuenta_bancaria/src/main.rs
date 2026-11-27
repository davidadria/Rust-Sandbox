struct CuentaBancaria {
    titular: String,
    saldo: f64,
}

impl CuentaBancaria {
    fn agregar_titular(titular: &str) -> Self {
        Self {
            titular: titular.trim().to_string(),
            saldo: 0.0,
        }
    }

    fn depositar(&mut self, monto: f64) -> bool {
        println!("Intentando depositar -> {}€", monto);
        if monto <= 0.0 {
            return false;
        }

        self.saldo += monto;
        true
    }

    fn retirar(&mut self, monto: f64) -> bool {
        println!("Intentando retirar -> {}€", monto);
        if monto <= 0.0 || self.saldo < monto {
            return false;
        }

        self.saldo -= monto;

        true
    }

    // mostrar_info
    fn mostrar_info(&self) {
        println!("Titular: {} \t Saldo {}", self.titular, self.saldo);
    }
}

fn main() {
    println!("CUENTA BANCARIA");

    let mut my_cuenta = CuentaBancaria::agregar_titular("Jose Luis");

    my_cuenta.mostrar_info();

    if my_cuenta.depositar(20.0) {
        println!("Dinero depositado");
    } else {
        println!("No se pudo depositar el dinero");
    }

    if my_cuenta.retirar(10.0) {
        println!("Dinero retirado");
    } else {
        println!("No se pudo retirar el dinero");
    }

    if my_cuenta.retirar(50.0) {
        println!("Dinero retirado");
    } else {
        println!("No se pudo retirar el dinero");
    }

    my_cuenta.mostrar_info();
}
