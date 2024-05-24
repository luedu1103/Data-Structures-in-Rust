fn main() {
    let mut array = vec![1,2,7,8,100,20,50];
    let long = array.len();

    println!("{:?}", &mut array);

    heapsort(&mut array, long);

    println!("{}", array[0]);
}

fn heapsort(array: &mut Vec<usize>, n: usize){
	for i in (0..n/2-1).rev() {
		heapify(array, i, n);
	}

	// for i in (0..n).rev() {
	// 	array.swap(i, 0);
	// 	heapify(array, 0, i);
	// }
}

fn heapify(array: &mut Vec<usize>, i: usize, n: usize){
	let mut largest: usize = i;
	let l: usize = 2*i+1;
	let r: usize = 2*i+2;

	if l < n && array[l] > array[largest]{
		largest = l;
	}
	if r < n && array[r] > array[largest]{
		largest = r;
	}

	if largest != i {
		array.swap(i, largest as usize);
		// println!("{:?}", array);
		return heapify(array, largest, n);
	}
}

