use std::io::{self, Read};

fn dfs(
    node: usize,
    tree: &Vec<Vec<usize>>,
    ans: &mut Vec<usize>,
) -> usize {
    let mut size = 1;

    for &child in &tree[node] {
        size += dfs(child, tree, ans);
    }

    ans[node] = size - 1;

    size
}

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

    let mut ans = vec![0; n + 1];

    dfs(1, &tree, &mut ans);

    for i in 1..=n {
        print!("{} ", ans[i]);
    }

    println!();
}