use rand::prelude::*;
use std::fs;

fn main() {
    let lane_names = ["Top", "Jungle", "Middle", "Bot", "Support"];

    let mut top: Vec<String> = Vec::new();
    let mut jng: Vec<String> = Vec::new();
    let mut mid: Vec<String> = Vec::new();
    let mut bot: Vec<String> = Vec::new();
    let mut sup: Vec<String> = Vec::new();

    let mut selected: Vec<String> = Vec::new();

    let mut rng = rand::thread_rng();

    let file_path = "lolchamps.txt";
    let file = fs::read_to_string(file_path)
        .expect("Unde-i fisieru' , alo?? Tre' sa fie numit \"lolchamps.txt\".");

    let file: Vec<_> = file.split('\n').collect();

    let mut vectors = vec![&mut top, &mut jng, &mut mid, &mut bot, &mut sup];

    for (index, vector) in vectors.iter_mut().enumerate() {
        file.iter()
            .nth(index)
            .expect("Nu a putut separa caracterele")
            .split(' ')
            .skip(1)
            .for_each(|x| vector.push(x.to_string()));
    }

    for (index, vector) in vectors.iter_mut().enumerate() {
        loop {
            let pick = vector.choose(&mut rng).unwrap().to_string();
            if !selected.contains(&pick) {
                selected.push(pick);
                break;
            };
        }
        println!("{}: {}", lane_names[index], selected[index].replace("+", " "));
    }
}
