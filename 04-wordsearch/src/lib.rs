#[derive(Clone, Copy)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub fn get_offsets(direction: Directions) -> (i32, i32) {
    match direction {
        Directions::Up => (-1, 0),
        Directions::Down => (1, 0),
        Directions::Left => (0, -1),
        Directions::Right => (0, 1),
        Directions::UpLeft => (-1, -1),
        Directions::UpRight => (-1, 1),
        Directions::DownLeft => (1, -1),
        Directions::DownRight => (1, 1),
    }
}

pub fn get_word_in_direction(
    graph: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    direction: Directions,
    length: usize,
) -> Option<String> {
    let (dx, dy) = get_offsets(direction);
    let mut x = x as i32;
    let mut y = y as i32;
    let mut word = String::new();
    for _ in 0..length {
        if x < 0 || x >= graph.len() as i32 || y < 0 || y >= graph[0].len() as i32 {
            return None;
        }
        let c = graph[x as usize][y as usize];
        if c == ' ' {
            return None;
        }
        word.push(c);
        x += dx;
        y += dy;
    }
    Some(word)
}
