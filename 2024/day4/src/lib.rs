use std::ops::Not;

pub fn solve_task_one(file: &str) -> usize {
    let map = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // let mut map_clone = map.clone();
    // map_clone.iter_mut().for_each(|e| e.iter_mut().for_each(|b| *b = '.'));
    let mut count = 0;

    for y in 0..map.len() as isize {
        for x in 0..map[0].len() as isize {
            let idx = [(y, x), (y, x + 1), (y, x + 2), (y, x + 3)];
            if check_for_xmas(&map, &idx) {
                count += 1;
                // for pos in idx {
                //     map_clone[pos.0 as usize][pos.1 as usize] = map[pos.0 as usize][pos.1 as usize];
                // }
                // println!("x {}", count);
            }

            let idx = [(y, x), (y + 1, x), (y + 2, x), (y + 3, x)];
            if check_for_xmas(&map, &idx) {
                count += 1;
                // for pos in idx {
                //     map_clone[pos.0 as usize][pos.1 as usize] = map[pos.0 as usize][pos.1 as usize];
                // }
                // println!("y {}", count);
            }

            let idx = [(y, x), (y + 1, x + 1), (y + 2, x + 2), (y + 3, x + 3)];
            if check_for_xmas(&map, &idx) {
                count += 1;
                // for pos in idx {
                //     map_clone[pos.0 as usize][pos.1 as usize] = map[pos.0 as usize][pos.1 as usize];
                // }
                // println!("xy {}", count);
            }

            let idx = [(y, x), (y - 1, x + 1), (y - 2, x + 2), (y - 3, x + 3)];
            if check_for_xmas(&map, &idx) {
                count += 1;
                // for pos in idx {
                //     map_clone[pos.0 as usize][pos.1 as usize] = map[pos.0 as usize][pos.1 as usize];
                // }
                // println!("x-y {}", count);
            }
        }
    }

    // print_map(&map_clone);
    count
}
type Idx = (isize, isize);

// Detect XMAS at idx
pub fn check_for_xmas(map: &[Vec<char>], idx: &[Idx; 4]) -> bool {
    // Check out of bounds
    for i in idx {
        if i.0 < 0 || i.1 < 0 {
            return false;
        }
        if i.0 > map.len() as isize - 1 || i.1 > map[0].len() as isize - 1 {
            return false;
        }
    }

    // Check Fast X & S
    let first_char = map[idx[0].0 as usize][idx[0].1 as usize];
    if (first_char == 'X' || first_char == 'S').not() {
        return false;
    }

    // Check XMAS
    if first_char == 'X' {
        map[idx[1].0 as usize][idx[1].1 as usize] == 'M'
            && map[idx[2].0 as usize][idx[2].1 as usize] == 'A'
            && map[idx[3].0 as usize][idx[3].1 as usize] == 'S'
    } else {
        map[idx[1].0 as usize][idx[1].1 as usize] == 'A'
            && map[idx[2].0 as usize][idx[2].1 as usize] == 'M'
            && map[idx[3].0 as usize][idx[3].1 as usize] == 'X'
    }
}

pub fn solve_task_two(file: &str) -> usize {
    let map = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // let mut map_clone = map.clone();
    // map_clone.iter_mut().for_each(|e| e.iter_mut().for_each(|b| *b = '.'));

    let mut count = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            if map[y][x] != 'A' {
                continue;
            }
            // map_clone[y][x] = 'A';
            if check_for_mas_x(&map, (y, x)) {
                count += 1;
            }
        }
    }
    // print_map(&map_clone);
    count
}

fn check_for_mas_x(map: &[Vec<char>], pos: (usize, usize)) -> bool {
    (map[pos.0 - 1][pos.1 - 1] == 'M' && map[pos.0 - 1][pos.1 + 1] == 'S' && 
     map[pos.0 + 1][pos.1 - 1] == 'M' && map[pos.0 + 1][pos.1 + 1] == 'S') || 
     
    (map[pos.0 - 1][pos.1 - 1] == 'M' && map[pos.0 - 1][pos.1 + 1] == 'M' && 
     map[pos.0 + 1][pos.1 - 1] == 'S' && map[pos.0 + 1][pos.1 + 1] == 'S') || 

    (map[pos.0 - 1][pos.1 - 1] == 'S' && map[pos.0 - 1][pos.1 + 1] == 'S' && 
     map[pos.0 + 1][pos.1 - 1] == 'M' && map[pos.0 + 1][pos.1 + 1] == 'M') || 

    (map[pos.0 - 1][pos.1 - 1] == 'S' && map[pos.0 - 1][pos.1 + 1] == 'M' && 
     map[pos.0 + 1][pos.1 - 1] == 'S' && map[pos.0 + 1][pos.1 + 1] == 'M')
}

#[allow(dead_code)]
fn print_map(map: &[Vec<char>]) {
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Check for Right, Down, Right-Down
        let map = vec![
            "XMAS".chars().collect::<Vec<char>>(),
            "MMSX".chars().collect(),
            "AMAS".chars().collect(),
            "SMAS".chars().collect(),
        ];
        assert!(check_for_xmas(&map, &[(0, 0), (0, 1), (0, 2), (0, 3),]));
        assert!(check_for_xmas(&map, &[(0, 0), (1, 0), (2, 0), (3, 0),]));
        assert!(check_for_xmas(&map, &[(0, 0), (1, 1), (2, 2), (3, 3),]));

        // Check for Reverse
        let map = vec![
            "SAMX".chars().collect::<Vec<char>>(),
            "AASX".chars().collect(),
            "MMMS".chars().collect(),
            "XMAX".chars().collect(),
        ];
        assert!(check_for_xmas(&map, &[(0, 0), (0, 1), (0, 2), (0, 3),]));
        assert!(check_for_xmas(&map, &[(0, 0), (1, 0), (2, 0), (3, 0),]));
        assert!(check_for_xmas(&map, &[(0, 0), (1, 1), (2, 2), (3, 3),]));

        // Check for Left, Top, Left-Up
        let map = vec![
            "SAMS".chars().collect::<Vec<char>>(),
            "AASA".chars().collect(),
            "MMMM".chars().collect(),
            "SAMX".chars().collect(),
        ];
        assert!(check_for_xmas(&map, &[(3, 3), (3, 2), (3, 1), (3, 0)]));
        assert!(check_for_xmas(&map, &[(3, 3), (2, 3), (1, 3), (0, 3)]));
        assert!(check_for_xmas(&map, &[(3, 3), (2, 2), (1, 1), (0, 0)]));

        // Out of bounds Check
        let map = vec![
            "XMASX".chars().collect::<Vec<char>>(),
            "MMSXX".chars().collect(),
            "AMASX".chars().collect(),
            "SMASX".chars().collect(),
        ];
        // Negative
        assert!(!check_for_xmas(&map, &[(-1, 0), (0, 1), (0, 2), (0, 3),]));
        assert!(!check_for_xmas(&map, &[(0, 0), (-1, 1), (0, 2), (0, 3),]));
        assert!(!check_for_xmas(&map, &[(0, 0), (1, 1), (0, -2), (0, 3),]));
        assert!(!check_for_xmas(&map, &[(0, 0), (1, 1), (0, 2), (0, -3),]));
        // Positive
        assert!(!check_for_xmas(&map, &[(4, 0), (0, 1), (0, 2), (0, 3),]));
        assert!(!check_for_xmas(&map, &[(3, 5), (0, 1), (0, 2), (0, 3),]));
        assert!(!check_for_xmas(&map, &[(5, 0), (0, 1), (0, 2), (0, 3),]));
        assert!(!check_for_xmas(&map, &[(3, 6), (0, 1), (0, 2), (0, 3),]));
        assert!(!check_for_xmas(&map, &[(0, 0), (0, 1), (0, 2), (7, 3),]));
    }
}
