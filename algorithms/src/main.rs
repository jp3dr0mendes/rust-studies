use rand::Rng;
// use std::io;

// buble sort - OK
// insertion sort - OK
// selection sort - OK
// merge sort
// quick sort - IN PROGRESS
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
//     // let mut array_1: [[usize;1];10] = [[0;1];10];
//     // let mut array_2: [[usize;2];2] = [[0;2];2];
//     // let mut array_3: [[usize;3];2] = [[0;3];2];
//     // let mut array_5: [[usize;5];2] = [[0;5];2];

//     let mut aux: usize = array.len()/2;

//     for i in array.len/2(){
//         fi 
//         i += 1;
//     }    

//     return array;

// }

fn quick_sort(mut array: [usize;10]) -> [usize;10]{

    let mut right: [usize;10] = [0;10];
    let mut left: [usize;10] = [0;10];

    for i in 0..array.len(){

        let mut index: [usize;2] = [0;2];
        let aux: usize = array[i];

        for j in i..array.len(){
            if array[j] >= array[i]{
                
                right[index[0]] = array[j];
                index[0] = index[0] + 1;

            } else if array[j] < array[i]{
                
                left[index[1]] = array[j];
                index[1] = index[1] + 1;
            
            }
        }

        array[left.len()-1] = aux;
        for j in 0..left.len(){
            if left[j] != 0 {
                array[j] = left[j];
            }
        }

        for j in (i..right.len()-1).rev(){
            if right[j] != 0 {
                array[j] = right[right.len()-1-j];
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
