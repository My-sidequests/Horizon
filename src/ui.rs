use colored::Colorize;

use crate::exercises::Exercise;
use crate::progress::Progress;

const W: usize = 54;
const LINE: &str = "─";

pub fn header() {
    println!();
    println!("  {}", "H O R I Z O N".bold().bright_cyan());
    println!("  {}", LINE.repeat(W).dimmed());
}

pub fn show_exercise(idx: usize, ex: &Exercise, progress: &Progress, total: usize) {
    header();

    let completed = progress.completed.len();

    // Title row
    println!(
        "  {}  {}",
        format!("[{:02}/{:02}]", idx + 1, total).bold().bright_white(),
        ex.title.bold().bright_yellow(),
    );
    println!("  {}", LINE.repeat(W).dimmed());
    println!();

    // Brief
    for line in ex.readme.lines() {
        println!("  {}", line);
    }
    println!();

    // Paths
    let ex_dir = Progress::exercises_dir().join(ex.slug);
    let binary  = ex_dir.join("target");
    let solution = ex_dir.join("solution");

    println!("  {}", "Binary:".bright_white().bold());
    println!("  {}  {}", "→".dimmed(), binary.display().to_string().bright_cyan());
    println!();
    println!("  {}", "Solution:".bright_white().bold());
    println!("  {}  {}", "→".dimmed(), solution.display().to_string().dimmed());
    println!();

    // Hint count
    let seen  = progress.hints_seen(idx);
    let total_hints = ex.hints.lines().filter(|l| !l.trim().is_empty()).count();
    if total_hints > 0 {
        println!(
            "  {}  hints seen: {}/{}",
            "·".dimmed(),
            seen,
            total_hints,
        );
    }

    // Progress bar
    let bar_w  = 24usize;
    let filled = if total > 0 { completed * bar_w / total } else { 0 };
    let bar = format!(
        "{}{}",
        "█".repeat(filled).bright_cyan(),
        "░".repeat(bar_w - filled).dimmed(),
    );
    println!("  {} [{bar}]  {}/{}", "progress".dimmed(), completed, total);
    println!();

    // Controls
    println!(
        "  {}  {}    {}  {}    {}  {}    {}  {}",
        "horizon done".bright_cyan().bold(), "solved".dimmed(),
        "horizon hint".bright_cyan().bold(), "hint".dimmed(),
        "horizon skip".bright_cyan().bold(), "skip".dimmed(),
        "horizon list".bright_cyan().bold(), "list".dimmed(),
    );
    println!();
}

pub fn show_hint(n: usize, total: usize, text: &str) {
    println!();
    println!("  {}  Hint {}/{}", "→".bright_yellow().bold(), n, total);
    println!("  {}", LINE.repeat(W).dimmed());
    println!("  {}", text.italic());
    println!();
}

pub fn no_more_hints() {
    println!();
    println!("  {}  No more hints. The solution file is always there when you need it.", "→".yellow());
    println!();
}


pub fn skipped(title: &str) {
    println!();
    println!("  {}  {} — skipped.", "→".yellow(), title.dimmed());
}

pub fn show_list(exercises: &[&Exercise], progress: &Progress) {
    header();
    println!("  {}", "EXERCISES".bold().bright_white());
    println!("  {}", LINE.repeat(W).dimmed());
    println!();
    for (i, ex) in exercises.iter().enumerate() {
        let icon = if progress.is_done(i) {
            "✓".bright_green().to_string()
        } else if i == progress.current {
            "▶".bright_cyan().to_string()
        } else {
            "○".dimmed().to_string()
        };
        let name = if progress.is_done(i) {
            ex.title.dimmed().to_string()
        } else {
            ex.title.normal().to_string()
        };
        println!("  {}  {:02}.  {}", icon, i + 1, name);
    }
    println!();
    println!(
        "  {}/{} completed",
        progress.completed.len().to_string().bright_cyan(),
        exercises.len()
    );
    println!();
}

pub fn all_done(total: usize) {
    header();
    println!();
    println!("  {}  Module complete — {}/{}", "★".bright_yellow(), total, total);
    println!();
    println!("  {}", "Binary Mastery: Memory Corruption".bold());
    println!("  {}", "Stack overflows, shellcode, off-by-one, format strings, ROP.".dimmed());
    println!("  {}", "You've touched them all.".dimmed());
    println!();
    println!("  {}", "More modules coming. Stay sharp.".dimmed());
    println!();
}

pub fn not_started() {
    println!();
    println!("  {}  Run {} to compile the exercises and begin.", "!".bright_yellow(), "horizon start".bright_cyan().bold());
    println!();
}

pub fn compile_progress(slug: &str, ok: bool) {
    if ok {
        println!("  {}  {}", "✓".bright_green(), slug);
    } else {
        println!("  {}  {} — compilation failed", "✗".bright_red(), slug);
    }
}

pub fn start_done(dir: &str) {
    println!();
    println!("  {}  All exercises compiled.", "✓".bright_green().bold());
    println!("  {}  Binaries are in: {}", "→".dimmed(), dir.bright_cyan());
    println!();
    println!("  Run {} to see your first exercise.", "horizon".bright_cyan().bold());
    println!();
}

pub fn update_start() {
    println!();
    println!("  {}  Pulling latest version from GitHub...", "→".bright_cyan());
    println!();
}

pub fn update_done() {
    println!();
    println!("  {}  Updated. Run {} to recompile the exercises.", "✓".bright_green().bold(), "horizon start".bright_cyan().bold());
    println!();
}

pub fn update_failed(reason: &str) {
    println!();
    println!("  {}  Update failed: {}", "✗".bright_red(), reason);
    println!();
}

pub fn wrong_flag(submitted: &str) {
    println!();
    println!("  {}  '{}' is not the right flag.", "✗".bright_red().bold(), submitted);
    println!("  Keep digging — or run {} for the next hint.", "horizon hint".bright_cyan().bold());
    println!();
}

pub fn correct_flag(title: &str) {
    println!();
    println!("  {}  Correct flag!", "✓".bright_green().bold());
    println!("  {}  {} — solved.", "★".bright_yellow(), title.bold());
}
