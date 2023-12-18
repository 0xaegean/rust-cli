use clap::Parser;

// bir dosyada bir pattern arayın ve onu içeren satırları görüntüleyin
#[derive(Parser)]
struct Cli {
    // aranacak pattern
    pattern: String,
    // okunacak dosya yolu
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content  = std::fs::read_to_string(&args.path)
        .with_context(|| format!("dosya okunamadı `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
