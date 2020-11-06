use std::time;


mod fib;

// mod test_sql;
mod naminggame;
mod test_rand;


fn main() {
	let start = time::Instant::now();

    // println!("Hello, world!");
    // fib::run();
    // test_sql::run();
    let tconv = naminggame::run(10_000,10_usize.pow(12));
    println!("tconv: {:?}", tconv);
    // test_rand::run();
    println!("Time elapsed: {:?}", start.elapsed() );
}
