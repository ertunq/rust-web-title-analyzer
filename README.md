# Web Title Analysis Tool
This Rust CLI application extracts and analyzes heading tags (h1, h2, h3, etc.) from a specified website.
Features

Automatic heading extraction from website URLs
Statistical breakdown by heading level (h1-h6)
Option to save found headings to a file

# Clone the project
```bash
git clone https://github.com/kullaniciadi/web-title-analyzer.git
cd web-title-analyzer
```
# Install dependencies and build
```bash
cargo build --release
```

# Dependencies

reqwest: For HTTP requests
tokio: Asynchronous runtime
scraper: HTML parsing
clap: Command-line arguments
anyhow: Error handling
