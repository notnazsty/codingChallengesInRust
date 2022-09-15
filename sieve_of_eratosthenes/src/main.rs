/**
 * Directions
 * To find all the prime numbers less than or equal to a given integer n by Eratosthenes' method:
 * Create a list of consecutive integers from 2 through n: (2, 3, 4, ..., n).
 * Initially, let p equal 2, the smallest prime number.
 * Enumerate the multiples of p by counting in increments of p from 2p to n, and mark them in the list (these will be 2p, 3p, 4p, ...; the p itself should not be marked).
 * Find the smallest number in the list greater than p that is not marked. If there was no such number, stop. Otherwise, let p now equal this new number (which is the next prime), and repeat from step 3.
 * When the algorithm terminates, the numbers remaining not marked in the list are all the primes below n.
 */

 // TODO in future make faster

fn remove_matches(prime_num_vec: &Vec<u32>, matches_arr: Vec<u32>) -> Vec<u32> {
    let mut removed_matches_arr: Vec<u32> = vec![];

    for el in prime_num_vec.iter() {
        if !matches_arr.contains(el) {
            removed_matches_arr.push(*el)
        }
    }
    return removed_matches_arr;
}

fn get_smallest_num_greater_than(list: &Vec<u32>, greater_than_num: u32) -> Option<u32> {
    for el in list.iter() {
        if *el > greater_than_num {
            return Some(*el);
        }
    }
    return None;
}

fn all_multiples_until_n(muliple_of: u32, n: u32) -> Vec<u32> {
    let mut multiple_arr: Vec<u32> = vec![];

    let mut current_multiple: u32 = 2 * muliple_of;

    loop {
        if current_multiple > n {
            break;
        }
        multiple_arr.push(current_multiple);
        current_multiple = current_multiple + muliple_of;
    }

    return multiple_arr;
}

fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {

    let mut prime_num_vec: Vec<u32> = (2..n).collect();
    let mut p: Option<u32> = Some(prime_num_vec[0]);
    loop {
        if p == None {
            break;
        }

        let multiples_of_p_arr: Vec<u32> = all_multiples_until_n(p.unwrap(), n);
        prime_num_vec = remove_matches(&prime_num_vec, multiples_of_p_arr);
        p = get_smallest_num_greater_than(&prime_num_vec, p.unwrap());
    }

    return prime_num_vec;
}

fn main() {
    println!("{:?}", sieve_of_eratosthenes(10000));
}
