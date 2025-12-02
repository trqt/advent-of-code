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
    let mut dial = 50;
    let mut res = 0;
    let mut res2 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.map_while(Result::ok) {
            let dir = if line.chars().nth(0).unwrap() == 'L' {
                -1
            } else {
                1
            };

            let line = line.replace("L", "");
            let line = line.replace("R", "");
            let rot: i32 = line.parse().unwrap();

            if dir == -1 {
                res2 += (100 - dial + rot) / 100 + if dial == 0 { -1 } else { 0 }
            } else {
                res2 += (dial + rot) / 100;
            }

            dial = (dial + (dir * rot)).rem_euclid(100);

            if dial == 0 {
                res += 1;
            }
            // dumb way
            // for _ in 0..rot {
            //     acc += dir;
            //     acc = acc.rem_euclid(100);

            //     if acc == 0 {
            //         res2 += 1;

            //         println!("passed on 0: {}", line);
            //     }
            // }
        }
    }

    println!("{} {}", res, res2);
}
