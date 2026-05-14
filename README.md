# Rust Snake - Ultra Edition 🐍

A high-performance, modern Snake game built from scratch in Rust. This version moves away from the traditional grid-based movement to a fluid, continuous system inspired by games like *Slither.io*.

![Snake Game Gameplay](assets/screenshot.png) *(Note: Add a real screenshot if you have one!)*

## ✨ Features

- **Fluid Movement**: Smooth, mouse-controlled steering with continuous positioning.
- **Ultra Performance**: Optimized rendering loop targeting 60 FPS.
- **Modern Aesthetics**:
    - High-contrast dark theme with grid background.
    - Dynamic vignette effect.
    - Glowing food items.
    - Animated snake eyes that follow the direction of movement.
- **Polished UI**: Complete game loop with Menu, Settings (Music Toggle), Paused, and Game Over states.
- **High Score System**: Tracks and saves your best performance locally.
- **Audio Experience**: Background music and SFX (powered by `rodio`).

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- OS Dependencies (for `minifb` and `rodio`):
    - **Linux**: `libx11-dev`, `libasound2-dev`

### Installation

1. Clone the repository:
   ```bash
   git clone git@github.com:augustuz-zeno/Rust-Snake-Game.git
   cd Rust-Snake-Game
   ```

2. Run the game:
   ```bash
   cargo run --release
   ```

## 🎮 Controls

- **Mouse**: Steering the snake.
- **Space / Enter**: Select in menus.
- **Esc**: Pause the game.
- **Up / Down**: Navigate menus.

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) - Programming language.
- [minifb](https://github.com/emoon/rust_minifb) - Windowing and basic framebuffer.
- [rodio](https://github.com/RustAudio/rodio) - Audio playback library.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
