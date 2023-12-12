use helper::IteratorExt;

use crate::Universe;

fn expand(universe: Universe) -> Universe {
    let mut rows = universe
        .rows
        .into_iter()
        .flat_map(|row| {
            if row.iter().all(|b| !b) {
                vec![row.clone(), row]
            } else {
                vec![row]
            }
        })
        .collect_vec();

    let mut col = 0;
    while col < rows[0].len() {
        if rows.iter().all(|row| !row[col]) {
            rows.iter_mut().for_each(|row| row.insert(col, false));
            col += 1;
        }
        col += 1;
    }

    Universe { rows }
}

pub fn part1(input: &str) -> u64 {
    let universe = super::parse(input);
    let universe = expand(universe);

    let galaxies = universe
        .rows
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter(|&(_, b)| b)
                .map(move |(j, _)| (i, j))
        })
        .collect_vec();

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let lhs = galaxies[i];
            let rhs = galaxies[j];

            let distance_x = ((lhs.1 as i64) - (rhs.1 as i64)).abs();
            let distance_y = ((lhs.0 as i64) - (rhs.0 as i64)).abs();

            sum += distance_x + distance_y;
        }
    }

    sum as u64
}
