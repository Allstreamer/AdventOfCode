use core::panic;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Visited,
}
fn dir_to_move(dir: &Direction) -> (isize, isize) {
    match dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

pub fn solve_task_one(file: &str) -> usize {
    let mut map = vec![];
    let mut player_pos = (0, 0);
    let mut player_dir = Direction::Up;

    for (y, line) in file.lines().enumerate() {
        let mut map_line = vec![];
        for (x, tile) in line.chars().enumerate() {
            map_line.push(match tile {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                '^' => {
                    player_pos = (y, x);
                    Tile::Empty
                }
                _ => {
                    panic!("Invalid Char found!")
                }
            })
        }
        map.push(map_line);
    }

    loop {
        let mov_dir = dir_to_move(&player_dir);
        let new_player_pos = (
            player_pos.0 as isize + mov_dir.0,
            player_pos.1 as isize + mov_dir.1,
        );
        if new_player_pos.0 < 0 || new_player_pos.1 < 0 {
            break;
        }

        let new_player_pos = (new_player_pos.0 as usize, new_player_pos.1 as usize);
        let tile = map.get(new_player_pos.0);
        if tile.is_none() {
            break;
        }
        let tile = tile.unwrap().get(new_player_pos.1);
        if tile.is_none() {
            break;
        }
        let tile = tile.unwrap();
        match tile {
            Tile::Empty | Tile::Visited => {
                map[player_pos.0][player_pos.1] = Tile::Visited;
                player_pos = new_player_pos;
            }
            Tile::Wall => {
                map[player_pos.0][player_pos.1] = Tile::Visited;
                player_dir = match player_dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }
        }
        //print_map(&map);
    }

    map.iter()
        .flatten()
        .filter(|a| **a == Tile::Visited)
        .count()
        + 1
}

fn print_map(map: &Vec<Vec<Tile>>) {
    for l in map {
        for c in l {
            print!(
                "{}",
                match c {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Visited => 'X',
                }
            )
        }
        println!();
    }
    println!();
}

fn validate_loop(
    map: &Vec<Vec<Tile>>,
    player_pos: (usize, usize),
    player_dir: Direction,
) -> Option<&Tile> {
    let mov_dir = dir_to_move(&player_dir);
    let new_player_pos = (
        player_pos.0 as isize + mov_dir.0,
        player_pos.1 as isize + mov_dir.1,
    );
    if new_player_pos.0 < 0 || new_player_pos.1 < 0 {
        return None;
    }

    let new_player_pos = (new_player_pos.0 as usize, new_player_pos.1 as usize);
    let tile = map.get(new_player_pos.0)?;
    let tile = tile.get(new_player_pos.1)?;

    // TODO check for loop
    Some(tile)
}

pub fn solve_task_two(file: &str) -> usize {
    let mut map = vec![];
    let mut player_pos = (0, 0);
    let mut player_dir = Direction::Up;

    for (y, line) in file.lines().enumerate() {
        let mut map_line = vec![];
        for (x, tile) in line.chars().enumerate() {
            map_line.push(match tile {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                '^' => {
                    player_pos = (y, x);
                    Tile::Visited
                }
                _ => {
                    panic!("Invalid Char found!")
                }
            })
        }
        map.push(map_line);
    }

    let mut count = 0;

    loop {
        // Check for obstacle Loop
        if validate_loop(&map, player_pos, player_dir) {
            count += 1;
        }

        // Check Real
        let mov_dir = dir_to_move(&player_dir);
        let new_player_pos = (
            player_pos.0 as isize + mov_dir.0,
            player_pos.1 as isize + mov_dir.1,
        );
        if new_player_pos.0 < 0 || new_player_pos.1 < 0 {
            break;
        }

        let new_player_pos = (new_player_pos.0 as usize, new_player_pos.1 as usize);
        let tile = map.get(new_player_pos.0);
        if tile.is_none() {
            break;
        }
        let tile = tile.unwrap().get(new_player_pos.1);
        if tile.is_none() {
            break;
        }
        let tile = tile.unwrap();
        match tile {
            Tile::Empty => {
                map[player_pos.0][player_pos.1] = Tile::Visited;
                player_pos = new_player_pos;
            }
            Tile::Visited => {
                map[player_pos.0][player_pos.1] = Tile::Visited;
                player_pos = new_player_pos;
            }
            Tile::Wall => {
                map[player_pos.0][player_pos.1] = Tile::Visited;
                player_dir = match player_dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }
        }
        //print_map(&map);
    }

    count
}
