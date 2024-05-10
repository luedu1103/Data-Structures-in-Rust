use std::{cmp::Ordering, vec};

fn main() {
    println!("Hola!!!");
    let hoy = Fecha { 
        dia: 3,
        mes: 11,
        año: 2004
     };
    let other_day = Fecha {
        dia: 3,
        mes: 11,
        año: 2004
     };
    println!("{:#?}", hoy.get_ano());
    println!("{:#?}", hoy.get_dia());
    println!("{:#?}", hoy.get_mes());
    println!("{:#?}", hoy.month_name());
    println!("{:#?}", hoy.compare_to(other_day));
}

#[derive(Debug)]
struct Fecha {
    dia: u32,
    mes: u32,
    año: u32
}

impl Fecha {
    fn get_dia(&self) -> u32 {
        self.dia
    }

    fn get_mes(&self) -> u32 {
        self.mes
    }

    fn get_ano(&self) -> u32 {
        self.año
    }

    fn month_name(&self) -> String {
        let meses: Vec<String> = vec![
            "Enero".to_string(), "Febrero".to_string(), "Marzo".to_string(), "Abril".to_string(), "Mayo".to_string(), "Junio".to_string(),
            "Julio".to_string(), "Agosto".to_string(), "Septiembre".to_string(), "Octubre".to_string(), "Noviembre".to_string(), "Diciembre".to_string(), 
        ];

        let string = &meses[self.mes as usize - 1 as usize];
        string.to_string()
    }

    fn compare_to(&self ,other_day: Fecha) -> Option<Ordering>{
        match self.año.cmp(&other_day.año) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => {
                match self.mes.cmp(&other_day.mes) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                    Ordering::Equal => {
                        Some(self.dia.cmp(&other_day.dia))
                    }
                }
            }
        }
    }
}