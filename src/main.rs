mod engine;
mod game;

use engine::{Engine, WIDTH, HEIGHT};
use game::Game;
use minifb::Key;
use std::time::Instant;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, Source};

const COLOR_BG: u32 = 0x0F0F0F;
const COLOR_GRID: u32 = 0x1A1A1A;

#[derive(PartialEq, Clone, Copy)]
enum GameState {
    Menu,
    Settings,
    Playing,
    Paused,
    GameOver,
}

fn main() {
    let mut engine = Engine::new("Rust Snake - Ultra Edition", WIDTH, HEIGHT);
    let mut game = Game::new(WIDTH as f32, HEIGHT as f32);
    let mut state = GameState::Menu;
    let mut previous_state = GameState::Menu;
    
    // UI State
    let mut menu_selected = 0;
    let mut music_enabled = true;

    // Audio Setup
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut music_sink = Sink::try_new(&stream_handle).unwrap();
    music_sink.set_volume(0.4);
    
    fn play_music(sink: &mut Sink) {
        if let Ok(file) = File::open("assets/music.mp3") {
            let source = Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();
            sink.append(source);
            sink.play();
        }
    }
    play_music(&mut music_sink);

    let mut key_cooldown = Instant::now();

    while engine.is_open() {
        let now = Instant::now();
        let can_press = now.duration_since(key_cooldown).as_millis() > 150;

        match state {
            GameState::Menu => {
                engine.clear(COLOR_BG);
                draw_background_grid(&mut engine);
                draw_header(&mut engine, "RUST SNAKE");
                
                let items = ["START GAME", "SETTINGS", "QUIT"];
                draw_menu(&mut engine, &items, menu_selected);

                if can_press {
                    if engine.is_key_down(Key::Up) {
                        menu_selected = (menu_selected + items.len() - 1) % items.len();
                        key_cooldown = now;
                    } else if engine.is_key_down(Key::Down) {
                        menu_selected = (menu_selected + 1) % items.len();
                        key_cooldown = now;
                    } else if engine.is_key_down(Key::Enter) || engine.is_key_down(Key::Space) {
                        match menu_selected {
                            0 => { game.reset(); state = GameState::Playing; }
                            1 => { previous_state = GameState::Menu; menu_selected = 0; state = GameState::Settings; }
                            2 => return,
                            _ => {}
                        }
                        key_cooldown = now;
                    }
                }
                engine.draw_vignette();
            }
            GameState::Settings => {
                engine.clear(COLOR_BG);
                draw_background_grid(&mut engine);
                draw_header(&mut engine, "SETTINGS");
                
                let music_text = if music_enabled { "MUSIC: ON" } else { "MUSIC: OFF" };
                let items = [music_text, "BACK"];
                draw_menu(&mut engine, &items, menu_selected);

                if can_press {
                    if engine.is_key_down(Key::Up) || engine.is_key_down(Key::Down) {
                        menu_selected = 1 - menu_selected;
                        key_cooldown = now;
                    } else if engine.is_key_down(Key::Enter) || engine.is_key_down(Key::Space) {
                        if menu_selected == 0 {
                            music_enabled = !music_enabled;
                            if music_enabled {
                                music_sink = Sink::try_new(&stream_handle).unwrap();
                                music_sink.set_volume(0.4);
                                play_music(&mut music_sink);
                            } else {
                                music_sink.stop();
                            }
                        } else {
                            state = previous_state;
                            menu_selected = 0;
                        }
                        key_cooldown = now;
                    }
                }
                engine.draw_vignette();
            }
            GameState::Playing => {
                if engine.is_key_down(Key::Escape) && can_press {
                    state = GameState::Paused;
                    menu_selected = 0;
                    key_cooldown = now;
                }

                // Mouse steering (with dead zone)
                let target_angle = if let Some((mx, my)) = engine.get_mouse_pos() {
                    let head = game.snake.body[0];
                    let dx = mx - head.x;
                    let dy = my - head.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    
                    if dist > 15.0 { // Dead zone
                        dy.atan2(dx)
                    } else {
                        game.snake.angle
                    }
                } else {
                    game.snake.angle
                };

                let old_score = game.score;
                game.update(target_angle);
                
                if game.is_over {
                    state = GameState::GameOver;
                    menu_selected = 0;
                }

                if game.score > old_score {
                    if let Ok(file) = File::open("assets/eat.wav") {
                        let source = Decoder::new(BufReader::new(file)).unwrap();
                        stream_handle.play_raw(source.convert_samples()).unwrap();
                    }
                }

                engine.clear(COLOR_BG);
                draw_background_grid(&mut engine);
                
                // Draw Food (with glow effect)
                let fx = game.food.x as usize;
                let fy = game.food.y as usize;
                engine.draw_rect(fx - 6, fy - 6, 12, 12, 0x44FF4444); // Simple glow
                engine.draw_rect(fx - 4, fy - 4, 8, 8, 0xFF4444);

                // Draw Snake
                for (i, part) in game.snake.body.iter().enumerate() {
                    let color = if i == 0 { 0x00FF00 } else { 0x00AA00 };
                    let size = if i == 0 { 12 } else { 10 };
                    engine.draw_rect(part.x as usize - size/2, part.y as usize - size/2, size, size, color);
                    
                    if i == 0 {
                        draw_eyes(&mut engine, part.x as usize - size/2, part.y as usize - size/2, game.snake.angle);
                    }
                }

                engine.draw_string(20, 20, &format!("SCORE {}", game.score), 0xFFFFFF, 2);
                engine.draw_string(20, 45, &format!("HIGH {}", game.high_score), 0x666666, 1);
                engine.draw_vignette();
            }
            GameState::Paused => {
                draw_header(&mut engine, "PAUSED");
                let items = ["RESUME", "SETTINGS", "MAIN MENU", "QUIT"];
                draw_menu(&mut engine, &items, menu_selected);

                if can_press {
                    if engine.is_key_down(Key::Up) {
                        menu_selected = (menu_selected + items.len() - 1) % items.len();
                        key_cooldown = now;
                    } else if engine.is_key_down(Key::Down) {
                        menu_selected = (menu_selected + 1) % items.len();
                        key_cooldown = now;
                    } else if engine.is_key_down(Key::Enter) || engine.is_key_down(Key::Space) {
                        match menu_selected {
                            0 => { state = GameState::Playing; }
                            1 => { previous_state = GameState::Paused; menu_selected = 0; state = GameState::Settings; }
                            2 => { state = GameState::Menu; menu_selected = 0; }
                            3 => return,
                            _ => {}
                        }
                        key_cooldown = now;
                    } else if engine.is_key_down(Key::Escape) {
                        state = GameState::Playing;
                        key_cooldown = now;
                    }
                }
                engine.draw_vignette();
            }
            GameState::GameOver => {
                engine.clear(COLOR_BG);
                draw_background_grid(&mut engine);
                draw_header(&mut engine, "GAME OVER");
                engine.draw_string_centered(180, &format!("FINAL SCORE: {}", game.score), 0xFFFFFF, 2);
                engine.draw_string_centered(215, &format!("BEST SCORE: {}", game.high_score), 0x00FF00, 1);
                
                let items = ["RESTART", "MAIN MENU", "QUIT"];
                draw_menu(&mut engine, &items, menu_selected);

                if can_press {
                    if engine.is_key_down(Key::Up) {
                        menu_selected = (menu_selected + items.len() - 1) % items.len();
                        key_cooldown = now;
                    } else if engine.is_key_down(Key::Down) {
                        menu_selected = (menu_selected + 1) % items.len();
                        key_cooldown = now;
                    } else if engine.is_key_down(Key::Enter) || engine.is_key_down(Key::Space) {
                        match menu_selected {
                            0 => { game.reset(); state = GameState::Playing; }
                            1 => { state = GameState::Menu; menu_selected = 0; }
                            2 => return,
                            _ => {}
                        }
                        key_cooldown = now;
                    }
                }
                engine.draw_vignette();
            }
        }
        engine.update();
    }
}

fn draw_background_grid(engine: &mut Engine) {
    for x in (0..WIDTH).step_by(40) {
        for y in 0..HEIGHT {
            engine.draw_pixel(x, y, COLOR_GRID);
        }
    }
    for y in (0..HEIGHT).step_by(40) {
        for x in 0..WIDTH {
            engine.draw_pixel(x, y, COLOR_GRID);
        }
    }
}

fn draw_header(engine: &mut Engine, title: &str) {
    // Drop shadow
    engine.draw_string_centered(62, title, 0x004400, 5);
    engine.draw_string_centered(60, title, 0x00FF00, 5);
}

fn draw_menu(engine: &mut Engine, items: &[&str], selected: usize) {
    for (i, item) in items.iter().enumerate() {
        let color = if i == selected { 0xFFFFFF } else { 0x444444 };
        let text = if i == selected { format!("> {}", item) } else { format!("  {}", item) };
        engine.draw_string_centered(300 + i * 50, &text, color, 2);
    }
}

fn draw_eyes(engine: &mut Engine, x: usize, y: usize, angle: f32) {
    let ex1 = (angle.cos() * 4.0 - angle.sin() * 3.0) as i32 + 6;
    let ey1 = (angle.sin() * 4.0 + angle.cos() * 3.0) as i32 + 6;
    let ex2 = (angle.cos() * 4.0 - angle.sin() * -3.0) as i32 + 6;
    let ey2 = (angle.sin() * 4.0 + angle.cos() * -3.0) as i32 + 6;
    
    engine.draw_rect((x as i32 + ex1) as usize, (y as i32 + ey1) as usize, 2, 2, 0x000000);
    engine.draw_rect((x as i32 + ex2) as usize, (y as i32 + ey2) as usize, 2, 2, 0x000000);
}
