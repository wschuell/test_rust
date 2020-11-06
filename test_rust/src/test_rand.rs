use rand;
use rand::seq::SliceRandom;

pub fn run() {
	let mut rng = rand::thread_rng();
	for _i in 1..10 {
		const MAX:usize = 100_000;
		let val =  rand::seq::index::sample(&mut rng,MAX,2);
		println!("{:?}", val);
	}

	for _i in 1..10 {

    let vs:Vec<u32> = vec![0, 1, 2, 3, 4];
    let ans: Vec<_>= vs.choose_multiple(&mut rand::thread_rng(),2).collect();
    println!("{:?}", ans[0]);
	}
}
