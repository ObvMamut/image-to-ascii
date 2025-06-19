use actix_multipart::Multipart;
use actix_web::{get, post, App, Error, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use futures_util::stream::StreamExt;
use image::{DynamicImage, ImageError};
use sanitize_filename::sanitize;
use std::io::Write;
use std::path::PathBuf;
use url_escape;

// --- ASCII CONVERSION LOGIC ---

const SIMPLE_CHARS: &str = " .:-=+*#%@";
const DETAILED_CHARS: &str = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(Clone, Copy)]
enum ColorTheme {
    Dark,
    Light,
}

struct AsciiConfig {
    width: u32,
    use_full_resolution: bool,
    character_set: Vec<char>,
    invert_mapping: bool,
    aspect_ratio_correction: f32,
    background_color: String,
    text_color: String,
}

struct AsciiConverter {
    config: AsciiConfig,
}

impl AsciiConverter {
    fn new(config: AsciiConfig) -> Self {
        Self { config }
    }

    fn load_image_from_memory(&self, buffer: &[u8]) -> Result<DynamicImage, ImageError> {
        image::load_from_memory(buffer)
    }

    fn resize_image(&self, img: &DynamicImage) -> DynamicImage {
        let original_width = img.width();
        let original_height = img.height();
        let new_height = ((original_height as f32 * self.config.width as f32)
            / original_width as f32 * self.config.aspect_ratio_correction)
            .max(1.0) as u32;
        img.resize_exact(self.config.width, new_height, image::imageops::FilterType::Lanczos3)
    }

    fn pixel_to_ascii(&self, brightness: u8) -> char {
        let char_count = self.config.character_set.len();
        let mut char_index = (brightness as f32 / 255.0 * (char_count - 1) as f32).round() as usize;
        if self.config.invert_mapping {
            char_index = char_count - 1 - char_index;
        }
        self.config.character_set[char_index]
    }

    fn convert_to_ascii(&self, img: &DynamicImage) -> (String, (u32, u32)) {
        let source_img = if self.config.use_full_resolution {
            println!("Using full resolution ({}x{})", img.width(), img.height());
            img.clone()
        } else {
            println!("Resizing image to width: {}", self.config.width);
            self.resize_image(img)
        };

        let gray_img = source_img.to_luma8();
        let (width, height) = gray_img.dimensions();
        let capacity = (width * height + height) as usize;
        let mut ascii_art = String::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                let brightness = gray_img.get_pixel(x, y)[0];
                ascii_art.push(self.pixel_to_ascii(brightness));
            }
            ascii_art.push('\n');
        }
        (ascii_art, (width, height))
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn generate_html_viewer(
    ascii_art: &str,
    dimensions: (u32, u32),
    bg_color: &str,
    txt_color: &str,
) -> String {
    let escaped_art = html_escape(ascii_art);
    let (art_width, art_height) = dimensions;
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ASCII Art Viewer</title>
    <style>
        html, body {{ margin: 0; padding: 0; width: 100%; height: 100%; display: flex; justify-content: center; align-items: center; background-color: {bg_color}; overflow: hidden; }}
        pre {{ color: {txt_color}; font-family: 'Courier New', Courier, monospace; white-space: pre; font-size: 10px; line-height: 0.8em; }}
    </style>
</head>
<body>
<pre id="ascii-art">{escaped_art}</pre>
<script>
    (function() {{
        const artElement = document.getElementById('ascii-art');
        const artCols = {art_width}; const artRows = {art_height};
        const FONT_ASPECT_RATIO = 0.6;
        function resizeArt() {{
            const fontSizeForWidth = (window.innerWidth / artCols) * FONT_ASPECT_RATIO;
            const fontSizeForHeight = window.innerHeight / artRows;
            artElement.style.fontSize = Math.min(fontSizeForWidth, fontSizeForHeight) + 'px';
        }}
        window.addEventListener('resize', resizeArt);
        document.addEventListener('DOMContentLoaded', resizeArt);
    }})();
</script>
</body>
</html>"#,
        bg_color = bg_color,
        txt_color = txt_color,
        escaped_art = escaped_art,
        art_width = art_width,
        art_height = art_height
    )
}

// --- WEB SERVER LOGIC ---

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html"))
}

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut image_data: Option<Vec<u8>> = None;
    let mut theme = ColorTheme::Dark;
    let mut detailed = false;
    let mut full_resolution = false;
    let mut original_filename = "image".to_string();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap_or_default();

        match field_name {
            "image" => {
                original_filename = sanitize(content_disposition.get_filename().unwrap_or("image.png"));
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await { data.extend_from_slice(&chunk?); }
                if !data.is_empty() { image_data = Some(data); }
            }
            "theme" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await { data.extend_from_slice(&chunk?); }
                if String::from_utf8(data).unwrap_or_default() == "light" { theme = ColorTheme::Light; }
            }
            "detailed" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await { data.extend_from_slice(&chunk?); }
                if String::from_utf8(data).unwrap_or_default() == "true" { detailed = true; }
            }
            "full_resolution" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await { data.extend_from_slice(&chunk?); }
                if String::from_utf8(data).unwrap_or_default() == "true" { full_resolution = true; }
            }
            _ => (),
        }
    }

    let image_data = match image_data {
        Some(data) => data,
        None => return Ok(HttpResponse::BadRequest().body("No image uploaded.")),
    };

    let (bg_color, txt_color, invert_mapping) = match theme {
        ColorTheme::Dark => ("#1a1a1a", "#e0e0e0", false),
        ColorTheme::Light => ("#f0f0f0", "#111111", true),
    };

    let char_string = if detailed { DETAILED_CHARS } else { SIMPLE_CHARS };
    let character_set = char_string.chars().collect();

    let config = AsciiConfig {
        width: 150, // Default width if not full resolution
        use_full_resolution: full_resolution,
        character_set,
        invert_mapping,
        aspect_ratio_correction: 0.5,
        background_color: bg_color.to_string(),
        text_color: txt_color.to_string(),
    };

    let converter = AsciiConverter::new(config);
    let img = converter.load_image_from_memory(&image_data).context("Failed to decode image").unwrap();
    let (ascii_art, dimensions) = converter.convert_to_ascii(&img);
    let html_viewer = generate_html_viewer(&ascii_art, dimensions, bg_color, txt_color);

    let filename_base = PathBuf::from(&original_filename).file_stem().unwrap().to_str().unwrap().to_string();
    let txt_filename = format!("{}.txt", filename_base);
    let html_filename = format!("{}.html", filename_base);

    let result_html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>ASCII Art Result</title>
            <style>
                body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background-color: #f0f2f5; margin: 0; padding: 20px; text-align: center; }}
                h1 {{ color: #333; }}
                .container {{ max-width: 1200px; margin: 0 auto; background: #fff; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); padding: 20px; }}
                .preview-container {{ width: 100%; height: 70vh; border: 1px solid #ddd; margin-top: 20px; border-radius: 8px; overflow: hidden; }}
                .download-links {{ margin-top: 20px; }}
                .download-links a {{ display: inline-block; padding: 12px 24px; background-color: #007bff; color: white; text-decoration: none; border-radius: 5px; margin: 0 10px; font-weight: bold; transition: background-color 0.2s; }}
                .download-links a:hover {{ background-color: #0056b3; }}
                a.home-link {{ display: inline-block; margin-top: 20px; color: #007bff; }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Your ASCII Art is Ready!</h1>
                <div class="preview-container">
                    <iframe srcdoc="{}" style="width:100%; height:100%; border:0;"></iframe>
                </div>
                <div class="download-links">
                    <a href="data:text/plain;charset=utf-8,{}" download="{}">Download .txt File</a>
                    <a href="data:text/html;charset=utf-8,{}" download="{}">Download .html Viewer</a>
                </div>
                <a href="/" class="home-link">Convert another image</a>
            </div>
        </body>
        </html>
        "#,
        html_escape(&html_viewer),
        url_escape::encode_component(&ascii_art),
        txt_filename,
        url_escape::encode_component(&html_viewer),
        html_filename
    );

    Ok(HttpResponse::Ok().content_type("text/html").body(result_html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(upload)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
