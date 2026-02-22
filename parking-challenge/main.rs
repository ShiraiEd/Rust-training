use std::io;

enum Tipo {
    Employee,
    Customer,
    Vip,
}

struct Car {
    plate: String,
    hours: u16,
    tipo: Tipo,
}

fn calculate_price(car: &Car) {
    let result = match car.tipo {
        Tipo::Employee => String::from("Free"),

        Tipo::Vip => {
            let valor = car.hours * 5;
            if car.hours > 8 {
                (valor + 20).to_string()
            } else {
                valor.to_string()
            }
        }
        Tipo::Customer => {
            let valor = car.hours * 10;
            if car.hours > 8 {
                (valor + 20).to_string()
            } else {
                valor.to_string()
            }
        }
    };
    println!("car plate:{}\nPrice:{result}", car.plate.trim());
}

fn main() {
    let mut plate = String::new();

    println!("Numero do carro");

    io::stdin()
        .read_line(&mut plate)
        .expect("Plate input error");

    let mut horas = String::new();

    println!("horas estacionadas");

    io::stdin()
        .read_line(&mut horas)
        .expect("Plate input error");

    let horas: u16 = horas.trim().parse().expect("horas nao validas");

    let mut tipo = String::new();

    println!("Tipo do cliente");

    io::stdin().read_line(&mut tipo).expect("Plate input error");

    println!();

    let tipo: Tipo = match tipo.trim().to_lowercase().as_str() {
        "employee" => Tipo::Employee,
        "customer" => Tipo::Customer,
        "vip" => Tipo::Vip,
        _ => {
            println!("tipo invalido");
            Tipo::Customer
        }
    };

    let carro = Car {
        plate: plate,
        hours: horas,
        tipo: tipo,
    };

    calculate_price(&carro);
}
