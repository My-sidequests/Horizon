use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Progress {
    pub current:     usize,
    pub completed:   HashSet<usize>,
    pub hints_shown: HashMap<usize, usize>,
}

impl Progress {
    pub fn load() -> Self {
        match fs::read_to_string(Self::path()) {
            Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, s);
        }
    }

    pub fn is_done(&self, idx: usize) -> bool {
        self.completed.contains(&idx)
    }

    /// Reveal and return the next unseen hint index, or None if all seen.
    pub fn next_hint_index(&mut self, exercise_idx: usize, total_hints: usize) -> Option<usize> {
        let shown = self.hints_shown.entry(exercise_idx).or_insert(0);
        if *shown < total_hints {
            let idx = *shown;
            *shown += 1;
            Some(idx)
        } else {
            None
        }
    }

    pub fn hints_seen(&self, exercise_idx: usize) -> usize {
        *self.hints_shown.get(&exercise_idx).unwrap_or(&0)
    }

    pub fn data_dir() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("horizon")
    }

    pub fn exercises_dir() -> PathBuf {
        Self::data_dir().join("exercises")
    }

    fn path() -> PathBuf {
        Self::data_dir().join("progress.json")
    }
}
