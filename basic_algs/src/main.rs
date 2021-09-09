fn get_min(arr: &[i32; 10]) -> i32 {
    let mut min = arr[0];
    for i in arr {
        if *i < min {
            min = *i;
        }
    }

    min
}

fn nth_prime(n: i32) -> i32 {
    let mut count = 0;

    for i in 2.. {
        let mut is_prime = true;
        for j in 2.. {
            if j * j > i {
                break;
            }

            if i % j == 0 {
                is_prime = false;
                break;
            }
        }

        if !is_prime {
            continue;
        }

        count += 1;

        if count == n {
            return i;
        }
    }

    panic!("something went wrong");
}

fn bin_search(arr: &[i32; 10], value: i32) -> bool {
    let mut lower_bound = 0;
    let mut upper_bound = arr.len();

    while upper_bound - lower_bound > 1 {
        let mid = (lower_bound + upper_bound) / 2;

        if arr[mid] > value {
            upper_bound = mid;
        } else {
            lower_bound = mid;
        }
    }

    return arr[lower_bound] == value;
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("min of {:?} is {}", arr, get_min(&arr));

    println!("42-th prime is {}", nth_prime(42));

    let arr = [1, 2, 2, 2, 3, 3, 4, 5, 9, 100];
    println!("{:?}", arr);
    println!("Search for 2 is {}", bin_search(&arr, 2));
    println!("Search for 6 is {}", bin_search(&arr, 6));
    println!("Search for 8 is {}", bin_search(&arr, 8));
}
