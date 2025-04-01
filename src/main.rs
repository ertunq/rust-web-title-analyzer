use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Web sitelerindeki başlıkları analiz eder")]
struct Args {
    /// Analiz edilecek web sitesinin URL'si
    #[clap(short, long)]
    url: String,

    /// Başlıkları kaydetmek için dosya yolu (isteğe bağlı)
    #[clap(short, long)]
    output: Option<PathBuf>,
}

struct HeadingInfo {
    level: u8,
    text: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Web sitesi analiz ediliyor: {}", args.url);
    
    let headings = fetch_headings(&args.url).await?;
    
    if headings.is_empty() {
        println!("Web sitesinde başlık bulunamadı!");
        return Ok(());
    }
    
    // İstatistikler hesaplanıyor
    let stats = calculate_stats(&headings);
    
    // Sonuçları göster
    print_results(&headings, &stats);
    
    // Eğer çıktı dosyası belirtilmişse, başlıkları dosyaya kaydet
    if let Some(output_path) = args.output {
        save_to_file(&headings, &output_path)?;
        println!("Başlıklar {} dosyasına kaydedildi.", output_path.display());
    }
    
    Ok(())
}

async fn fetch_headings(url: &str) -> Result<Vec<HeadingInfo>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    
    let document = Html::parse_document(&html);
    let mut headings = Vec::new();
    
    // H1-H6 başlıklarını topla
    for level in 1..=6 {
        let selector_str = format!("h{}", level);
        let selector = match Selector::parse(&selector_str) {
            Ok(s) => s,
            Err(_) => {
                return Err(anyhow::anyhow!("Geçersiz seçici: {}", selector_str));
            }
        };
        
        for element in document.select(&selector) {
            let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
            if !text.is_empty() {
                headings.push(HeadingInfo {
                    level: level as u8,
                    text,
                });
            }
        }
    }
    
    Ok(headings)
}

fn calculate_stats(headings: &[HeadingInfo]) -> HashMap<u8, usize> {
    let mut stats = HashMap::new();
    
    for heading in headings {
        *stats.entry(heading.level).or_insert(0) += 1;
    }
    
    stats
}

fn print_results(headings: &[HeadingInfo], stats: &HashMap<u8, usize>) {
    println!("\n--- Başlık Analizi Sonuçları ---");
    println!("Toplam başlık sayısı: {}", headings.len());
    
    println!("\nBaşlık seviyelerine göre dağılım:");
    for level in 1..=6 {
        let count = stats.get(&(level as u8)).unwrap_or(&0);
        println!("  H{}: {} adet", level, count);
    }
    
    println!("\nBulunan başlıklar:");
    for (i, heading) in headings.iter().enumerate() {
        println!("{}. [H{}] {}", i + 1, heading.level, heading.text);
    }
}

fn save_to_file(headings: &[HeadingInfo], path: &PathBuf) -> Result<()> {
    let mut file = File::create(path)?;
    
    writeln!(file, "# Başlık Analizi\n")?;
    
    for (i, heading) in headings.iter().enumerate() {
        writeln!(file, "{}. [H{}] {}", i + 1, heading.level, heading.text)?;
    }
    
    Ok(())
}