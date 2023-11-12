# ClipPop - Clipboard History App

ClipPop is a simple and lightweight clipboard history application built with the [Iced](https://github.com/hecrj/iced) Rust GUI framework. It allows you to keep track of your clipboard history and quickly access previously copied items.

## Features

- Monitor clipboard changes and store a history of copied text.
- Quickly access and copy previously copied items.
- Clear the clipboard history.
- A user-friendly and minimalistic user interface.

## Getting Started

### Prerequisites

Before building and running ClipPop, make sure you have the following installed on your system:

- Rust programming language. You can install it from [rust-lang.org](https://www.rust-lang.org/learn/get-started).

### Building and Running

1. Clone the ClipPop repository:

   ```bash
   git clone https://github.com/your-username/clip-pop.git
   cd clip-pop
   ```

2. Build the application using `cargo`:

```
cargo build --release
```

This will compile the application in release mode for optimized performance.

Run the application:

```
cargo run --release
```

The application window will open, and ClipPop will start monitoring your clipboard.

### Usage

When you copy text to your clipboard, ClipPop will automatically add it to the history list.
Click on an item in the history list to copy it back to the clipboard.
Use the "Clear Clipboard" button to clear the clipboard history.

### Contributing
Contributions to the EasyTime library are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve this library.