fn swap_arr(arr: &mut [i32], i: usize, j: usize) {
    let pom = arr[i];
    arr[i] = arr[j];
    arr[j] = pom;
}

fn change_seed(seed: &mut i128) {
    let a = 22695477;
    *seed = (a * *seed + 1) % 2_i128.pow(31);
}

fn rand(seed: &mut i128, min_rand: i128, max_rand: i128) -> i128 {
    change_seed(seed);
    println!("seed: {seed}");
    *seed % (max_rand - min_rand + 1) + min_rand
}

fn rand_perm(arr: &mut [i32], seed: &mut i128){
    let i = rand(seed, 0,arr.len() as i128) as usize;
    let j = rand(seed, 0,arr.len() as i128) as usize;
    println!("i: {i}, j: {j}");
    swap_arr(arr, i, j);
}


fn main() {
    let mut seed:i128 = 8141;
    let mut tab = [0,1,2,3,4,5,6,7,8,9];
    println!("{:?}",tab);
    rand_perm(&mut tab, &mut seed);
    println!("{:?}",tab);
    // for i in 0..10 {
    //     print!("{} ", tab[i]);
    // }
}
