use std::{cmp::Ordering, collections::HashMap};

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
        let mut meses: HashMap<u32, String> = HashMap::new();
        meses.insert(1, "Enero".to_string());
        meses.insert(2, "Febrero".to_string());
        meses.insert(3, "Marzo".to_string());
        meses.insert(4, "Abril".to_string());
        meses.insert(5, "Mayo".to_string());
        meses.insert(6, "Junio".to_string());
        meses.insert(7, "Julio".to_string());
        meses.insert(8, "Agosto".to_string());
        meses.insert(9, "Septiembre".to_string());
        meses.insert(10, "Octubre".to_string());
        meses.insert(11, "Noviembre".to_string());
        meses.insert(12, "Diciembre".to_string());

        meses.get(&self.mes).unwrap_or(&String::new()).to_string()
    }

    fn compare_to(&self ,other_day: Fecha) -> Option<Ordering>{
        // if self.año < other_day.año {
        //     -1
        // }
        // else if self.año == other_day.año {
        //     if self.mes == other_day.mes {
        //         if self.dia == other_day.dia {return 0;}
        //         else if self.dia < other_day.dia {return -1;}
        //         else {return 1;}
        //     }
        //     else if self.mes < other_day.mes {
        //         return -1;
        //     }
        //     else {
        //         return 1
        //     }
        // }
        // else {
        //     1
        // }
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