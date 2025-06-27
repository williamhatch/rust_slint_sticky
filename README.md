# Rust Slint Sticky Notes

A cross-platform sticky notes application built with Rust and Slint GUI framework. Inspired by the [Sticky project](https://github.com/williamhatch/sticky), this application provides a native desktop experience for organizing your thoughts and ideas.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Slint](https://img.shields.io/badge/slint-1.12-green.svg)

## Features

- 📝 **Create and Edit Notes**: Easy-to-use interface for creating and editing sticky notes
- 🎨 **Customizable Colors**: Choose from multiple color themes for your notes
- 💾 **Persistent Storage**: Notes are automatically saved to local storage
- 🖥️ **Cross-Platform**: Runs on Windows, macOS, and Linux
- ⚡ **Native Performance**: Built with Rust for optimal speed and memory usage
- 🔄 **Real-time Updates**: Changes are saved automatically
- 📱 **Responsive Design**: Adapts to different screen sizes
- 🎯 **Lightweight**: Minimal resource consumption

## Technology Stack

- **Backend**: Rust with async/await support via Tokio
- **GUI Framework**: [Slint](https://slint.dev/) - Modern declarative GUI toolkit
- **Data Storage**: JSON-based file storage with atomic writes
- **Serialization**: Serde for JSON handling
- **Cross-platform**: Native support for Windows, macOS, and Linux

## Installation

### Prerequisites

- Rust 1.70 or later
- Git

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust_slint_sticky.git
cd rust_slint_sticky
```

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

### Development

For development with hot-reload capabilities:

```bash
cargo run
```

## Usage

### Creating Notes

1. Click the "Add New Note" button in the header
2. Enter a title and content for your note
3. Choose a color from the available palette
4. Click "Save Note" to create the note

### Editing Notes

1. Click on any existing note to edit it
2. Modify the title, content, or color as needed
3. Click "Save Note" to update the note

### Deleting Notes

1. Click the "×" button in the top-right corner of any note
2. The note will be permanently deleted

### Data Storage

Notes are automatically saved to:
- **Windows**: `%APPDATA%\rust_slint_sticky\notes.json`
- **macOS**: `~/Library/Application Support/rust_slint_sticky/notes.json`
- **Linux**: `~/.local/share/rust_slint_sticky/notes.json`

## Project Structure

```
rust_slint_sticky/
├── src/
│   ├── main.rs          # Application entry point and UI logic
│   ├── note.rs          # StickyNote struct and implementations
│   └── storage.rs       # Data persistence layer
├── ui/
│   └── app.slint        # Slint UI definition
├── build.rs             # Build script for Slint compilation
├── Cargo.toml           # Rust dependencies and project metadata
└── README.md           # Project documentation
```

## Configuration

The application uses sensible defaults but can be customized:

- **Note Colors**: Predefined color palette (Yellow, Orange, Green, Blue, Pink)
- **Default Note Size**: 220×200 pixels
- **Storage Location**: Platform-specific data directories

## Development

### Code Structure

- **UI Layer**: Defined in `ui/app.slint` using Slint's declarative syntax
- **Business Logic**: Rust code in `src/` handles data management and UI callbacks
- **Storage Layer**: Async file operations with atomic writes for data safety

### Adding Features

1. **UI Changes**: Modify `ui/app.slint` for visual components
2. **Business Logic**: Update Rust modules in `src/`
3. **Data Model**: Extend the `StickyNote` struct if needed

### Building for Different Platforms

#### Windows
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

#### macOS
```bash
cargo build --release --target x86_64-apple-darwin
```

#### Linux
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

## Roadmap

- [ ] **Drag & Drop**: Move notes around the interface
- [ ] **Search Functionality**: Find notes by content or title
- [ ] **Categories/Tags**: Organize notes with labels
- [ ] **Export/Import**: Backup and restore notes
- [ ] **Themes**: Dark mode and custom themes
- [ ] **Collaboration**: Real-time syncing across devices
- [ ] **Mobile Support**: iOS and Android apps

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust's official style guidelines
- Use `cargo fmt` for formatting
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality

## Testing

Run the test suite:

```bash
cargo test
```

Run with coverage:

```bash
cargo test --coverage
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Slint Team](https://slint.dev/) for the excellent GUI framework
- [Original Sticky Project](https://github.com/williamhatch/sticky) for inspiration
- The Rust community for amazing tools and libraries

## Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/rust_slint_sticky/issues) page
2. Create a new issue if your problem isn't already reported
3. Provide detailed information about your environment and the issue

---

**Enjoy organizing your thoughts with Rust Slint Sticky Notes!** 📝✨ 