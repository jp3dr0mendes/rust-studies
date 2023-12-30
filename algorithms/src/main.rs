use rand::Rng;

//buble sort - OK
//insertion sort
//selection sort
//merge sort
//quick sort
//binary search
//linear search

fn buble_sort(mut array: [usize;10]) -> [usize;10] {
    let mut aux: [usize;2] = [0;2];

    for i in 0..array.len()-1{
        for j in 0..array.len()-1-i{ //uma vez que ao final de cada loop i o maior vai ficar no final

            println!("{j}");
            if array[j] > array[j+1]{

                aux[0] = array[j];
                aux[1] = array[j+1];

                array[j+1] = aux[0];
                array[j] = aux[1];
            }

            println!("array on iteration{i}: {:?}", array);
        }
    }

    return array;
}


fn main() {
    let list_size: usize = 10;
    let mut array: [usize;10] = [0;10];

    for i in 0..list_size {
        array[i] = rand::thread_rng().gen_range(0..=100);
    }

    println!("O array resultante eh {:?}", array);

    let order_array: [usize;10] = buble_sort(array);
    println!("Order array: {:?}", order_array);
}
