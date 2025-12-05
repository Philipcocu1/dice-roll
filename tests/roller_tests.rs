use dice_roll::parser::{parse, DiceExpression};
use dice_roll::roller::{roll, calculate_statistics};

#[test]
fn test_basic_roll_range() {
    let expr = parse("2d6").unwrap();
    for _ in 0..100 {
        let result = roll(&expr);
        assert!(result.total >= 2 && result.total <= 12);
        assert_eq!(result.rolls.len(), 2);
    }
}

#[test]
fn test_roll_with_modifier() {
    let expr = parse("1d20+5").unwrap();
    for _ in 0..100 {
        let result = roll(&expr);
        assert!(result.total >= 6 && result.total <= 25);
        assert_eq!(result.modifier, 5);
    }
}

#[test]
fn test_keep_highest() {
    let expr = parse("4d6k3").unwrap();
    for _ in 0..100 {
        let result = roll(&expr);
        assert_eq!(result.rolls.len(), 4);
        assert_eq!(result.kept_indices.len(), 3);
        assert!(result.total >= 3 && result.total <= 18);
    }
}

#[test]
fn test_advantage() {
    let expr = parse("adv").unwrap();
    for _ in 0..100 {
        let result = roll(&expr);
        assert_eq!(result.rolls.len(), 2);
        assert!(result.total >= 1 && result.total <= 20);
        assert!(result.total >= *result.rolls.iter().min().unwrap());
    }
}

#[test]
fn test_disadvantage() {
    let expr = parse("dis").unwrap();
    for _ in 0..100 {
        let result = roll(&expr);
        assert_eq!(result.rolls.len(), 2);
        assert!(result.total >= 1 && result.total <= 20);
        assert!(result.total <= *result.rolls.iter().max().unwrap());
    }
}

#[test]
fn test_statistics_basic() {
    let expr = parse("2d6+3").unwrap();
    let stats = calculate_statistics(&expr);
    assert_eq!(stats.min, 5);
    assert_eq!(stats.max, 15);
    assert!((stats.average - 10.0).abs() < 0.1);
}

#[test]
fn test_statistics_advantage() {
    let expr = parse("adv").unwrap();
    let stats = calculate_statistics(&expr);
    assert_eq!(stats.min, 1);
    assert_eq!(stats.max, 20);
    assert!(stats.average > 10.0);
}

#[test]
fn test_exploding_dice() {
    let expr = parse("2d6!").unwrap();
    for _ in 0..50 {
        let result = roll(&expr);
        assert!(result.rolls.len() >= 2);
        assert!(result.total >= 2);
    }
}
