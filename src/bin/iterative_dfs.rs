use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut tree = vec![Vec::new(); n + 1];

    for employee in 2..=n {
        let boss: usize = iter.next().unwrap().parse().unwrap();
        tree[boss].push(employee);
    }

    let mut stack = Vec::new();
    let mut order = Vec::new();

    stack.push(1);

    while let Some(node) = stack.pop() {
        order.push(node);

        for &child in &tree[node] {
            stack.push(child);
        }
    }

    let mut size = vec![1usize; n + 1];

    while let Some(node) = order.pop() {
        for &child in &tree[node] {
            size[node] += size[child];
        }
    }

    for i in 1..=n {
        print!("{} ", size[i] - 1);
    }

    println!();
}
