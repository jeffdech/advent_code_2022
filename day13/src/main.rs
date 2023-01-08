mod packet;
use packet::*;

fn main() {
    // let mut sum = 0;
    // for (i, groups) in include_str!("sample-input.txt").split("\n\n").enumerate() {
    //     let i = i + 1;

    //     let mut nodes = groups
    //         .lines()
    //         .map(|line| serde_json::from_str::<Node>(line).unwrap());

    //     let l = nodes.next().unwrap();
    //     let r = nodes.next().unwrap();
    //     println!("\n== Pair {i} ==");
    //     println!("l = {l:?}");
    //     println!("r = {r:?}");
    //     println!("l < r = {}", l < r);
    //     if l < r {
    //         sum += i;
    //     }
    // }
    // dbg!(sum);

    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec!(Node::Number(6)))
    ];

    let mut packets = include_str!("input.txt")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| serde_json::from_str::<Node>(line).unwrap())
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();

    packets.sort();

    let decoder_key = dividers
        .iter()
        .map(|p| packets.binary_search(p).unwrap() + 1)
        .product::<usize>();

    dbg!(decoder_key);
}
