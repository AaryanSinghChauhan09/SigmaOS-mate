use anyhow::Result;
use clap::Parser;
use mate::mate;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_integration: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = mate::Config { 
        desktop_integration: !cli.no_integration, 
        ..Default::default() 
    };
    let mut component = mate::new(config);
    
    component.initialize()?;
    component.launch()?;
    
    println!("SigmaOS-mate ready");
    Ok(())
}
