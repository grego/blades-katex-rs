use blades::Page;
use fnv::FnvHasher;
use logos::Logos;
use nohash_hasher::IntMap;
use beef::Cow;
use std::fs::File;
use std::hash::Hasher;
use std::io::Read;

static CACHE_FILE: &str = ".rkatex.cache";

#[derive(Logos, Copy, Clone, Debug)]
enum Expr {
    #[regex(r"\$\$((?:[^\$]|\\\$)+)[^\\]\$\$", |_| true)]
    #[regex(r"\$((?:[^\$]|\\\$)+)[^\\]\$", |_| false)]
    Math(bool),

    #[error]
    Plaintext,
}

/// A wrapper that enables zero-copy deserialization.
#[derive(serde::Deserialize)]
#[serde(transparent)]
struct SerCow<'a>(#[serde(borrow)] Cow<'a, str>);

#[inline]
fn hash(s: &str, display: bool) -> u64 {
    let mut hasher = FnvHasher::default();
    hasher.write(s.as_ref());
    hasher.write_u8(display as u8);
    hasher.finish()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = Vec::new();
    std::io::stdin().read_to_end(&mut source)?;
    let mut pages: Vec<Page> = serde_json::from_slice(&source)?;

    let cache_data = std::fs::read(CACHE_FILE).unwrap_or_default();
    let cache: IntMap<u64, SerCow> = bincode::deserialize(&cache_data).unwrap_or_default();
    let mut cache: IntMap<u64, Cow<str>> = cache.into_iter().map(|(k, v)| (k, v.0)).collect();

   for page in &mut pages {
        let content = &page.content;
        let mut lex = Expr::lexer(content);
        let mut found = false;
        let mut rendered = String::new();

        while let Some(token) = lex.next() {
            let s = lex.slice();
            if let Expr::Math(display) = token {
                if !found {
                    found = true;
                    rendered.push_str(&content[..lex.span().start]);
                }

                let s = s.trim_matches('$');
                let hash = hash(s, display);
                if let Some(cached) = cache.get(&hash) {
                    rendered.push_str(cached);
                } else {
                    let mut opts = katex::Opts::default();
                    opts.set_display_mode(display);
                    let r = katex::render_with_opts(s, opts).unwrap_or_else(|e| e.to_string());
                    rendered.push_str(&r);
                    cache.insert(hash, r.into());
                }
            } else if found {
                rendered.push_str(s);
            }
        }

        if found {
            page.content = rendered.into();
        }
    }

    serde_json::to_writer(std::io::stdout(), &pages)?;
    bincode::serialize_into(File::create(CACHE_FILE)?, &cache)?;
    Ok(())
}
