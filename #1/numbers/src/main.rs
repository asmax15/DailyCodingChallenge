use rand::Rng;

fn main() {
    let list = [10, 15, 3, 7];
    let k = 17;

    loop {
        let rnd = rand::thread_rng().gen_range(0..list.len());
        let rnd2 = rand::thread_rng().gen_range(0..list.len());

        let result = &list[rnd] + &list[rnd2];
        

        if result == k {
            println!("k = {} and result = {} + {}", k, rnd, rnd2);
            break
        }
    }
}
