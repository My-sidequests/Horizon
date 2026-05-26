use std::io::{self, Write};
use std::path::Path;
use std::process::Command as Cmd;
use std::thread;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};

use crate::exercise::{self, Exercise, Progress};
use crate::ui;

// ─── Public commands ──────────────────────────────────────────────────────────

pub fn watch(root: &Path) {
    let exercises = load_or_die(root);
    loop {
        let mut progress = Progress::load();
        match exercises.iter().find(|e| !progress.is_complete(e.meta.number)) {
            None => {
                ui::print_all_done(exercises.len());
                return;
            }
            Some(ex) => {
                if !run_exercise(ex, &exercises, &mut progress) {
                    return; // user pressed q
                }
            }
        }
    }
}

pub fn run_named(root: &Path, query: &str) {
    let exercises = load_or_die(root);
    let found = exercises.iter().find(|e| {
        e.meta.name.to_lowercase().contains(&query.to_lowercase())
            || e.meta.number.to_string() == query
    });
    match found {
        Some(ex) => {
            let mut progress = Progress::load();
            run_exercise(ex, &exercises, &mut progress);
        }
        None => eprintln!(
            "Exercise '{}' not found. Run `horizon list` to see available names.",
            query
        ),
    }
}

pub fn show_hint(root: &Path) {
    let exercises = load_or_die(root);
    let mut progress = Progress::load();
    match exercises.iter().find(|e| !progress.is_complete(e.meta.number)) {
        Some(ex) => give_hint(ex, &mut progress),
        None      => println!("All exercises completed!"),
    }
}

pub fn list(root: &Path) {
    let exercises = load_or_die(root);
    let progress  = Progress::load();
    ui::print_list(&exercises, &progress);
}

pub fn reset() {
    Progress::default().save();
    println!("  Progress reset.");
}

// ─── Core exercise loop ───────────────────────────────────────────────────────

/// Returns true = move to next exercise, false = user quit.
fn run_exercise(ex: &Exercise, all: &[Exercise], progress: &mut Progress) -> bool {
    redraw(ex, all, progress, None);

    let compile_ok = compile(ex);
    redraw(ex, all, progress, Some(compile_ok));

    if !compile_ok {
        println!("\n  Fix the compilation errors (see the flags in STEPS),");
        println!("  then re-run `horizon watch`.\n");
        println!("  {}", "─".repeat(50));
        println!("  Press Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
        return true;
    }

    terminal::enable_raw_mode().ok();
    let result = interactive_loop(ex, progress);
    terminal::disable_raw_mode().ok();

    match result {
        LoopResult::Quit => false,
        LoopResult::Done => true,
        LoopResult::Skip => {
            progress.mark_complete(ex.meta.number);
            progress.save();
            true
        }
    }
}

enum LoopResult { Done, Skip, Quit }

fn interactive_loop(ex: &Exercise, progress: &mut Progress) -> LoopResult {
    loop {
        match event::poll(Duration::from_secs(3)) {
            Err(_) => return LoopResult::Quit,

            Ok(true) => {
                let Ok(Event::Key(key)) = event::read() else { continue };

                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    terminal::disable_raw_mode().ok();
                    ui::print_goodbye();
                    return LoopResult::Quit;
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        terminal::disable_raw_mode().ok();
                        ui::print_goodbye();
                        return LoopResult::Quit;
                    }

                    KeyCode::Char('h') => {
                        terminal::disable_raw_mode().ok();
                        give_hint(ex, progress);
                        terminal::enable_raw_mode().ok();
                    }

                    KeyCode::Char('n') => {
                        terminal::disable_raw_mode().ok();
                        ui::print_skipped(&ex.meta.name);
                        thread::sleep(Duration::from_millis(800));
                        return LoopResult::Skip;
                    }

                    KeyCode::Char('v') => {
                        terminal::disable_raw_mode().ok();
                        ui::print_verifying();
                        if verify(ex) {
                            progress.mark_complete(ex.meta.number);
                            progress.save();
                            ui::print_success(&ex.meta.name);
                            thread::sleep(Duration::from_secs(3));
                            return LoopResult::Done;
                        } else {
                            ui::print_verify_fail(ex.is_auto());
                            terminal::enable_raw_mode().ok();
                        }
                    }

                    _ => {}
                }
            }

            Ok(false) => {
                // 3-second tick: auto-verify
                if ex.is_auto() {
                    terminal::disable_raw_mode().ok();
                    ui::print_verifying();
                    if verify(ex) {
                        progress.mark_complete(ex.meta.number);
                        progress.save();
                        ui::print_success(&ex.meta.name);
                        thread::sleep(Duration::from_secs(3));
                        return LoopResult::Done;
                    } else {
                        ui::print_still_waiting();
                        terminal::enable_raw_mode().ok();
                    }
                }
            }
        }
    }
}

// ─── Compilation ─────────────────────────────────────────────────────────────

pub fn compile(ex: &Exercise) -> bool {
    let mut cmd = Cmd::new("gcc");
    for flag in &ex.compile.flags {
        cmd.arg(flag);
    }
    cmd.arg("-o")
        .arg(ex.binary_path())
        .arg(ex.source_path());

    match cmd.status() {
        Ok(s)  => s.success(),
        Err(e) => { eprintln!("  gcc not found: {}", e); false }
    }
}

// ─── Verification ─────────────────────────────────────────────────────────────

pub fn verify(ex: &Exercise) -> bool {
    let script = ex.verify_script();
    if !script.exists() {
        return true; // manual exercise: trust user's [v]
    }

    let _ = Cmd::new("chmod").arg("+x").arg(&script).status();

    let output = Cmd::new("bash")
        .arg(&script)
        .current_dir(&ex.dir)
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            stdout.contains("[SUCCESS]") || stderr.contains("[SUCCESS]")
        }
        Err(_) => false,
    }
}

// ─── Hints ───────────────────────────────────────────────────────────────────

fn give_hint(ex: &Exercise, progress: &mut Progress) {
    match progress.next_hint_index(ex.meta.number, ex.hints.len()) {
        Some(idx) => ui::print_hint(idx + 1, ex.hints.len(), &ex.hints[idx].text),
        None       => ui::print_no_more_hints(),
    }
    progress.save();
    let _ = io::stdin().read_line(&mut String::new());
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn load_or_die(root: &Path) -> Vec<Exercise> {
    match exercise::load_all(root) {
        Ok(v)  => v,
        Err(e) => {
            eprintln!("\n  Error: {}", e);
            eprintln!("  Run horizon from the project root (where 'exercises/' lives).\n");
            std::process::exit(1);
        }
    }
}

fn redraw(ex: &Exercise, all: &[Exercise], progress: &Progress, compile_ok: Option<bool>) {
    ui::clear();
    ui::print_header();
    ui::print_exercise(ex, progress, all.len());
    if let Some(ok) = compile_ok {
        ui::print_compile_status(ex, ok);
        if ok {
            ui::print_footer(ex.is_auto());
        }
    }
}
