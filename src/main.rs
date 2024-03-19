use pipey::Pipey;
use std::{
    env,
    path::{Path, PathBuf},
};

const SPACE: &str = "   ";

// rust paths are a pain in the ass

fn last_str_component(path: &Path) -> String {
    match path.file_name() {
        Some(s) => s.to_string_lossy().to_string(),
        _ => path.to_string_lossy().to_string(),
    }
}

fn current_dir(path: &Path) -> Vec<PathBuf> {
    path.read_dir()
        .unwrap()
        .map(|r| r.unwrap().path())
        .collect()
}

// applies f to input if input is Ok else default
fn apply_f_on_ok_or_default<P, R, E>(p: Result<P, E>, f: impl FnOnce(P) -> R, u: R) -> R {
    match p {
        Ok(v) => f(v),
        Err(_) => u,
    }
}

fn recursive_colored_symlink(path: &Path) -> String {
    apply_f_on_ok_or_default(
        path.read_link(),
        |l| {
            if l.is_symlink() {
                format!("{} -> {}", l.display(), recursive_colored_symlink(&l))
            } else {
                l.display().to_string()
            }
        },
        format!("\x1b[1;31mbroken symlink: {:?}\x1b[m", path.display()),
    )
}
fn print_file(path: &Path, depth: usize) {
    if path.is_symlink() {
        println!(
            "{}\x1b[1;33m└── {} -> {}\x1b[m",
            SPACE.repeat(depth),
            last_str_component(path),
            recursive_colored_symlink(path)
        );
    } else {
        println!(
            "{}\x1b[1;32m└── {}\x1b[m",
            SPACE.repeat(depth),
            last_str_component(path)
        );
    }
}

fn print_whole_dir(ps: &[PathBuf], depth: usize) {
    if ps.len() <= 0 {
        return;
    }
    // moved due to function being too big
    if !ps[0].is_dir() {
        print_file(&ps[0], depth);
    } else {
        println!(
            "{}\x1b[1;35m{}/\x1b[m",
            SPACE.repeat(depth),
            last_str_component(&ps[0])
        );
        print_whole_dir(&current_dir(&ps[0]), depth + 1);
    }
    print_whole_dir(&ps[1..], depth)
}

fn main() {
    env::args()
        .map(|a| PathBuf::from(a))
        .collect::<Vec<PathBuf>>()[1..]
        .pipe(|ps| print_whole_dir(&ps, 0))
}
