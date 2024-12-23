use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let numbers = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let directions = ['A', '^', 'v', '<', '>'];
    let numbers = numbers
        .into_iter()
        .cartesian_product(numbers)
        .sorted()
        .collect_vec();
    let directions = directions
        .into_iter()
        .cartesian_product(directions)
        .sorted()
        .collect_vec();

    for (a, b) in numbers {
        let res = calculate_shortest_path(a, b, Keypad::Numeric);
        let res = res.iter().join("");
        println!("{a},{b},{res}");
    }

    for (a, b) in directions {
        if matches!((a, b), ('<', '>') | ('>', '<') | ('^', 'v') | ('v', '^')) {
            continue;
        }
        let res = calculate_shortest_path(a, b, Keypad::Directional);
        let res = res.iter().join("");
        println!("{a},{b},{res}");
    }
}

#[derive(Debug, Copy, Clone)]
enum Keypad {
    Numeric,
    Directional,
}

fn calculate_shortest_path(start: char, end: char, keypad: Keypad) -> Vec<char> {
    #[rustfmt::skip]
    let numeric: HashMap<char, (i8,i8)> = vec![
        ('7', (0, 0)), ('8', (0, 1)), ('9', (0, 2)),
        ('4', (1, 0)), ('5', (1, 1)), ('6', (1, 2)),
        ('1', (2, 0)), ('2', (2, 1)), ('3', (2, 2)),
                       ('0', (3, 1)), ('A', (3, 2)),
    ].into_iter().collect();
    #[rustfmt::skip]
    let directional: HashMap<char, (i8,i8)> = vec![
                       ('^', (0, 1)), ('A', (0, 2)),
        ('<', (1, 0)), ('v', (1, 1)), ('>', (1, 2)),
    ].into_iter().collect();

    let grid = match keypad {
        Keypad::Numeric => &numeric,
        Keypad::Directional => &directional,
    };

    let mut queue = std::collections::VecDeque::new();
    let mut shortest_paths = Vec::new();
    let mut found_length = None;

    queue.push_back((start, vec![*grid.get(&start).unwrap()]));

    while let Some((current, path)) = queue.pop_front() {
        // If we reach the end, update shortest paths
        if current == end {
            if let Some(min_length) = found_length {
                if path.len() == min_length {
                    shortest_paths.push(path.clone());
                }
            } else {
                found_length = Some(path.len());
                shortest_paths.push(path.clone());
            }
            continue;
        }

        // Process the current node and its neighbors
        if let Some(&cur_coords) = grid.get(&current) {
            for (&neighbor, &neighbor_coords) in grid {
                // Check if `neighbor` is adjacent and not yet visited
                if is_adjacent(cur_coords, neighbor_coords) && !path.contains(&neighbor_coords) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor_coords);
                    queue.push_back((neighbor, new_path));
                }
            }
        }
    }

    // Translate shortest path points into a series of directional changes
    // translates 0,1 0,2 1,2 into v>A
    #[allow(clippy::items_after_statements)]
    fn points_to_directions(path: &[(i8, i8)]) -> Vec<char> {
        let mut path = path
            .windows(2)
            .map(|window| {
                let (a, b) = (window[0], window[1]);
                match (b.0 - a.0, b.1 - a.1) {
                    (1, 0) => 'v',  // Move down
                    (-1, 0) => '^', // Move up
                    (0, 1) => '>',  // Move right
                    (0, -1) => '<', // Move left
                    _ => panic!("Invalid direction between {a:?} and {b:?}"),
                }
            })
            .collect_vec();
        path.push('A');
        path
    }

    // this is assuming both paths have exactly 1 turn
    // The left arrow is furthest away from A, therefore it is the most expensive key
    // it is better to hit this key first
    #[allow(clippy::items_after_statements)]
    fn compare_paths(a: &[char], b: &[char]) -> Ordering {
        // Define the custom order as a slice
        let order = ['<', '^', 'v', '>', 'A'];

        // Iterate through both vectors element by element
        for (a, b) in a.iter().zip(b.iter()) {
            let a_index = order.iter().position(|&x| x == *a).unwrap_or(usize::MAX);
            let b_index = order.iter().position(|&x| x == *b).unwrap_or(usize::MAX);

            match a_index.cmp(&b_index) {
                Ordering::Equal => continue, // If the elements are equal, compare the next
                other => return other,       // Return `Less` or `Greater` immediately
            }
        }

        // If they are equal up to this point, compare by length
        a.len().cmp(&b.len())
    }

    shortest_paths
        .iter()
        .map(|p| points_to_directions(p))
        // it will never be true that the shortest path backtracks
        .filter(|p| count_turns(p) <= 1)
        // get the cheapest path
        .sorted_by(|a, b| compare_paths(a, b))
        // just take the first
        .next()
        .unwrap()
}

// Helper function to determine if two keys are adjacent
fn is_adjacent(a: (i8, i8), b: (i8, i8)) -> bool {
    let row_diff = (a.0 - b.0).abs();
    let col_diff = (a.1 - b.1).abs();
    row_diff + col_diff == 1 // Manhattan distance: adjacent cells
}

/// Counts the number of "turns" in a vector of chars.
/// A turn is defined as a change in direction (`<`, `^`, `v`, `>`).
fn count_turns(path: &[char]) -> usize {
    path.iter()
        .tuple_windows()
        .filter(|&(&a, &b)| a != b && b != 'A')
        .count()
}
