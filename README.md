# ASCII Art Generator

A web-based tool that converts images into ASCII art using Rust and Actix Web. Upload any image and transform it into beautiful text-based art with customizable options.

## Features

- **Web Interface**: Clean, intuitive HTML interface for easy image uploads
- **Multiple Themes**: Dark theme (for terminals) and light theme (for printing)
- **Character Set Options**: Choose between simple or detailed ASCII character sets
- **Resolution Control**: Option to use full resolution or optimized width
- **Download Options**: Export as both `.txt` and `.html` files
- **Live Preview**: View your ASCII art in an interactive HTML viewer
- **Drag & Drop**: Support for drag-and-drop file uploads

## Screenshots

The application provides:
- A simple upload interface with theme and quality options
- Real-time preview of generated ASCII art
- Downloadable files in multiple formats

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd ascii-art-generator
```

2. Install dependencies:
```bash
cargo build
```

3. Run the application:
```bash
cargo run
```

4. Open your browser and navigate to:
```
http://127.0.0.1:8080
```

## Usage

### Basic Usage

1. **Upload Image**: Click the upload area or drag and drop an image file
2. **Choose Options**:
   - **Theme**: Select dark (terminal-friendly) or light (print-friendly)
   - **Character Set**: Enable detailed characters for higher quality output
   - **Resolution**: Use full resolution for maximum detail (may be slower)
3. **Generate**: Click the "Generate" button to create your ASCII art
4. **Download**: Save as `.txt` file or `.html` viewer

### Supported Formats

The application supports common image formats including:
- JPEG/JPG
- PNG
- GIF
- BMP
- And other formats supported by the Rust `image` crate

### Character Sets

- **Simple**: ` .:-=+*#%@` (10 characters)
- **Detailed**: ` .'^",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$` (65+ characters)

## Configuration

### Default Settings

- **Default Width**: 150 characters (when not using full resolution)
- **Aspect Ratio Correction**: 0.5 (compensates for character height/width ratio)
- **Filter**: Lanczos3 for high-quality resizing

### Themes

#### Dark Theme
- Background: `#1a1a1a`
- Text: `#e0e0e0`
- Optimized for dark terminals and screens

#### Light Theme
- Background: `#f0f0f0`
- Text: `#111111`
- Inverted brightness mapping for printing

## Technical Details

### Architecture

- **Backend**: Rust with Actix Web framework
- **Frontend**: Vanilla HTML, CSS, and JavaScript
- **Image Processing**: Rust `image` crate for loading and manipulation
- **File Handling**: Actix Multipart for upload processing

### Key Components

#### AsciiConverter
- Handles image loading from memory
- Performs resizing with aspect ratio correction
- Converts pixels to ASCII characters based on brightness

#### Web Server
- Serves static HTML interface
- Processes multipart form uploads
- Generates downloadable content with proper MIME types

### Performance Considerations

- **Memory Efficient**: Streams file uploads without loading entire files in memory
- **Optimized Resizing**: Uses Lanczos3 filtering for quality
- **Configurable Resolution**: Balance between quality and processing time

## Dependencies

### Rust Crates

```toml
[dependencies]
actix-web = "4"
actix-multipart = "0.6"
image = "0.24"
futures-util = "0.3"
anyhow = "1.0"
sanitize-filename = "0.5"
url-escape = "0.1"
```

## API Endpoints

### GET `/`
Returns the main HTML interface for uploading images.

### POST `/upload`
Processes image uploads with the following form fields:
- `image`: Image file (required)
- `theme`: "dark" or "light" (default: "dark")
- `detailed`: "true" to use detailed character set
- `full_resolution`: "true" to skip resizing

## Development

### Running in Development

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080` with hot reloading for Rust code changes.

### Building for Production

```bash
cargo build --release
```

The optimized binary will be available in `target/release/`.

## Customization

### Adding New Character Sets

Modify the constants in `main.rs`:

```rust
const CUSTOM_CHARS: &str = "your_characters_here";
```

### Adjusting Default Settings

Update the `AsciiConfig` in the upload handler:

```rust
let config = AsciiConfig {
    width: 200,  // Change default width
    aspect_ratio_correction: 0.6,  // Adjust aspect ratio
    // ... other settings
};
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source. Please check the LICENSE file for details.

## Troubleshooting

### Common Issues

**"No image uploaded" error**
- Ensure you've selected a valid image file
- Check that the file isn't corrupted

**Slow processing with full resolution**
- Large images may take time to process
- Consider using the standard width option for faster results

**Browser compatibility**
- Modern browsers are recommended for the best experience
- JavaScript must be enabled for the interface to work properly

### Performance Tips

- Use JPEG format for photographs (smaller file size)
- PNG works well for graphics with fewer colors
- Enable "detailed characters" only when needed for quality
- Use "full resolution" sparingly for very large images

## Acknowledgments

- Built with [Actix Web](https://actix.rs/)
- Image processing powered by the [image crate](https://crates.io/crates/image)
- Character sets inspired by classic ASCII art traditions
