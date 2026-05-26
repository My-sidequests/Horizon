use std::fs;
use std::path::{Path, PathBuf};

use crate::progress::Progress;
use crate::ui;

// ─── Exercise on disk ─────────────────────────────────────────────────────────

pub struct Exercise {
    pub name:   String, // directory name, e.g. "01_return_oriented"
    pub path:   PathBuf,
    pub brief:  String,
    pub hints:  Vec<String>,
}

impl Exercise {
    fn load(dir: &Path) -> Result<Self, String> {
        let name = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let brief = fs::read_to_string(dir.join("README"))
            .map_err(|_| format!("{}: missing README file", name))?;

        let hints = fs::read_to_string(dir.join("hints"))
            .unwrap_or_default()
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(String::from)
            .collect();

        Ok(Self { name, path: dir.to_path_buf(), brief, hints })
    }
}

// ─── Load all exercises from the exercises/ directory ─────────────────────────

fn load_all() -> Vec<Exercise> {
    let root = exercises_root();

    let mut dirs: Vec<PathBuf> = match fs::read_dir(&root) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect(),
        Err(_) => {
            eprintln!(
                "  Could not read exercises directory at '{}'.",
                root.display()
            );
            eprintln!("  Run horizon from the project root.");
            std::process::exit(1);
        }
    };

    dirs.sort(); // alphabetical = numbered order

    let mut exercises = Vec::new();
    for dir in dirs {
        match Exercise::load(&dir) {
            Ok(ex) => exercises.push(ex),
            Err(e)  => eprintln!("  Warning: {}", e),
        }
    }

    if exercises.is_empty() {
        eprintln!("  No exercises found in '{}'.", root.display());
        std::process::exit(1);
    }

    exercises
}

fn exercises_root() -> PathBuf {
    if let Ok(p) = std::env::var("HORIZON_EXERCISES") {
        return PathBuf::from(p);
    }
    let local = PathBuf::from("exercises");
    if local.exists() {
        return local;
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let p = parent.join("exercises");
            if p.exists() {
                return p;
            }
        }
    }
    PathBuf::from("exercises")
}

// ─── Commands ─────────────────────────────────────────────────────────────────

pub fn watch() {
    let exercises = load_all();
    let progress  = Progress::load();

    let idx = progress.current;

    if idx >= exercises.len() {
        ui::all_done(exercises.len());
        return;
    }

    let ex = &exercises[idx];
    ui::show_exercise(ex, idx, &exercises, &progress);
}

pub fn done() {
    let exercises = load_all();
    let mut progress = Progress::load();

    let idx = progress.current;
    if idx >= exercises.len() {
        ui::all_done(exercises.len());
        return;
    }

    progress.completed.insert(idx);

    let next = find_next(&exercises, &progress, idx);
    progress.current = next;
    progress.save();

    if next >= exercises.len() {
        ui::all_done(exercises.len());
    } else {
        ui::marked_done(&exercises[idx].name);
        println!();
        ui::show_exercise(&exercises[next], next, &exercises, &progress);
    }
}

pub fn hint() {
    let exercises = load_all();
    let mut progress = Progress::load();

    let idx = progress.current;
    if idx >= exercises.len() {
        ui::all_done(exercises.len());
        return;
    }

    let ex = &exercises[idx];
    match progress.next_hint(idx, &ex.hints) {
        Some(text) => {
            let shown = *progress.hints_shown.get(&idx).unwrap_or(&0);
            ui::show_hint(shown, ex.hints.len(), text);
        }
        None => ui::no_more_hints(),
    }

    progress.save();
}

pub fn skip() {
    let exercises = load_all();
    let mut progress = Progress::load();

    let idx = progress.current;
    if idx >= exercises.len() {
        ui::all_done(exercises.len());
        return;
    }

    let name = exercises[idx].name.clone();
    let next = find_next_uncompleted(&exercises, &progress, idx + 1);
    progress.current = next;
    progress.save();

    if next >= exercises.len() {
        ui::skipped(&name);
        ui::all_done(exercises.len());
    } else {
        ui::skipped(&name);
        println!();
        ui::show_exercise(&exercises[next], next, &exercises, &progress);
    }
}

pub fn list() {
    let exercises = load_all();
    let progress  = Progress::load();
    ui::show_list(&exercises, &progress);
}

pub fn reset() {
    Progress::default().save();
    println!("  Progress reset.");
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Find the next uncompleted exercise starting at idx, wrapping to 0 if needed.
fn find_next(exercises: &[Exercise], progress: &Progress, current: usize) -> usize {
    // Try everything after current first, then from the beginning
    let order = (current + 1..exercises.len()).chain(0..=current);
    for i in order {
        if !progress.is_done(i) {
            return i;
        }
    }
    exercises.len() // all done
}

fn find_next_uncompleted(exercises: &[Exercise], progress: &Progress, from: usize) -> usize {
    for i in from..exercises.len() {
        if !progress.is_done(i) {
            return i;
        }
    }
    exercises.len()
}
