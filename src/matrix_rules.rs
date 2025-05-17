pub fn conways_rule(neighbors: [bool; 9]) -> bool {
    let center = neighbors[4];
    let live_neighbors =
        neighbors.iter().filter(|&&cell| cell).count() - if center { 1 } else { 0 };

    match (center, live_neighbors) {
        (true, 2 | 3) => true,
        (true, 0 | 1 | 4..=8) => false,
        (false, 3) => true,
        (false, _) => false,
        _ => false,
    }
}
