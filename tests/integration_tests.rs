use testing_assignment_04::win_conditions::has_won;
#[test]
fn test_has_won() {
    let symbol = 'X';
    let rows = [
        [symbol, symbol, 'O'],
        [symbol, symbol, 'O'],
        ['O', 'O', symbol]
    ];
    let rows2 = [
        ['O', symbol, 'O'],
        [symbol, symbol, 'O'],
        ['O', symbol, symbol]
    ];
    let rows3 = [
        [symbol, 'O', 'O'],
        [symbol, symbol, symbol],
        ['O', symbol, 'O']
    ];
    assert_eq!(has_won(rows, symbol), true);
    assert_eq!(has_won(rows2, symbol), true);
    assert_eq!(has_won(rows3, symbol), true);
}