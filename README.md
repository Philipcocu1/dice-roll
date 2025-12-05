# dice-roll

CLI dice roller supporting RPG notation (2d6+3, 4d20k3) with statistics and probability analysis

## Features

- Parse standard dice notation (XdY, XdY+Z, XdY-Z)
- Support keep highest/lowest (4d6k3, 4d6kl1)
- Support advantage/disadvantage rolls (adv, dis)
- Support exploding dice (2d6!)
- Display individual die results with total
- Calculate and display probability distributions for simple rolls
- Show min/max/average for roll expressions
- Support multiple rolls with --count flag
- Colorized output for critical successes/failures
- Roll history tracking in current session
- Verbose mode showing step-by-step calculation

## Installation

```bash
# Clone the repository
git clone https://github.com/KurtWeston/dice-roll.git
cd dice-roll

# Install dependencies
cargo build
```

## Usage

```bash
cargo run
```

## Built With

- rust

## Dependencies

- `clap`
- `rand`
- `colored`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
