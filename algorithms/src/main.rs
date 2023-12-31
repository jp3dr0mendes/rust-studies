use rand::Rng;
use std::io;

// buble sort - OK
// insertion sort
// selection sort
// merge sort
// quick sort
// binary search - OK
// linear search - OK
// busca ternaria
// busca exponencial
// busca interpolada

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
        }
    }

    return array;
}


fn binary_search(array: [usize;10], number: usize) -> (bool, usize){

    let mut on_array: bool = false;
    let mut pivo: usize = (array.len()-1)/2;

    if array[pivo] == number{
        on_array = true;
        return (on_array, pivo);
    } else {
        while pivo != 0 && pivo != array.len()-1 {

            if number > array[pivo]{
                pivo = ((array.len()) + pivo) / 2; //para garantir que chegue ao final do array
            } else {
                pivo = pivo / 2; 
            }

            if array[pivo] == number {
                on_array = true;
                return (on_array, pivo);                
            }
        }
        return (on_array, pivo);
    }
}

fn linear_search(array: [usize;10], number: usize) -> (bool, usize) {
    for i in 0..array.len()-1{
        if array[i] == number{
            return (true, i);
        }
    }
    return (false, 0);
}

fn main() {
        let list_size: usize = 10;
        let mut array:  [usize;10] = [0;10];
        let mut num: String = String::new();
    
        for i in 0..list_size {
            array[i] = rand::thread_rng().gen_range(0..=100);
        }
    
        println!("O array resultante eh {:?}", array);
    
        let order_array: [usize;10] = buble_sort(array);

        println!("Order array: {:?}", order_array);
        println!("Type a number you'd like to search on array:");

        io::stdin().read_line(&mut num).expect("Error on reading element!");

        let num:usize = num.trim().parse().expect("Please type a number!");
        let result: (bool, usize) = linear_search(order_array, num);
        // let result: (bool, usize) = binary_search(order_array, num);

        // println!("result: {}", result.0);

        if result.0 {
            println!("The number {num} is on index {}", result.1);
        } else {
            println!("Error on find the number on array!");
        }
    }
