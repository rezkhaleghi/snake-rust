# Terminal Snake Game

A classic Snake game implementation in Rust using the Crossterm library for terminal manipulation.

![Project Demo](/demo.png)

## Overview

This is a simple, terminal-based Snake game where you control a snake that grows longer as it eats food. The game ends when the snake hits the wall or itself.

## Requirements

- Rust and Cargo (https://www.rust-lang.org/tools/install)
- Dependencies:
  - crossterm (for terminal manipulation)
  - rand (for random food placement)

## Installation

1. Clone this repository or create a new Rust project
2. Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
crossterm = "0.26.1"
rand = "0.8.5"
```

3. Copy the source code into `src/main.rs`
4. Build and run the game with `cargo run`

## How to Play

- Use arrow keys (↑, ↓, ←, →) to navigate the snake
- Eat food (●) to increase your score and make the snake grow
- Avoid hitting walls and the snake's own body
- Press 'q' to quit the game

## Controls

- ↑ (Up Arrow): Change direction to upward
- ↓ (Down Arrow): Change direction to downward
- ← (Left Arrow): Change direction to left
- → (Right Arrow): Change direction to right
- q: Quit the game

## Game Elements

- ■: Snake head
- □: Snake body
- ●: Food
- █: Walls/Border

## Author

Created and maintained by "PocketJack (Rez Khaleghi)"

- GitHub: https://github.com/rezkhaleghi
- Email: rezaxkhaleghi@gmail.com

## Support

If you enjoy this app and would like to support my work:

- Patreon : https://patreon.com/PocketJack
  Your support helps me continue developing free and open-source stuff.

## License

This project is available under the MIT License.

## Acknowledgments

- Built with Rust and Crossterm
- Inspired by the classic Snake game on nokia

## Contributing

Contributions are welcome! Feel free to submit a Pull Request.
