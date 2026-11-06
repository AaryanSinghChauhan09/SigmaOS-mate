use ::mate::{mate, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = mate::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-mate operational");
    Ok(())
}
