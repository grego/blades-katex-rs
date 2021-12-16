use blades::Page;
use rayon::prelude::*;
use std::io::Read;

// Dependencies in Cargo.toml:
//
// [dependencies]
// blades = { version = "0.3.0-alpha", default_features = false }
// rayon = "1.5"
// serde_json = "1"
// katex = "0.4"

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = Vec::new();
    std::io::stdin().read_to_end(&mut source)?;
    let mut pages: Vec<Page> = serde_json::from_slice(&source)?;

    pages.par_iter_mut().for_each(|page| {
        let content = &page.content;
        // if content contains $.+$, replace it with
        // katex.render(equation)
    });

    serde_json::to_writer(std::io::stdout(), &pages)?;
    Ok(())
}
