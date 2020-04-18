pub mod backend;
mod board_iter;
mod square;

#[cfg(test)]
mod tests {
    use ::backend::Game;
    use ::board_iter::board_iter;

    #[test]
    fn it_works() {
        let mut g = Game::new(9, 9, 20);
        Game::guess(&mut g, 3, 2);
        println!("{:}", g);
    }

    #[test]
    fn test_board_iter() {
        let b = board_iter(1, 1, 3, 3);
        let expected: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        for (i, pair) in b.enumerate() {
            assert_eq!(expected[i], pair);
        }
    }
}
