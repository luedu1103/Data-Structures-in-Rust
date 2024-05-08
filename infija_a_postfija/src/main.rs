use std::collections::HashMap;
use crate::infija_to_postfija::postfija::postfija_and_popall;
use crate::infija_to_postfija::evaluate::evaluating;
pub mod infija_to_postfija;

fn main() {
    // Hashmap (No se me ocurría otra forma de hacerlo)
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("+".to_string(), 1);
    map.insert("-".to_string(), 1);
    map.insert("*".to_string(), 2);
    map.insert("/".to_string(), 2);
    map.insert("^".to_string(), 3);
    map.insert("(".to_string(), 0);
    map.insert(")".to_string(), 0);

    let string: String = String::from("4*(5+6-(8/2^3)-7)-1");
    // let string: String = String::from("29-9*2^10");
    // let string: String = String::from("29^9*2-10");
    // let string: String = String::from("10+5.2-200");
    // let string: String = String::from("3*11+2004+1+2000+3");

    let postfija_mostrar = postfija_and_popall::postfija(&string, &map);

    println!("{}", &string);
    println!("{:?}", &postfija_mostrar);
    evaluating::evaluate(&postfija_mostrar, &map);
}
