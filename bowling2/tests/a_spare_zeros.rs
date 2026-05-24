
use bowling::*;
#[test]


fn a_spare_followed_by_zeros_is_worth_ten_points() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
}