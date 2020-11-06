use std::io;
use std::time;

pub fn run() {
	println!("Fibonacci module");
		let mut input = String::new();
	println!("Fibonacci sequence element number:");
		io::stdin().read_line(&mut input).expect("error: unable to read user input");

	let input:usize = input.trim().parse().expect("Error with string conversion, input a number");
	println!("Written: {}",input);

	let start = time::Instant::now();

	let result :usize;

	if input < 3 { result=1; }
	else {

		let mut f: [usize;2] = [1,1];
		let mut counter:usize = 2;

		let mut current_index:usize = counter % 2;

		while counter < input {
			current_index = counter % 2;
			f[current_index] = f[0] + f[1];
			counter += 1;

		}

		result = f[current_index];
	}

	let elapsed = start.elapsed();
	println!("{:?}",result );
	println!("{:?}",elapsed );

}

