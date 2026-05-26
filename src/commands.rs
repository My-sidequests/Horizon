use std::fs;
use std::io::{self, Write};
use std::process::Command as Cmd;

use crate::exercises::EXERCISES;
use crate::progress::Progress;
use crate::ui;

// ─── horizon start ────────────────────────────────────────────────────────────

pub fn start() {
    let ex_dir = Progress::exercises_dir();
    fs::create_dir_all(&ex_dir).expect("Could not create exercises directory");

    println!();
    println!("  {}  Compiling exercises...", "→");
    println!();

    let mut all_ok = true;

    for ex in EXERCISES {
        let dest = ex_dir.join(ex.slug);
        fs::create_dir_all(&dest).expect("Could not create exercise directory");

        let src_path = dest.join("target.c");
        fs::write(&src_path, ex.c_source).expect("Could not write source file");

        fs::write(dest.join("solution"), ex.solution).expect("Could not write solution");

        let binary = dest.join("target");
        let mut cmd = Cmd::new("gcc");
        cmd.arg("-w");
        for flag in ex.compile_flags {
            cmd.arg(flag);
        }
        cmd.arg("-o").arg(&binary).arg(&src_path);

        let ok = cmd.status().map(|s| s.success()).unwrap_or(false);
        let _ = fs::remove_file(&src_path);

        ui::compile_progress(ex.slug, ok);
        if !ok {
            all_ok = false;
        }
    }

    if all_ok {
        ui::start_done(&ex_dir.display().to_string());
    } else {
        println!();
        println!("  {}  Some exercises failed to compile.", "!");
        println!("  Make sure gcc is installed: gcc --version");
        println!();
    }

    Progress::default().save();
}

// ─── horizon update ───────────────────────────────────────────────────────────

pub fn update() {
    ui::update_start();

    let status = Cmd::new("cargo")
        .args([
            "install",
            "--git",
            "https://github.com/My-sidequests/Horizon.git",
            "--force",
        ])
        .status();

    match status {
        Ok(s) if s.success() => ui::update_done(),
        Ok(_)  => ui::update_failed("cargo install returned a non-zero exit code"),
        Err(e) => ui::update_failed(&format!("could not run cargo: {}", e)),
    }
}

// ─── horizon (no subcommand) ──────────────────────────────────────────────────

pub fn show() {
    let progress = Progress::load();

    if !exercises_compiled() {
        ui::not_started();
        return;
    }

    let total = EXERCISES.len();
    if progress.current >= total {
        ui::all_done(total);
        return;
    }

    ui::show_exercise(progress.current, &EXERCISES[progress.current], &progress, total);
}

// ─── horizon done <FLAG> ──────────────────────────────────────────────────────

pub fn done(flag: String) {
    if !exercises_compiled() {
        ui::not_started();
        return;
    }

    let mut progress = Progress::load();
    let total        = EXERCISES.len();
    let idx          = progress.current;

    if idx >= total {
        ui::all_done(total);
        return;
    }

    let ex = &EXERCISES[idx];

    // Validate the flag
    if flag.trim() != ex.flag {
        ui::wrong_flag(&flag);
        return;
    }

    progress.completed.insert(idx);
    let next = next_uncompleted(&progress, idx + 1, total);
    progress.current = next;
    progress.save();

    ui::correct_flag(ex.title);
    println!();

    if next >= total {
        ui::all_done(total);
    } else {
        ui::show_exercise(next, &EXERCISES[next], &progress, total);
    }
}

// ─── horizon hint ─────────────────────────────────────────────────────────────

pub fn hint() {
    if !exercises_compiled() {
        ui::not_started();
        return;
    }

    let mut progress = Progress::load();
    let idx          = progress.current;
    let total        = EXERCISES.len();

    if idx >= total {
        ui::all_done(total);
        return;
    }

    let ex = &EXERCISES[idx];
    let hints: Vec<&str> = ex
        .hints
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    match progress.next_hint_index(idx, hints.len()) {
        Some(i) => ui::show_hint(i + 1, hints.len(), hints[i]),
        None     => ui::no_more_hints(),
    }

    progress.save();
}

// ─── horizon skip ─────────────────────────────────────────────────────────────

pub fn skip() {
    if !exercises_compiled() {
        ui::not_started();
        return;
    }

    let mut progress = Progress::load();
    let total        = EXERCISES.len();
    let idx          = progress.current;

    if idx >= total {
        ui::all_done(total);
        return;
    }

    let title = EXERCISES[idx].title;
    let next  = next_uncompleted(&progress, idx + 1, total);
    progress.current = next;
    progress.save();

    ui::skipped(title);
    println!();

    if next >= total {
        ui::all_done(total);
    } else {
        ui::show_exercise(next, &EXERCISES[next], &progress, total);
    }
}

// ─── horizon reset ────────────────────────────────────────────────────────────

pub fn reset() {
    print!("  Reset all progress? [y/N] ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            Progress::default().save();
            println!("  Progress reset. Run {} to start over.", "horizon start");
        }
        _ => println!("  Cancelled."),
    }
    println!();
}

// ─── horizon list ─────────────────────────────────────────────────────────────

pub fn list() {
    let progress  = Progress::load();
    let exercises: Vec<&_> = EXERCISES.iter().collect();
    ui::show_list(&exercises, &progress);
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn exercises_compiled() -> bool {
    Progress::exercises_dir()
        .join(EXERCISES[0].slug)
        .join("target")
        .exists()
}

fn next_uncompleted(progress: &Progress, from: usize, total: usize) -> usize {
    (from..total).find(|&i| !progress.is_done(i)).unwrap_or(total)
}
