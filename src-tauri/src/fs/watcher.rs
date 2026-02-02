use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

pub struct FileWatcher {
    _watcher: RecommendedWatcher,
    rx: mpsc::Receiver<FileEvent>,
}

#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed(PathBuf, PathBuf),
}

impl FileWatcher {
    pub fn new(workspace_path: PathBuf) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::channel(100);

        let watcher = {
            let workspace = workspace_path.clone();

            let mut watcher =
                notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
                    Ok(event) => {
                        let events = Self::convert_event(&workspace, event);
                        for evt in events {
                            let _ = tx.try_send(evt);
                        }
                    }
                    Err(e) => {
                        error!("Watch error: {}", e);
                    }
                })?;

            watcher.watch(&workspace_path, RecursiveMode::Recursive)?;
            watcher
        };

        info!("File watcher started for: {:?}", workspace_path);

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }

    fn convert_event(_workspace: &PathBuf, event: Event) -> Vec<FileEvent> {
        let mut results = Vec::new();

        // Filter for markdown files
        let paths: Vec<_> = event
            .paths
            .into_iter()
            .filter(|p| {
                p.extension()
                    .map(|e| e == "md" || e == "json")
                    .unwrap_or(false)
            })
            .collect();

        if paths.is_empty() {
            return results;
        }

        match event.kind {
            notify::EventKind::Create(_) => {
                for path in paths {
                    results.push(FileEvent::Created(path));
                }
            }
            notify::EventKind::Modify(_) => {
                for path in paths {
                    results.push(FileEvent::Modified(path));
                }
            }
            notify::EventKind::Remove(_) => {
                for path in paths {
                    results.push(FileEvent::Deleted(path));
                }
            }
            notify::EventKind::Any => {
                // Unknown event type, treat as modified
                for path in paths {
                    results.push(FileEvent::Modified(path));
                }
            }
            _ => {}
        }

        results
    }

    pub async fn next_event(&mut self) -> Option<FileEvent> {
        self.rx.recv().await
    }
}

/// Debounced file event handler
pub struct DebouncedWatcher {
    watcher: FileWatcher,
    debounce_duration: Duration,
}

impl DebouncedWatcher {
    pub fn new(workspace_path: PathBuf, debounce_ms: u64) -> anyhow::Result<Self> {
        let watcher = FileWatcher::new(workspace_path)?;

        Ok(Self {
            watcher,
            debounce_duration: Duration::from_millis(debounce_ms),
        })
    }

    pub async fn run<F>(mut self, mut callback: F)
    where
        F: FnMut(FileEvent) + Send + 'static,
    {
        let mut pending_events: Vec<FileEvent> = Vec::new();
        let mut last_event_time = tokio::time::Instant::now();

        loop {
            tokio::select! {
                Some(event) = self.watcher.next_event() => {
                    pending_events.push(event);
                    last_event_time = tokio::time::Instant::now();
                }
                _ = tokio::time::sleep(self.debounce_duration) => {
                    if !pending_events.is_empty() &&
                       last_event_time.elapsed() >= self.debounce_duration {
                        // Process pending events
                        for event in pending_events.drain(..) {
                            callback(event);
                        }
                    }
                }
            }
        }
    }
}
