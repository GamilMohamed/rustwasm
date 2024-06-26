use crate::random::random_range;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
/*



*/

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}


pub type Position = (i32, i32);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

#[derive(Debug)]
pub struct SnakeGame {
  pub width: i32,
  pub height: i32,
  pub snake: VecDeque<Position>, // Head is the first item, tail is the last item
  pub direction: Direction,
  next_direction: Direction,
  pub food: Position,
  pub finished: bool,
  pub score: i32,
}

impl SnakeGame {
  pub fn new(width: i32, height: i32) -> Self {
    Self {
      width,
      height,
      snake: [((width - 3).max(0), height / 2)].into_iter().collect(),
      direction: Direction::Left,
      next_direction: Direction::Left,
      food: (2.min(width - 1), height / 2),
      finished: false,
      score: 0,
    }
  }

  pub fn change_direction(&mut self, direction: Direction) {
    if self.finished {
      return;
    }

    match (self.direction, direction) {
      (Direction::Up, Direction::Up)
      | (Direction::Up, Direction::Down)
      | (Direction::Right, Direction::Right)
      | (Direction::Right, Direction::Left)
      | (Direction::Down, Direction::Up)
      | (Direction::Down, Direction::Down)
      | (Direction::Left, Direction::Right)
      | (Direction::Left, Direction::Left) => {}
      (_, direction) => self.next_direction = direction,
    }
  }

  pub fn is_valid(&self, (x, y): Position) -> bool {
    x < self.width && y < self.height && x >= 0 && y >= 0
  }

  pub fn tick(&mut self) {
    if self.finished && self.snake.len() == 0 {
      return;
    }
    // log(&format!("test {:?}", self.snake));
    // println!("test {:?}", self.snake);
    self.direction = self.next_direction;

    let (x, y) = self.snake[0];
    // WARNING: There's no explicit underflow handling here
    // (will panic in debug build)
    let mut new_head = match self.direction {
      Direction::Up => (x, y - 1),
      Direction::Right => (x + 1, y),
      Direction::Down => (x, y + 1),
      Direction::Left => (x - 1, y),
    };
    if !self.is_valid(new_head)
    {
      log(&format!("new_head {:?}", new_head));
      if new_head.0 >= self.width {
        new_head.0 = 0;
        log(&format!("new_head.0 >= self.width {}  {}",new_head.0,self.width));
      }
      if new_head.1 >= self.height {
        new_head.1 = 0;
        log(&format!("new_head.1 >= self.height {}  {}",new_head.1,self.height));
      }
      if new_head.0 < 0 {
        new_head.0 = self.width - 1;
        log(&format!("new_head.0 < 0 {}  {}",new_head.0,self.width));
      }
      if new_head.1 < 0 {
        new_head.1 = self.height - 1;
        log(&format!("new_head.1 < 0 {}  {}",new_head.1,self.height));
      }
    }
    if self.snake.contains(&new_head) {
      // Lose conditions
      self.finished = true;
    } else {
      if new_head != self.food {
        // Do not pop tail when eating food to make snake longer
        self.snake.pop_back();
      } else {
        let free_positions = (0..self.height)
          .flat_map(|y| (0..self.width).map(move |x| (x, y)))
          .filter(|pos| !self.snake.contains(pos))
          .collect::<Vec<_>>();

        if free_positions.is_empty() {
          self.finished = true;
          return;
        }

        self.food = free_positions[random_range(0, free_positions.len())];
        self.score += 1;
      }

      self.snake.push_front(new_head);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::SnakeGame;

  #[test]
  fn test() {
    println!("{:?}", SnakeGame::new(15, 15));
  }
}
