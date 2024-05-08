pub mod evaluating{
    use std::collections::HashMap;


    pub fn evaluate(postfija_mostrar: &Vec<String>, map: &HashMap<String, u32>){
        let mut pila: Vec<String> = Vec::new();

        for character in postfija_mostrar {
            match map.get(&character.to_string()) {
                Some(_value) => {
                    evaluate_post(&mut pila, &character);
                },
                None => {
                    pila.push(character.to_string());
                },
            }
        }

        println!("{}", pila[0]);
    }

    pub fn evaluate_post(pila: &mut Vec<String>, string: &String){
        let valor_de = pila.pop().unwrap_or("0".to_string());
        let valor_iz = pila.pop().unwrap_or("0".to_string());

        let a: f32 = valor_de.parse().unwrap();
        let b: f32 = valor_iz.parse().unwrap();

        if string == &"+" {
            pila.push((b+a).to_string());
        }
        else if string == &"-" {
            pila.push((b-a).to_string());
        }
        else if string == &"*" {
            pila.push((b*a).to_string());
        }
        else if string == &"/" {
            pila.push((b/a).to_string());
        }
        else if string == &"^" {
            pila.push((b.powf(a)).to_string());
        }
    }
}

