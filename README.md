Image to ASCII Art Web Converter
This project is a web-based application built in Rust that converts uploaded images (like PNGs, JPEGs, etc.) into detailed ASCII art. It provides a simple, interactive user interface for uploading files, selecting conversion options, and viewing or downloading the results.

The application has evolved from a simple command-line tool into a full-featured web server using the Actix web framework.

Features
Interactive Web UI: Modern, user-friendly interface for uploading images.

Live Preview: Displays the generated ASCII art in a results page.

Customizable Output:

Character Sets: Choose between a simple set for basic art and a detailed set for more nuance.

Color Themes: Supports dark (light text on dark background) and light (dark text on light background) themes for the HTML preview.

Full Resolution: An option to convert the image pixel-for-pixel, bypassing resizing for maximum detail.

Dynamic Viewer: The generated HTML file is a self-contained viewer that scales the ASCII art to fit any browser window size.

Downloadable Files: Provides easy download links for both a plain .txt file and the interactive .html viewer.

Pure Rust Backend: The entire application logic, from image processing to serving web content, is handled by a high-performance Rust backend using Actix Web.

How to Run the Project
To run this application locally, you will need to have the Rust toolchain (including cargo) installed.

1. Clone or Set Up the Project
   Ensure you have the following files in your project directory:

Cargo.toml (with the required dependencies)

src/main.rs (the application source code)

index.html.txt (the frontend HTML, in the project root)

2. Update Dependencies
   Make sure your Cargo.toml file includes the following dependencies:

[dependencies]
actix-web = "4"
actix-multipart = "0.6"
actix-files = "0.6"
futures-util = "0.3"
sanitize-filename = "0.5"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
url-escape = "0.1.1"
image = "0.24"
anyhow = "1.0"

3. Build and Run the Server
   Navigate to the project's root directory in your terminal and run the following command:

cargo run

This command will compile the project and start the web server. If successful, you will see a message like this:

Starting server at [http://127.0.0.1:8080](http://127.0.0.1:8080)

4. Use the Application
   Open your favorite web browser and navigate to http://127.0.0.1:8080. You will see the upload page where you can select an image and your desired conversion options.

Technologies Used
Backend Framework: Actix Web

Image Processing: image-rs

Error Handling: anyhow

Asynchronous Runtime: Tokio