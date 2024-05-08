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

    let mut pila: Vec<String> = Vec::new();
    let mut datos: Vec<char> = Vec::new();
    let mut postfija_mostrar: Vec<String> = Vec::new();
    // let mut operationString: Vec<String> = Vec::new();

    // let string: String = String::from("4*(5+6-(8/2^3)-7)-1");
    // let string: String = String::from("29-9*2^10");
    let string: String = String::from("29^9*2-10");
    // let string: String = String::from("10+5.2-200");
    // let string: String = String::from("3*11+2004+1+2000+3");
    // let mut string_nuevo: String = String::new();
    for i in string.chars() {
        datos.push(i);
    }

    for i in &datos {
        match map.get(&i.to_string()) {
            Some(_value) => {
                postfija_and_popall::postfija(&map, &mut pila, &i,String::from(""), &mut postfija_mostrar);
            },
            None => {
                let ultimo = postfija_mostrar.last().cloned();
                if let Some(mut last_string) = ultimo {
                    if let Some(character) = last_string.chars().next() {
                        if character.is_digit(10) {
                            last_string += &i.to_string();
                            postfija_mostrar.pop();
                            postfija_mostrar.push(last_string);
                            continue;
                        }
                    }
                }
                postfija_mostrar.push(i.to_string());
            },
        }
    }

    // string_nuevo += &postfija_and_popall::pop_everything(&mut pila);
    // println!("{:?}", pila);
    postfija_and_popall::pop_everything(&mut pila, &mut postfija_mostrar);
    postfija_mostrar.retain(|s| !s.is_empty());

    for character in &postfija_mostrar {
        match map.get(&character.to_string()) {
            Some(_value) => {
                evaluating::evaluate_post(&mut pila, &character);
            },
            None => {
                pila.push(character.to_string());
            },
        }
    }

    println!("{}", &string);
    println!("{:?}", &postfija_mostrar);
    println!("{:?}", pila[0]);
}
