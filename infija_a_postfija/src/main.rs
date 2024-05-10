use std::collections::HashMap;
use crate::infija_to_postfija::postfija::postfija_and_popall;
use crate::infija_to_postfija::evaluate::evaluating;
use crate::infija_to_prefija::prefija::prefija_and_popall;
pub mod infija_to_postfija;
pub mod infija_to_prefija;

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

    // let string: String = String::from("4*(5+6-(8/2^3)-7)-1");
    // let string: String = String::from("29-9*2^10");
    // let string: String = String::from("29^9*2-10");
    // let string: String = String::from("10+5.2-200");
    // let string: String = String::from("3*11+2004+1+2000+3");
    let string: String = String::from("5*4+((7/2)-3)");

    let postfija_mostrar = postfija_and_popall::postfija(&string, &map);

    println!("{}", &string);
    println!("{:?}", &postfija_mostrar);
    evaluating::evaluate(&postfija_mostrar, &map);

    let string_inv: String = reverse_expression(&string);

    let mut prefija_mostrar = prefija_and_popall::prefija(&string_inv, &map);
    prefija_mostrar.reverse();

    println!("{}", &string_inv);
    println!("{:?}", &prefija_mostrar);
}

fn reverse_expression(expr: &String) -> String {
    let mut result = String::new();
    let mut temp = String::new();

    for c in expr.chars() {
        if c.is_numeric() || c == '.' {
            temp.push(c);
        } else {
            if !temp.is_empty() {
                result.insert_str(0, &temp);
                temp.clear();
            }
            result.insert(0, c);
        }
    }

    if !temp.is_empty() {
        result.insert_str(0, &temp);
    }

    result
}