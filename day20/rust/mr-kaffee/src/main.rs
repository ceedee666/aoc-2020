use mr_kaffee_2020_20::*;
use std::time::Instant;
use std::fs;

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read from file.")
}

fn main() {
    // start timer
    let instant_main = Instant::now();

    // read file
    let content = read_input();

    // parse
    let tiles = Tile::parse(&content);

    // solve part 1
    let instant_part = Instant::now();
    let (width, solution) = solve(&tiles);
    let sol = corners_checksum(width, &solution);
    println!("Solution part 1 done in {:?}: {}", instant_part.elapsed(), sol);
    assert!(sol < 97_037_306_166_193);

    // solve part 2
    let instant_part = Instant::now();
    let picture = get_picture(width, &solution);
    let (monsters, _) = find_monsters(&picture);
    let sol = get_roughness(&picture, monsters.len());
    println!("Solution part 2 done in {:?}: {}", instant_part.elapsed(), sol);
    assert_eq!(sol, 2_084);

    // print elapsed time
    println!("Total time: {:?}", instant_main.elapsed());
}
