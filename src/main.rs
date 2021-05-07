use id3::Tag;
use std::env;
use std::fs;

struct Track {
    path: String,
    tag: id3::Tag,
}

impl Track {
    pub fn from_path(path: &str) -> Result<Track, Box<dyn std::error::Error>> {
        Ok(Track {
            path: String::from(path),
            tag: Tag::read_from_path(path)?,
        })
    }

    pub fn rename(&mut self, new_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::rename(&self.path, new_path)?;
        self.path = String::from(new_path);
        Ok(())
    }
}

fn fmt_field(field: &str, tag: &id3::Tag) -> String {
    match field {
        "tracknumber" => tag.track().map(|t| t.to_string()),
        "artist" => tag.artist().map(String::from),
        "title" => tag.title().map(String::from),
        _ => None,
    }
    .unwrap_or_else(String::new)
}

fn format(fmt_str: &str, tag: &id3::Tag) -> String {
    let pv: Vec<&str> = fmt_str.split('%').collect();

    let mut is_tmpl = [false, true].iter().cycle();
    let mut parts = pv.iter();

    // Formatter string starts with a template variable.
    if pv[0].is_empty() {
        is_tmpl.next();
        parts.next();
    }

    parts
        .zip(is_tmpl)
        .map(|i| match i {
            (p, true) => fmt_field(p, tag),
            (&p, false) => String::from(p),
        })
        .collect::<String>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut track = Track::from_path(&args[1])?;

    println!("Renaming {}", track.path);
    track.rename(&format("%tracknumber%. %title%.mp3", &track.tag))?;
    println!("Renamed to {}", track.path);
    Ok(())
}
