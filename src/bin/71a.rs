#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
#[allow(dead_code)]
fn yn(result: bool) {
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    let n = read::<usize>();
    for _i in 0..n {
        let s = read::<String>().chars().collect::<Vec<char>>();
        if s.len() > 10 {
            let head = s[0];
            let last = s.last().unwrap();
            let ans = format!("{}{}{}", head, s.len() - 2, last);
            println!("{}", ans);
        } else {
            println!("{}", s.iter().collect::<String>());
        }
    }
}
