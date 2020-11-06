use rand;

use rand::seq::SliceRandom;

pub fn run(n:u32,tmax:usize) -> usize {

	println!("Population: {}, tmax: {}",n,tmax );

	let mut t = 0;
	let mut vec_pop: Vec<Vec<u32>> = Vec::new();

	let mut converged = false;
	let mut next_word:u32 = 0;


	for _i in 0..n {
		let new_ag : Vec<u32> = Vec::new();
		vec_pop.push(new_ag);

	}

	let mut rng = rand::thread_rng();

	while !converged && t < tmax {
		// println!("Step {:?}",t );
		step(&mut vec_pop,&mut next_word, &mut rng);
		converged = converged_val(&vec_pop,&t);
		// if t % (vec_pop.len())==0 {println!("{:?}",t );}
		t += 1;
	}

	t


}

fn converged_val(vec_pop:&Vec<Vec<u32>>,t:&usize) -> bool {
	(t % vec_pop.len() == 0) &&
	vec_pop[0].len()==1 && vec_pop.iter().all(|ag| (ag.len()==1 && ag[0]==vec_pop[0][0]))

}

fn converged_val2(vec_pop:&Vec<Vec<u32>>,counter:usize) -> bool {
	counter == vec_pop.len()

}

fn step(vec_pop:&mut Vec<Vec<u32>>,next_word:&mut u32,rng:&mut rand::RngCore) {



	// let mut agents: Vec<_> = vec_pop.choose_multiple(&mut rand::thread_rng(),2).collect();
	assert!(vec_pop.len()>1);
	let agent_id_list = rand::seq::index::sample(rng,vec_pop.len(),2).into_vec();

	let speaker_id = agent_id_list[0] as usize;
	let hearer_id = agent_id_list[1] as usize;


	if vec_pop[speaker_id].len() == 0 {

		vec_pop[speaker_id].push(*next_word);
		// println!("New word {} for speaker {}", next_word,speaker_id);
		*next_word += 1;
	}

	let speaker = &vec_pop[speaker_id];
	let hearer = &vec_pop[hearer_id];

	// let mut speaker = &mut vec_pop[agent_id_list[0]];
	// // let mut hearer: &mut Vec<u32> = &mut agents[1];

	let word = *speaker.choose(&mut rand::thread_rng()).unwrap();
	if hearer.iter().any(|&wh| wh==word) {
		vec_pop[hearer_id] = vec![word];
		vec_pop[speaker_id] = vec![word];
		// println!("Success with word {} between speaker {} and hearer {}", word,speaker_id,hearer_id );
	}
	else {
		vec_pop[hearer_id].push(word);
	}
}
