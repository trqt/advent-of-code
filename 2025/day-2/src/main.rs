use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut res = 0;
    let mut res2 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.map_while(Result::ok) {
            for range in line.split(',') {
                if range.len() < 2 {
                    break;
                }
                let mut it = range.split('-');
                let beg: i64 = it.next().unwrap().parse().unwrap();
                let end: i64 = it.next().unwrap().parse().unwrap();

                for id in beg..=end {
                    let ids = id.to_string();
                    let it = std::iter::once(2).chain((3..=ids.len()).step_by(2)); // 2 and then odds(primes would be better, but alas)

                    for div in it {
                        if ids.len() % div != 0 {
                            continue;
                        }
                        let n = ids.len() / div;
                        let mut chunks = (0..div).map(|i| &ids[i * n..(i + 1) * n]);
                        let first = chunks.next().unwrap();
                        if chunks.all(|x| x == first) {
                            res2 += id;
                            if div == 2 {
                                res += id;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{} {}", res, res2);
}
