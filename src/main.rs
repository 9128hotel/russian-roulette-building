use std::io::{stdin, stdout, Write};
use rand::Rng;
fn russian_roulette(barrel_count:u32) {
    loop {
        let mut barrels:Vec<String>=vec![];
        for _x in 0..barrel_count {
            barrels.push("O".to_string());
        }
        let mut rng = rand::thread_rng();
        let bullet_location= rng.gen_range(0..barrel_count) as usize;
        
        for x in 0..barrels.len() {
            if x==bullet_location {
                barrels[x]="0".to_string();
            }
            print!("{}", barrels[x]);
        }
        println!();
        let mut fire:Vec<String>=vec![];
        let fire_location= rng.gen_range(0..barrel_count) as usize;
        for _x in 0..barrel_count {
            fire.push(" ".to_string());
        }
        for x in 0..fire.len() {
            if x==fire_location {
                fire[x]="^".to_string();
            }
            print!("{}", fire[x]);
        }
        println!();
        if bullet_location==fire_location {
            println!("Dead")
        }
        else {
            println!("Alive")
        }
        println!();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }
}
fn main() {
    print!("Number of barrels: ");
    #[allow(unused_must_use)]{stdout().flush();};
    let mut input_text = String::new();
    stdin().read_line(&mut input_text).expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => russian_roulette(i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    
}
