pub mod evaluating{
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

