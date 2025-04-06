fn swap(tab: &mut [u32], a: usize, b: usize){
    let pom = tab[a];
    tab[a] = tab[b];
    tab[b] = pom;
}

fn swap_range(tab: &mut [u32], r1: (u32,u32), r2: (u32, u32)) {
    let len1 = r1.1 - r1.0 + 1;
    let len2 = r2.1 - r2.0 + 1;
    let len: usize = if len1 > len2 {len2 as usize} else {len1 as usize};

    for k in 0..len {
        swap(tab, r1.0 as usize + k, r2.0 as usize + k);
    }
}


fn main() {
    let mut tab: [u32;8] = [1,2,3,4,5,6,7,8];
    swap_range(&mut tab, (2,5), (5,7));
    for i in 0..tab.len() {
        print!("{} ",tab[i]);
    }
}
