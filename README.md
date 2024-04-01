# Afterlife Online

## Description
Afterlife Online is a desktop application that connects players to the immersive world of Afterlife Online, a multiplayer RPG. Built with the Tauri framework, it combines the security of Rust with the ease of web technologies (HTML, CSS, JavaScript) for its interface. This offers improved performance and reduced resource usage compared to traditional clients, for a seamless gaming experience.

Experience Afterlife Online through this client or via the web at [Afterlife Online](https://play.afterlife-online.com/).

## Getting Started

### For Players
To play Afterlife Online without the hassle of building the application, you can download pre-compiled binaries available for macOS, Linux, and Windows from our [Releases Page](https://github.com/spoonpaw/afterlife-online-tauri/releases).

#### Installation Instructions:

- **macOS**: Download `Afterlife.Online_1.0.0_x64.dmg`, mount it, and drag the application to your Applications folder.
- **Linux**:
   - **AppImage**: Download `afterlife-online_1.0.0_amd64.AppImage`, make it executable (`chmod +x afterlife-online_1.0.0_amd64.AppImage`), and run it directly.
   - **Debian Package**: Download `afterlife-online_1.0.0_amd64.deb` and install via your package manager or using `sudo dpkg -i afterlife-online_1.0.0_amd64.deb` in the terminal. For missing dependencies, run `sudo apt-get install -f`.
- **Windows**: Instructions to be added based on the provided Windows distribution format.

### For Developers and Contributors
If you wish to build Afterlife Online from source or contribute to its development, follow these prerequisites and installation steps.

#### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install): The core programming language and package manager, Cargo.
- [Tauri CLI](https://tauri.studio/en/docs/getting-started/intro): Required for building the Tauri application.

#### Installation
1. Clone the repository:
    ```bash
    git clone https://github.com/spoonpaw/afterlife-online-tauri.git
    ```
2. Navigate to the project folder:
    ```bash
    cd afterlife-online-tauri
    ```
3. Build the project:
    ```bash
    cargo tauri build
    ```

#### Development
- Run in development mode with live reloading:
    ```bash
    cargo tauri dev
    ```

#### Packaging
- Tauri packages the application for the OS you're using. After building, find the app in `src-tauri/target/release`.

## Features
- Secure, efficient access to Afterlife Online servers.
- Consistent, immersive gaming experience across all desktop platforms.
- Streamlined UI focused on game interaction without browser clutter.

## Version Information
Current version: 1.0.0 of the Afterlife Online.
