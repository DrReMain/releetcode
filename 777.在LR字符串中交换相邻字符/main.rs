pub fn can_transform(start: String, end: String) -> bool {
    start.chars().filter(|&c| c == 'X').count() == end.chars().filter(|&c| c == 'X').count()
        && start
            .chars()
            .enumerate()
            .filter(|c| c.1 != 'X')
            .zip(end.chars().enumerate().filter(|c| c.1 != 'X'))
            .all(|(c1, c2)| match (c1.1, c2.1) {
                ('L', 'L') => c1.0 >= c2.0,
                ('R', 'R') => c1.0 <= c2.0,
                _ => false,
            })
}
