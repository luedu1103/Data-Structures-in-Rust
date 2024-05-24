
fn main() {
    let cambio = Option::Some(2.24);
    print!("{} = ", &cambio.unwrap());
    algoritmo_voraz(cambio);
}

fn algoritmo_voraz(mut cambio: Option<f64>) {
    let moneda = vec![2.0, 1.0, 0.50, 0.20, 0.12, 0.10, 0.95, 0.02, 0.01];
    let mut indice = 0;

    while cambio.unwrap() != 0.0 {
        match cambio {
            Some(mut x) if x >= moneda[indice] => {
                print!("{}   ", moneda[indice]);
                x = x - moneda[indice];
                cambio = Some(x);
            },
            Some(_) => {},
            None => panic!(),
        }
        if cambio.unwrap() < moneda[indice] {
            indice += 1;
        }
        if indice == moneda.len() { break; }
    }
}
