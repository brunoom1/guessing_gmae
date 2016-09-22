extern crate rand;

use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    
	println!("Adivinhe o numero!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("Por favor, coloque seu palpite.");

	// println!("O numero secreto e {:?}", secret_number);

	loop {
		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Falha para ler a linha");

		let guess: u32 = guess.trim().parse().expect("Por favor, o tipo deve ser um numero");

		println!("Seu palpite: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less 		=> println!("Mais alto!"),
			Ordering::Greater	=> println!("Mais baixo!"),
			Ordering::Equal		=> {
				println!("You win!");
				break;
			},
		}
	}

}
