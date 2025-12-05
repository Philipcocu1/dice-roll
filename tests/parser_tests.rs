use dice_roll::parser::{parse, DiceExpression};

#[test]
fn test_basic_notation() {
    let result = parse("2d6");
    assert!(result.is_ok());
    match result.unwrap() {
        DiceExpression::Basic { count, sides, modifier } => {
            assert_eq!(count, 2);
            assert_eq!(sides, 6);
            assert_eq!(modifier, 0);
        }
        _ => panic!("Wrong expression type"),
    }
}

#[test]
fn test_with_positive_modifier() {
    let result = parse("1d20+5");
    assert!(result.is_ok());
    match result.unwrap() {
        DiceExpression::Basic { count, sides, modifier } => {
            assert_eq!(count, 1);
            assert_eq!(sides, 20);
            assert_eq!(modifier, 5);
        }
        _ => panic!("Wrong expression type"),
    }
}

#[test]
fn test_with_negative_modifier() {
    let result = parse("3d8-2");
    assert!(result.is_ok());
    match result.unwrap() {
        DiceExpression::Basic { count, sides, modifier } => {
            assert_eq!(count, 3);
            assert_eq!(sides, 8);
            assert_eq!(modifier, -2);
        }
        _ => panic!("Wrong expression type"),
    }
}

#[test]
fn test_keep_highest() {
    let result = parse("4d6k3");
    assert!(result.is_ok());
    match result.unwrap() {
        DiceExpression::KeepHighest { count, sides, keep, modifier } => {
            assert_eq!(count, 4);
            assert_eq!(sides, 6);
            assert_eq!(keep, 3);
            assert_eq!(modifier, 0);
        }
        _ => panic!("Wrong expression type"),
    }
}

#[test]
fn test_keep_lowest() {
    let result = parse("4d6kl1");
    assert!(result.is_ok());
    match result.unwrap() {
        DiceExpression::KeepLowest { count, sides, keep, modifier } => {
            assert_eq!(count, 4);
            assert_eq!(sides, 6);
            assert_eq!(keep, 1);
        }
        _ => panic!("Wrong expression type"),
    }
}

#[test]
fn test_advantage() {
    let result = parse("adv");
    assert!(result.is_ok());
    match result.unwrap() {
        DiceExpression::Advantage { sides, modifier } => {
            assert_eq!(sides, 20);
            assert_eq!(modifier, 0);
        }
        _ => panic!("Wrong expression type"),
    }
}

#[test]
fn test_disadvantage() {
    let result = parse("dis+2");
    assert!(result.is_ok());
    match result.unwrap() {
        DiceExpression::Disadvantage { sides, modifier } => {
            assert_eq!(sides, 20);
            assert_eq!(modifier, 2);
        }
        _ => panic!("Wrong expression type"),
    }
}

#[test]
fn test_exploding() {
    let result = parse("2d6!");
    assert!(result.is_ok());
    match result.unwrap() {
        DiceExpression::Exploding { count, sides, modifier } => {
            assert_eq!(count, 2);
            assert_eq!(sides, 6);
            assert_eq!(modifier, 0);
        }
        _ => panic!("Wrong expression type"),
    }
}

#[test]
fn test_invalid_notation() {
    assert!(parse("invalid").is_err());
    assert!(parse("d20").is_err());
    assert!(parse("2d").is_err());
}
