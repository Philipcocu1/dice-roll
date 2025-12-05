use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DiceExpression {
    Basic { count: i32, sides: i32, modifier: i32 },
    KeepHighest { count: i32, sides: i32, keep: i32, modifier: i32 },
    KeepLowest { count: i32, sides: i32, keep: i32, modifier: i32 },
    Advantage { sides: i32, modifier: i32 },
    Disadvantage { sides: i32, modifier: i32 },
    Exploding { count: i32, sides: i32, modifier: i32 },
}

impl fmt::Display for DiceExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiceExpression::Basic { count, sides, modifier } => {
                write!(f, "{}d{}{:+}", count, sides, modifier)
            }
            DiceExpression::KeepHighest { count, sides, keep, modifier } => {
                write!(f, "{}d{}k{}{:+}", count, sides, keep, modifier)
            }
            DiceExpression::KeepLowest { count, sides, keep, modifier } => {
                write!(f, "{}d{}kl{}{:+}", count, sides, keep, modifier)
            }
            DiceExpression::Advantage { sides, modifier } => {
                write!(f, "adv(d{}){:+}", sides, modifier)
            }
            DiceExpression::Disadvantage { sides, modifier } => {
                write!(f, "dis(d{}){:+}", sides, modifier)
            }
            DiceExpression::Exploding { count, sides, modifier } => {
                write!(f, "{}d{}!{:+}", count, sides, modifier)
            }
        }
    }
}

pub fn parse(input: &str) -> Result<DiceExpression, String> {
    let input = input.trim().to_lowercase();

    if input.starts_with("adv") {
        return parse_advantage(&input);
    }
    if input.starts_with("dis") {
        return parse_disadvantage(&input);
    }

    let (dice_part, modifier) = extract_modifier(&input);

    if dice_part.contains('!') {
        return parse_exploding(dice_part, modifier);
    }

    if dice_part.contains("kl") {
        return parse_keep_lowest(dice_part, modifier);
    }

    if dice_part.contains('k') {
        return parse_keep_highest(dice_part, modifier);
    }

    parse_basic(dice_part, modifier)
}

fn extract_modifier(input: &str) -> (&str, i32) {
    if let Some(pos) = input.rfind('+') {
        let modifier = input[pos + 1..].parse().unwrap_or(0);
        return (&input[..pos], modifier);
    }
    if let Some(pos) = input.rfind('-') {
        let modifier = input[pos + 1..].parse().unwrap_or(0);
        return (&input[..pos], -modifier);
    }
    (input, 0)
}

fn parse_basic(dice_part: &str, modifier: i32) -> Result<DiceExpression, String> {
    let parts: Vec<&str> = dice_part.split('d').collect();
    if parts.len() != 2 {
        return Err("Invalid dice notation. Use format: XdY".to_string());
    }

    let count = parts[0].parse().map_err(|_| "Invalid dice count")?;
    let sides = parts[1].parse().map_err(|_| "Invalid die size")?;

    if count < 1 || sides < 1 {
        return Err("Dice count and sides must be positive".to_string());
    }

    Ok(DiceExpression::Basic { count, sides, modifier })
}

fn parse_keep_highest(dice_part: &str, modifier: i32) -> Result<DiceExpression, String> {
    let parts: Vec<&str> = dice_part.split('k').collect();
    if parts.len() != 2 {
        return Err("Invalid keep highest notation".to_string());
    }

    let dice_parts: Vec<&str> = parts[0].split('d').collect();
    if dice_parts.len() != 2 {
        return Err("Invalid dice notation".to_string());
    }

    let count = dice_parts[0].parse().map_err(|_| "Invalid dice count")?;
    let sides = dice_parts[1].parse().map_err(|_| "Invalid die size")?;
    let keep = parts[1].parse().map_err(|_| "Invalid keep count")?;

    if count < 1 || sides < 1 || keep < 1 || keep > count {
        return Err("Invalid keep highest parameters".to_string());
    }

    Ok(DiceExpression::KeepHighest { count, sides, keep, modifier })
}

fn parse_keep_lowest(dice_part: &str, modifier: i32) -> Result<DiceExpression, String> {
    let parts: Vec<&str> = dice_part.split("kl").collect();
    if parts.len() != 2 {
        return Err("Invalid keep lowest notation".to_string());
    }

    let dice_parts: Vec<&str> = parts[0].split('d').collect();
    if dice_parts.len() != 2 {
        return Err("Invalid dice notation".to_string());
    }

    let count = dice_parts[0].parse().map_err(|_| "Invalid dice count")?;
    let sides = dice_parts[1].parse().map_err(|_| "Invalid die size")?;
    let keep = parts[1].parse().map_err(|_| "Invalid keep count")?;

    if count < 1 || sides < 1 || keep < 1 || keep > count {
        return Err("Invalid keep lowest parameters".to_string());
    }

    Ok(DiceExpression::KeepLowest { count, sides, keep, modifier })
}

fn parse_advantage(input: &str) -> Result<DiceExpression, String> {
    let input = input.trim_start_matches("adv");
    let (dice_part, modifier) = extract_modifier(input);
    
    let sides = if dice_part.is_empty() {
        20
    } else {
        let trimmed = dice_part.trim_start_matches("(d").trim_end_matches(')');
        trimmed.parse().map_err(|_| "Invalid die size for advantage")?;
    };

    Ok(DiceExpression::Advantage { sides, modifier })
}

fn parse_disadvantage(input: &str) -> Result<DiceExpression, String> {
    let input = input.trim_start_matches("dis");
    let (dice_part, modifier) = extract_modifier(input);
    
    let sides = if dice_part.is_empty() {
        20
    } else {
        let trimmed = dice_part.trim_start_matches("(d").trim_end_matches(')');
        trimmed.parse().map_err(|_| "Invalid die size for disadvantage")?;
    };

    Ok(DiceExpression::Disadvantage { sides, modifier })
}

fn parse_exploding(dice_part: &str, modifier: i32) -> Result<DiceExpression, String> {
    let parts: Vec<&str> = dice_part.split('!').collect();
    if parts.len() != 2 || !parts[1].is_empty() {
        return Err("Invalid exploding dice notation".to_string());
    }

    let dice_parts: Vec<&str> = parts[0].split('d').collect();
    if dice_parts.len() != 2 {
        return Err("Invalid dice notation".to_string());
    }

    let count = dice_parts[0].parse().map_err(|_| "Invalid dice count")?;
    let sides = dice_parts[1].parse().map_err(|_| "Invalid die size")?;

    if count < 1 || sides < 1 {
        return Err("Dice count and sides must be positive".to_string());
    }

    Ok(DiceExpression::Exploding { count, sides, modifier })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_dice() {
        let result = parse("2d6").unwrap();
        assert_eq!(result, DiceExpression::Basic { count: 2, sides: 6, modifier: 0 });
    }

    #[test]
    fn test_parse_with_positive_modifier() {
        let result = parse("2d6+3").unwrap();
        assert_eq!(result, DiceExpression::Basic { count: 2, sides: 6, modifier: 3 });
    }

    #[test]
    fn test_parse_with_negative_modifier() {
        let result = parse("3d8-2").unwrap();
        assert_eq!(result, DiceExpression::Basic { count: 3, sides: 8, modifier: -2 });
    }

    #[test]
    fn test_parse_keep_highest() {
        let result = parse("4d6k3").unwrap();
        assert_eq!(result, DiceExpression::KeepHighest { count: 4, sides: 6, keep: 3, modifier: 0 });
    }

    #[test]
    fn test_parse_keep_lowest() {
        let result = parse("4d6kl1").unwrap();
        assert_eq!(result, DiceExpression::KeepLowest { count: 4, sides: 6, keep: 1, modifier: 0 });
    }

    #[test]
    fn test_parse_advantage() {
        let result = parse("adv").unwrap();
        assert_eq!(result, DiceExpression::Advantage { sides: 20, modifier: 0 });
    }

    #[test]
    fn test_parse_disadvantage() {
        let result = parse("dis").unwrap();
        assert_eq!(result, DiceExpression::Disadvantage { sides: 20, modifier: 0 });
    }

    #[test]
    fn test_parse_exploding_dice() {
        let result = parse("2d6!").unwrap();
        assert_eq!(result, DiceExpression::Exploding { count: 2, sides: 6, modifier: 0 });
    }

    #[test]
    fn test_parse_invalid_format() {
        assert!(parse("invalid").is_err());
        assert!(parse("2x6").is_err());
    }

    #[test]
    fn test_parse_zero_dice() {
        assert!(parse("0d6").is_err());
    }

    #[test]
    fn test_parse_negative_sides() {
        assert!(parse("2d-6").is_err());
    }
}