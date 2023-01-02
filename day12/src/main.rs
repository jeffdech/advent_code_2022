mod grid;
use grid::*;

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));
    println!("{:?}", grid);

    println!("==== Part I ====");
    let mut runner = GraphRunner::new(&grid, grid.start());
    let path = runner.find_path();
    println!("The minimum distance is {}", path.unwrap().len());

    // println!("==== Part II ====");
    // println!("There are {} cells at height 0", grid.at_height(0).len());

    // let min_dist = grid.at_height(0).iter()
    //     .enumerate()
    //     .map(|(idx, &coord)| {
    //         println!("Running grid {idx}");
    //         GraphRunner::new(&grid, coord).find_path()
    //     })
    //     .filter_map(|p| if p == None { None } else { Some(p.unwrap().len() - 1) })
    //     .min()
    //     .unwrap();
    // println!("The minimum distance is {}", min_dist);
}
