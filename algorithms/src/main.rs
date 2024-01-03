use rand::Rng;
// use std::io;

// buble sort - OK
// insertion sort - OK
// selection sort - OK
// merge sort
// quick sort s
// binary search - OK
// linear search - OK
// busca ternaria
// busca exponencial
// busca interpolada

fn validation_sort(array: [usize;10]) -> bool{
    for i in 1..array.len(){
        if array[i] < array[i-1]{
            return false;
        }
    } 
    return true;
} //validando se o array realmente está ordenado

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

fn insertion_sort(mut array: [usize;10]) -> [usize;10]{
    let mut aux_array: [usize;2] = [0;2];
    
    for i in 1..array.len(){

        if i-1 == 0{
            if array[i] < array[i-1]{

                aux_array = [array[i], array[i-1]];
                array[i-1] = aux_array[0];
                array[i] = aux_array[1];

            }
        } else {

            for j in (1..i+1).rev(){ //garantindo que pegue até o ultimo elemento do array, visto que para antes do ultimo 
                //garantindo que pegue ate o primeiro elemento
                if array[j] < array[j-1]{

                    aux_array = [array[j], array[j-1]];
                    array[j-1] = aux_array[0];
                    array[j] = aux_array[1];

                }
            }
        }
    }

    return array;
}

fn selection_sort(mut array: [usize;10]) -> [usize;10]{
    let mut aux: [usize;2] = [0;2];

    for i in 0..array.len(){

        let mut min: usize = array[i];
        let mut index: usize = 0;

        aux = [array[i],array[i]];

        for j in i..array.len(){

            if array[j] < min{
                min = array[j];
                aux[1] = min;
                index = j;
                println!("aux: {:?}", aux);
            }
        }

        if index != 0 {
            array[i] = aux[1];
            array[index] = aux[0];
        }
    }
    return array;
}

// fn merge_sort(mut array: [usize;10]) -> [usize;10]{
// }

fn quick_sort(mut array: [usize;10]) -> [usize;10]{
    let mut result: [usize;10] = [0;10];

    for i in (0..array.len()).rev(){
        let mut left: [usize;10] = [0;10];
        let mut right: [usize;10] = [0;10];

        let mut aux: [usize;2] = [0, 10]; //[aux left; aux right]
        let mut index: [usize;2] = [0;2]; //[index left; index right]

        println!("on iteration {i}, element: {}", array[i]);

        for j in 0..i{
            if array[j] < array[i]{
                left[index[0]] = array[j];
                index[0] += 1;
                aux[0] += 1;
            } else {
                right[index[1]] = array[j];
                index[1] += 1;
                aux[1] -= 1;
            }

            println!("right: {:?}; left: {:?}", right, left);
        }

        array[aux[0]] = array[i];

        for j in 0..aux[0]{
            array[j] = left[j];
        }

        let mut count: usize = 0;

        for j in aux[1]..array.len(){
            array[j] = right[count];
            count += 1;
        }

        println!("result: {:?}",array);
        
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
        // let mut num: String = String::new();
    
        for i in 0..list_size {
            array[i] = rand::thread_rng().gen_range(0..=100);
        }
    
        println!("O array resultante eh {:?}", array);
    
        let order_array: [usize;10] = quick_sort(array);

        println!("Order array: {:?}", order_array);
        
        // println!("Type a number you'd like to search on array:");

        // io::stdin().read_line(&mut num).expect("Error on reading element!");

        // let num:usize = num.trim().parse().expect("Please type a number!");
        // let result: (bool, usize) = linear_search(order_array, num);
        // let result: (bool, usize) = binary_search(order_array, num);

        // println!("result: {}", result.0);

        // if result.0 {
        //     println!("The number {num} is on index {}", result.1);
        // } else {
        //     println!("Error on find the number on array!");
        // }
    }
