pub mod prefija_and_popall{
    use std::collections::HashMap;

    pub fn prefija(string: &String, map: &HashMap<String, u32>) -> Vec<String>{
        let mut prefija_mostrar: Vec<String> = Vec::new();
        let mut pila: Vec<String> = Vec::new();
        let mut datos: Vec<char> = Vec::new();
        
        for i in string.chars() {
            datos.push(i);
        }

        for i in &datos {
            match map.get(&i.to_string()) {
                Some(_value) => {
                    prefija_rules(&map, &mut pila, &i,String::from(""), &mut prefija_mostrar);
                },
                None => {
                    let ultimo = prefija_mostrar.last().cloned();
                    if let Some(mut last_string) = ultimo {
                        if let Some(character) = last_string.chars().next() {
                            if character.is_digit(10) {
                                last_string += &i.to_string();
                                prefija_mostrar.pop();
                                prefija_mostrar.push(last_string);
                                continue;
                            }
                        }
                    }
                    prefija_mostrar.push(i.to_string());
                },
            }
        }

        pop_everything(&mut pila, &mut prefija_mostrar);
        prefija_mostrar.retain(|s| !s.is_empty());
        prefija_mostrar
    }

    pub fn prefija_rules(map: &HashMap<String, u32> ,pila: &mut Vec<String>, i: &char, mut operation_string: String, prefija_mostrar: &mut Vec<String>){
        // Si la pila esta vacia, se le coloca el operador
        if pila.is_empty(){
            pila.push(i.to_string());
            // println!("{:?}", &pila);
            prefija_mostrar.push("".to_string());
            return;
        }

        let key = &pila[pila.len()-1]; // Ultima key de la pila
        let valor: u32 = map.get(key).copied().unwrap_or(0); // Obtenemos el valor de la ultima key de la pila
        let value: u32 = map.get(&i.to_string()).copied().unwrap_or(0); // Obtenemos el valor de la llave leida actual

        // Caso con los paréntesis
        if i == &'(' as &char {
            if &valor == &0 { // si es parentesis izquierdo se elimina de la pila
                pila.pop();
            }
            else { // si no es parentesis izquierdo se pasa lo de la pila a la expresion
                // println!("{:?}", &pila);
                pop_everything(pila, prefija_mostrar)
                // operation_string += &pila.pop().unwrap_or("".to_string());
                // return prefija(map, pila, i, operation_string);
            }
            return;
        }

        // se evaluan el valor actual del operador con el valor del ultimo operador de la pila
        if &value > &valor || &value == &0 {
            pila.push(i.to_string());
        }
        else if &value <= &valor {
            operation_string += &pila.pop().unwrap_or("".to_string());
            pila.push(i.to_string());
        }
        // println!("{:?}", &pila);
        prefija_mostrar.push(operation_string);
    }

    pub fn pop_everything(pila: &mut Vec<String>, prefija_mostrar: &mut Vec<String>){
        let valor = pila.pop().unwrap_or("".to_string());
        if valor == "(".to_string() || valor == ")".to_string(){
            return;
        }
        prefija_mostrar.push(valor);
        if !pila.is_empty() {
            return pop_everything(pila, prefija_mostrar);
        }
    }
}