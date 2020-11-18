use clap::Clap;
use std::{env, path::PathBuf};
use strikeout::{link, scan};

#[derive(Clap)]
#[clap(version=clap::crate_version!(), author=clap::crate_authors!())]
struct Opts {
    src: PathBuf,
    dest: PathBuf,
    #[clap(short, long = "dry-run")]
    dry_run: bool,
    #[clap(short, long = "working-dir")]
    working_dir: Option<PathBuf>,
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let Opts {
        src,
        dest,
        dry_run,
        working_dir,
        verbose,
    } = Opts::parse();
    // log set
    let log_level = if verbose {
        simplelog::LevelFilter::max()
    } else {
        simplelog::LevelFilter::Error
    };
    let log_config = simplelog::Config::default();
    simplelog::SimpleLogger::init(log_level, log_config).expect("log set failed");
    // change working dir
    if let Some(ref working_dir) = working_dir {
        env::set_current_dir(working_dir).expect("Working dir change failed");
    }
    // get file list
    let mut file_set = scan::get_file_list().unwrap_or_default();
    let list = scan::scan_new_file(&src, &mut file_set);
    for file in list.into_iter() {
        if let Ok(target) = link::map_to_dest(file.path(), &src, &dest) {
            log::info!("{} -> {}", file.path().display(), target.display());
            if !dry_run {
                if let Err(e) = link::link_to(file.path(), &target) {
                    log::error!("{} link failed. {}.", file.path().display(), e);
                }
            } else {
                log::info!("{} skiped.", file.path().display())
            }
        }
    }
    if !dry_run {
        if let Err(e) = scan::store_file_list(&file_set) {
            log::error!("File list cache failed.\n{}", e);
        };
    }
}
