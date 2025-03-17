use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor,
};
use std::io::{stdout, Write};
use std::{thread, time::Duration};
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: Vec<Point>,
    dir: Direction,
    food: Point,
    width: i32,
    height: i32,
    game_over: bool,
    score: usize,
}

impl Game {
    fn new(width: i32, height: i32) -> Game {
        let head = Point { x: width / 2, y: height / 2 };
        Game {
            snake: vec![head],
            dir: Direction::Right,
            food: Game::spawn_food(width, height),
            width,
            height,
            game_over: false,
            score: 0,
        }
    }

    fn spawn_food(width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(1..width - 1),
            y: rng.gen_range(1..height - 1),
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let head = self.snake[0];
        let new_head = match self.dir {
            Direction::Up => Point { x: head.x, y: head.y - 1 },
            Direction::Down => Point { x: head.x, y: head.y + 1 },
            Direction::Left => Point { x: head.x - 1, y: head.y },
            Direction::Right => Point { x: head.x + 1, y: head.y },
        };

        if new_head.x < 1 || new_head.x >= self.width - 1 || 
           new_head.y < 1 || new_head.y >= self.height - 1 || 
           self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);
        if new_head == self.food {
            self.score += 10;
            self.food = Game::spawn_food(self.width, self.height);
        } else {
            self.snake.pop();
        }
    }

    fn render(&self) {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        // Draw top border with score
        for x in 0..self.width {
            execute!(stdout, cursor::MoveTo(x as u16, 0)).unwrap();
            execute!(stdout, Print("█")).unwrap();
        }
        
        // Display score at the top
        let score_text = format!(" Score: {} ", self.score);
        let score_x = 2;
        execute!(stdout, cursor::MoveTo(score_x, 0), Print(score_text)).unwrap();

        // Draw the rest of the game
        for y in 1..self.height {
            for x in 0..self.width {
                let p = Point { x, y };
                execute!(stdout, cursor::MoveTo(x as u16, y as u16)).unwrap();
                if x == 0 || x == self.width - 1 || y == self.height - 1 {
                    execute!(stdout, Print("█")).unwrap();
                } else if self.snake[0] == p {
                    execute!(stdout, Print("■")).unwrap();
                } else if self.snake.contains(&p) {
                    execute!(stdout, Print("□")).unwrap();
                } else if self.food == p {
                    execute!(stdout, Print("●")).unwrap();
                } else {
                    execute!(stdout, Print(" ")).unwrap();
                }
            }
        }

        if self.game_over {
            let msg = format!("Game Over! Final Score: {}. Press Q to quit.", self.score);
            let x_pos = if self.width as usize > msg.len() {
                (self.width as u16 / 2).saturating_sub(msg.len() as u16 / 2)
            } else {
                0
            };
            execute!(stdout, cursor::MoveTo(x_pos, self.height as u16 / 2), Print(msg)).unwrap();
        }
        stdout.flush().unwrap();
    }

    fn change_direction(&mut self, dir: Direction) {
        match (&self.dir, &dir) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => return,
            _ => self.dir = dir,
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide).unwrap();

    let mut game = Game::new(30, 15); 

    loop {
        game.render();
        if game.game_over {
            if let Ok(Event::Key(key)) = event::read() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        if event::poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up => game.change_direction(Direction::Up),
                    KeyCode::Down => game.change_direction(Direction::Down),
                    KeyCode::Left => game.change_direction(Direction::Left),
                    KeyCode::Right => game.change_direction(Direction::Right),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        game.update();
        thread::sleep(Duration::from_millis(150)); 
    }

    execute!(stdout, LeaveAlternateScreen, cursor::Show).unwrap();
    disable_raw_mode().unwrap();
}