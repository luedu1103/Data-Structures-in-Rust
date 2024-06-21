fn main() {
    let numeros = vec![2, 3, 6, 1, 5];
    let valor = 9;
    let mut solutions = Vec::new();
    let mut posible_solution = Vec::new();
    
    backtracking(valor, &numeros, &mut posible_solution, &mut solutions, 0);

    println!("{:?}", solutions);
}

fn backtracking(valor: i32, numeros: &Vec<i32>, posible_solution: &mut Vec<i32>, solutions: &mut Vec<Vec<i32>>, start: usize) {
    let suma: i32 = posible_solution.iter().sum();
    println!("{:?}", posible_solution);
    
    if suma == valor {
        solutions.push(posible_solution.clone());
        return;
    }
    
    if suma > valor {
        return;
    }
    
    for i in start..numeros.len() {
        posible_solution.push(numeros[i]);
        backtracking(valor, numeros, posible_solution, solutions, i + 1);
        posible_solution.pop();
    }
}