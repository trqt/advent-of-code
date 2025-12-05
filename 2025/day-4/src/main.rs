use std::{fs::read_to_string, io};

static DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbors(mat: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<char> {
    let n = mat.len();
    let m = mat[i].len();
    let mut out = Vec::new();

    for (dx, dy) in DIRS {
        let ni = i as isize + dx;
        let nj = j as isize + dy;

        if ni >= 0 && ni < n as isize && nj >= 0 && nj < m as isize {
            out.push(mat[ni as usize][nj as usize]);
        }
    }

    out
}

fn read_matrix_from_file(path: &str) -> io::Result<Vec<Vec<char>>> {
    let content = read_to_string(path)?;
    let mut mat = Vec::new();

    for line in content.lines() {
        mat.push(line.chars().collect());
    }

    Ok(mat)
}

fn main() {
    let mut res = 0;
    let mut res2 = 0;

    let mat = read_matrix_from_file("input.txt").unwrap();
    let mut mat2 = mat.clone();

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == '.' {
                continue;
            }
            if neighbors(&mat, i, j).iter().filter(|x| **x == '@').count() < 4 {
                res += 1
            };
        }
    }

    let mut any_removed = true;
    while any_removed {
        any_removed = false;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat2[i][j] == '.' {
                    continue;
                }
                if neighbors(&mat2, i, j).iter().filter(|x| **x == '@').count() < 4 {
                    mat2[i][j] = '.';
                    res2 += 1;
                    any_removed = true;
                };
            }
        }
    }
    println!("{} {}", res, res2);
}
