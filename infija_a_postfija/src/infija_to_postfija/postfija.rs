pub mod postfija_and_popall{
    use std::collections::HashMap;

    pub fn postfija(string: &String, map: &HashMap<String, u32>) -> Vec<String>{
        let mut postfija_mostrar: Vec<String> = Vec::new();
        let mut pila: Vec<String> = Vec::new();
        let mut datos: Vec<char> = Vec::new();
        
        for i in string.chars() {
            datos.push(i);
        }

        for i in &datos {
            match map.get(&i.to_string()) {
                Some(_value) => {
                    postfija_rules(&map, &mut pila, &i,String::from(""), &mut postfija_mostrar);
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

        pop_everything(&mut pila, &mut postfija_mostrar);
        postfija_mostrar.retain(|s| !s.is_empty());
        postfija_mostrar
    }

    pub fn postfija_rules(map: &HashMap<String, u32> ,pila: &mut Vec<String>, i: &char, mut operation_string: String, postfija_mostrar: &mut Vec<String>){
        // Si la pila esta vacia, se le coloca el operador
        if pila.is_empty(){
            pila.push(i.to_string());
            postfija_mostrar.push("".to_string());
            return;
        }

        let key = &pila[pila.len()-1]; // Ultima key de la pila
        let valor: u32 = map.get(key).copied().unwrap_or(0); // Obtenemos el valor de la ultima key de la pila
        let value: u32 = map.get(&i.to_string()).copied().unwrap_or(0); // Obtenemos el valor de la llave leida actual

        // Caso con los paréntesis
        if i == &')' as &char {
            if &valor == &0 { // si es parentesis izquierdo se elimina de la pila
                pila.pop();
            }
            else { // si no es parentesis izquierdo se pasa lo de la pila a la expresion
                pop_everything(pila, postfija_mostrar)
                // operation_string += &pila.pop().unwrap_or("".to_string());
                // return postfija(map, pila, i, operation_string);
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
        postfija_mostrar.push(operation_string);
    }

    pub fn pop_everything(pila: &mut Vec<String>, postfija_mostrar: &mut Vec<String>){
        let valor = pila.pop().unwrap_or("".to_string());
        if valor == "(".to_string() || valor == ")".to_string(){
            return;
        }
        postfija_mostrar.push(valor);
        if !pila.is_empty() {
            return pop_everything(pila, postfija_mostrar);
        }
    }
}