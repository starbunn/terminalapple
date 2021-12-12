extern crate ytd_rs;
use ytd_rs::{YoutubeDL, Arg};
use std::path::PathBuf;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let exe = std::env::current_exe().unwrap();
    let exe_dir = exe.parent().unwrap();
    // TODO: figure out why this isn't working
    let args = vec![Arg::new("-f"), Arg::new("'bestvideo,bestaudio'"), Arg::new("-o"), Arg::new("'%(title)s.f%(format_id)s.%(ext)s'")];
    let link = "https://www.youtube.com/watch?v=ABe4o099M8s";
    let path = PathBuf::from(exe_dir);
    let bad_apple = YoutubeDL::new(&path, args, link)?;

    // start download
    let _download = bad_apple.download();
    Ok(())
}
// youtube-dl --quiet -f 'bestvideo,bestaudio' -o '%(title)s.f%(format_id)s.%(ext)s' https://www.youtube.com/watch\?v\=ABe4o099M8s