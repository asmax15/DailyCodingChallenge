use rand::Rng;

fn main() {
    let mut list = [10, 15, 3, 7];
    let k = 17;

    loop {
        let mut rnd = rand::thread_rng().gen_range(0..list.len());
        let mut rnd2 = rand::thread_rng().gen_range(0..list.len());

        let mut result = &list[rnd] + &list[rnd2];
        

        if result == k {
            println!("k = {} and result = {} + {}", k, rnd, rnd2);
            break
        }
    }
}
