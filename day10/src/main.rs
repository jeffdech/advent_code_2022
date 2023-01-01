mod cpu;
use cpu::*;

fn main() {
    let instructions = parser::parse_input(include_str!("input.txt")).unwrap().1;
    
    let break_points = vec![20, 60, 100, 140, 180, 220];
    let total: i64 = break_points.iter()
        .map(|&n| CpuState::run_to(&instructions, n).signal_strength())
        .sum();

    println!("Total is {}", total);

    let screen = Screen::new(&instructions);
    screen.render();
}
