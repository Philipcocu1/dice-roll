use crate::parser::DiceExpression;
use rand::Rng;

#[derive(Debug)]
pub struct RollResult {
    pub rolls: Vec<i32>,
    pub kept_indices: Vec<usize>,
    pub total: i32,
    pub modifier: i32,
    pub die_size: i32,
}

#[derive(Debug)]
pub struct Statistics {
    pub min: i32,
    pub max: i32,
    pub average: f64,
}

pub fn roll(expr: &DiceExpression) -> RollResult {
    let mut rng = rand::thread_rng();

    match expr {
        DiceExpression::Basic { count, sides, modifier } => {
            let rolls = roll_dice(&mut rng, *count, *sides);
            let total: i32 = rolls.iter().sum::<i32>() + modifier;
            RollResult {
                rolls,
                kept_indices: vec![],
                total,
                modifier: *modifier,
                die_size: *sides,
            }
        }
        DiceExpression::KeepHighest { count, sides, keep, modifier } => {
            let rolls = roll_dice(&mut rng, *count, *sides);
            let kept_indices = keep_highest_indices(&rolls, *keep);
            let total: i32 = kept_indices.iter().map(|&i| rolls[i]).sum::<i32>() + modifier;
            RollResult {
                rolls,
                kept_indices,
                total,
                modifier: *modifier,
                die_size: *sides,
            }
        }
        DiceExpression::KeepLowest { count, sides, keep, modifier } => {
            let rolls = roll_dice(&mut rng, *count, *sides);
            let kept_indices = keep_lowest_indices(&rolls, *keep);
            let total: i32 = kept_indices.iter().map(|&i| rolls[i]).sum::<i32>() + modifier;
            RollResult {
                rolls,
                kept_indices,
                total,
                modifier: *modifier,
                die_size: *sides,
            }
        }
        DiceExpression::Advantage { sides, modifier } => {
            let rolls = roll_dice(&mut rng, 2, *sides);
            let total = *rolls.iter().max().unwrap() + modifier;
            RollResult {
                rolls,
                kept_indices: vec![],
                total,
                modifier: *modifier,
                die_size: *sides,
            }
        }
        DiceExpression::Disadvantage { sides, modifier } => {
            let rolls = roll_dice(&mut rng, 2, *sides);
            let total = *rolls.iter().min().unwrap() + modifier;
            RollResult {
                rolls,
                kept_indices: vec![],
                total,
                modifier: *modifier,
                die_size: *sides,
            }
        }
        DiceExpression::Exploding { count, sides, modifier } => {
            let rolls = roll_exploding(&mut rng, *count, *sides);
            let total: i32 = rolls.iter().sum::<i32>() + modifier;
            RollResult {
                rolls,
                kept_indices: vec![],
                total,
                modifier: *modifier,
                die_size: *sides,
            }
        }
    }
}

fn roll_dice<R: Rng>(rng: &mut R, count: i32, sides: i32) -> Vec<i32> {
    (0..count).map(|_| rng.gen_range(1..=sides)).collect()
}

fn roll_exploding<R: Rng>(rng: &mut R, count: i32, sides: i32) -> Vec<i32> {
    let mut results = Vec::new();
    for _ in 0..count {
        let mut roll = rng.gen_range(1..=sides);
        results.push(roll);
        while roll == sides {
            roll = rng.gen_range(1..=sides);
            results.push(roll);
        }
    }
    results
}

fn keep_highest_indices(rolls: &[i32], keep: i32) -> Vec<usize> {
    let mut indexed: Vec<(usize, i32)> = rolls.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    indexed.sort_by(|a, b| b.1.cmp(&a.1));
    indexed.iter().take(keep as usize).map(|(i, _)| *i).collect()
}

fn keep_lowest_indices(rolls: &[i32], keep: i32) -> Vec<usize> {
    let mut indexed: Vec<(usize, i32)> = rolls.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    indexed.sort_by(|a, b| a.1.cmp(&b.1));
    indexed.iter().take(keep as usize).map(|(i, _)| *i).collect()
}

pub fn calculate_statistics(expr: &DiceExpression) -> Statistics {
    match expr {
        DiceExpression::Basic { count, sides, modifier } => {
            let min = *count + modifier;
            let max = count * sides + modifier;
            let average = (*count as f64 * (*sides as f64 + 1.0) / 2.0) + *modifier as f64;
            Statistics { min, max, average }
        }
        DiceExpression::KeepHighest { count, sides, keep, modifier } => {
            let min = *keep + modifier;
            let max = keep * sides + modifier;
            let average = (*keep as f64 * (*sides as f64 + 1.0) / 2.0) + *modifier as f64;
            Statistics { min, max, average }
        }
        DiceExpression::KeepLowest { count, sides, keep, modifier } => {
            let min = *keep + modifier;
            let max = keep * sides + modifier;
            let average = (*keep as f64 * (*sides as f64 + 1.0) / 2.0) + *modifier as f64;
            Statistics { min, max, average }
        }
        DiceExpression::Advantage { sides, modifier } => {
            let min = 1 + modifier;
            let max = sides + modifier;
            let average = (*sides as f64 * 0.65) + *modifier as f64;
            Statistics { min, max, average }
        }
        DiceExpression::Disadvantage { sides, modifier } => {
            let min = 1 + modifier;
            let max = sides + modifier;
            let average = (*sides as f64 * 0.35) + *modifier as f64;
            Statistics { min, max, average }
        }
        DiceExpression::Exploding { count, sides, modifier } => {
            let min = *count + modifier;
            let max = i32::MAX;
            let average = (*count as f64 * (*sides as f64 + 1.0) / 2.0 * 1.2) + *modifier as f64;
            Statistics { min, max, average }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_basic_dice() {
        let expr = DiceExpression::Basic { count: 2, sides: 6, modifier: 0 };
        let result = roll(&expr);
        assert_eq!(result.rolls.len(), 2);
        assert!(result.rolls.iter().all(|&r| r >= 1 && r <= 6));
        assert_eq!(result.modifier, 0);
    }

    #[test]
    fn test_roll_with_modifier() {
        let expr = DiceExpression::Basic { count: 1, sides: 6, modifier: 5 };
        let result = roll(&expr);
        assert!(result.total >= 6 && result.total <= 11);
    }

    #[test]
    fn test_keep_highest_indices() {
        let rolls = vec![3, 1, 5, 2, 4];
        let indices = keep_highest_indices(&rolls, 3);
        assert_eq!(indices.len(), 3);
        let kept_values: Vec<i32> = indices.iter().map(|&i| rolls[i]).collect();
        assert!(kept_values.contains(&5));
        assert!(kept_values.contains(&4));
        assert!(kept_values.contains(&3));
    }

    #[test]
    fn test_keep_lowest_indices() {
        let rolls = vec![3, 1, 5, 2, 4];
        let indices = keep_lowest_indices(&rolls, 2);
        assert_eq!(indices.len(), 2);
        let kept_values: Vec<i32> = indices.iter().map(|&i| rolls[i]).collect();
        assert!(kept_values.contains(&1));
        assert!(kept_values.contains(&2));
    }

    #[test]
    fn test_calculate_statistics_basic() {
        let expr = DiceExpression::Basic { count: 2, sides: 6, modifier: 3 };
        let stats = calculate_statistics(&expr);
        assert_eq!(stats.min, 5);
        assert_eq!(stats.max, 15);
        assert_eq!(stats.average, 10.0);
    }

    #[test]
    fn test_calculate_statistics_advantage() {
        let expr = DiceExpression::Advantage { sides: 20, modifier: 0 };
        let stats = calculate_statistics(&expr);
        assert_eq!(stats.min, 1);
        assert_eq!(stats.max, 20);
        assert!(stats.average > 10.0);
    }

    #[test]
    fn test_roll_advantage_returns_max() {
        let expr = DiceExpression::Advantage { sides: 20, modifier: 0 };
        let result = roll(&expr);
        assert_eq!(result.rolls.len(), 2);
        assert_eq!(result.total, *result.rolls.iter().max().unwrap());
    }

    #[test]
    fn test_roll_disadvantage_returns_min() {
        let expr = DiceExpression::Disadvantage { sides: 20, modifier: 0 };
        let result = roll(&expr);
        assert_eq!(result.rolls.len(), 2);
        assert_eq!(result.total, *result.rolls.iter().min().unwrap());
    }
}