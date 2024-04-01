# Afterlife Online

## Description
Afterlife Online is a desktop application that connects players to the immersive world of Afterlife Online, a multiplayer RPG. This client is built using the Tauri framework, combining the security and efficiency of Rust with straightforward web technologies (HTML, CSS, and JavaScript) for the user interface. Unlike traditional web-based or Electron clients, this Tauri client offers enhanced performance and reduced resource consumption, providing a seamless gaming experience directly from your desktop.

Play Afterlife Online directly through this dedicated client or via the web at [Afterlife Online](https://play.afterlife-online.com/).

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install): The core programming language used, along with its package manager, Cargo.
- [Tauri CLI](https://tauri.studio/en/docs/getting-started/intro): Necessary for building and running the Tauri application.

## Installation
Follow these steps to set up the Afterlife Online on your local machine:

1. Clone the repository:
    ```bash
    git clone https://github.com/spoonpaw/afterlife-online-tauri.git
    ```
2. Navigate to the project folder:
    ```bash
    cd afterlife-online-tauri
    ```
3. Build the project (this compiles the Rust code and packages the web assets):
    ```bash
    cargo tauri build
    ```

## Usage

### Development
Run the client in a development environment to connect to Afterlife Online with live reloading:
```bash
cargo tauri dev
```

### Production
Build and run the production version of the client:
```bash
cargo tauri build
```
This command prepares a distributable version of the application, optimized for performance.

## Packaging
Tauri automatically packages your application for the operating system you're using. After running the build command, find your application in the `src-tauri/target/release` directory.

## Features
- Secure, efficient access to the Afterlife Online game servers.
- A consistent and immersive gaming experience across all supported desktop platforms.
- A streamlined user interface focused on game interaction without the clutter of traditional web browsers.

## Version Information
Current version: 1.0.0 of the Afterlife Online.
