use rand::Rng;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Snake {
    pub body: Vec<Point>,
    pub angle: f32,
    pub speed: f32,
    pub radius: f32,
}

impl Snake {
    pub fn new(x: f32, y: f32) -> Self {
        let mut body = Vec::new();
        for i in 0..10 {
            body.push(Point { x: x - i as f32 * 10.0, y });
        }
        Self {
            body,
            angle: 0.0,
            speed: 3.5,
            radius: 10.0,
        }
    }

    pub fn update(&mut self, target_angle: f32) {
        // Smoothly rotate towards target angle
        let mut diff = target_angle - self.angle;
        while diff > std::f32::consts::PI { diff -= std::f32::consts::TAU; }
        while diff < -std::f32::consts::PI { diff += std::f32::consts::TAU; }
        self.angle += diff * 0.15;

        // Move head
        let mut new_head = self.body[0];
        new_head.x += self.angle.cos() * self.speed;
        new_head.y += self.angle.sin() * self.speed;

        // Update body parts (follow like a chain)
        let mut prev = new_head;
        let segment_dist = 8.0;

        let mut new_body = vec![new_head];
        for i in 1..self.body.len() {
            let mut curr = self.body[i];
            let dx = prev.x - curr.x;
            let dy = prev.y - curr.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > segment_dist {
                let ratio = segment_dist / dist;
                curr.x = prev.x - dx * ratio;
                curr.y = prev.y - dy * ratio;
            }
            new_body.push(curr);
            prev = curr;
        }
        self.body = new_body;
    }
}

pub struct Game {
    pub snake: Snake,
    pub food: Point,
    pub score: u32,
    pub high_score: u32,
    pub is_over: bool,
    pub width: f32,
    pub height: f32,
}

impl Game {
    pub fn new(width: f32, height: f32) -> Self {
        let high_score = fs::read_to_string("highscore.txt")
            .unwrap_or_else(|_| "0".to_string())
            .trim()
            .parse()
            .unwrap_or(0);

        let mut rng = rand::thread_rng();
        Self {
            snake: Snake::new(width / 2.0, height / 2.0),
            food: Point {
                x: rng.gen_range(50.0..width - 50.0),
                y: rng.gen_range(50.0..height - 50.0),
            },
            score: 0,
            high_score,
            is_over: false,
            width,
            height,
        }
    }

    pub fn update(&mut self, target_angle: f32) {
        if self.is_over { return; }

        self.snake.update(target_angle);

        let head = self.snake.body[0];

        // Wall collision
        if head.x < 0.0 || head.x > self.width || head.y < 0.0 || head.y > self.height {
            self.is_over = true;
        }

        // Self collision (skip some segments near head)
        for i in 10..self.snake.body.len() {
            let part = self.snake.body[i];
            let dx = head.x - part.x;
            let dy = head.y - part.y;
            if (dx * dx + dy * dy).sqrt() < self.snake.radius + 2.0 {
                self.is_over = true;
                break;
            }
        }

        // Food collision
        let dx = head.x - self.food.x;
        let dy = head.y - self.food.y;
        if (dx * dx + dy * dy).sqrt() < self.snake.radius + 10.0 {
            self.score += 10;
            if self.score > self.high_score {
                self.high_score = self.score;
                let _ = fs::write("highscore.txt", self.high_score.to_string());
            }
            
            // Grow snake
            let last = *self.snake.body.last().unwrap();
            for _ in 0..5 {
                self.snake.body.push(last);
            }

            let mut rng = rand::thread_rng();
            self.food = Point {
                x: rng.gen_range(50.0..self.width - 50.0),
                y: rng.gen_range(50.0..self.height - 50.0),
            };
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.width, self.height);
    }
}
