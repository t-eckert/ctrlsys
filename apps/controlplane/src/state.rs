use crate::models::Timer;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state for managing timers
#[derive(Clone)]
pub struct AppState {
    /// In-memory storage for timers (keyed by timer ID)
    pub timers: Arc<RwLock<HashMap<String, Timer>>>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            timers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a timer to the state
    pub async fn add_timer(&self, timer: Timer) {
        let mut timers = self.timers.write().await;
        timers.insert(timer.id.clone(), timer);
    }

    /// Get a timer by ID
    pub async fn get_timer(&self, id: &str) -> Option<Timer> {
        let timers = self.timers.read().await;
        timers.get(id).cloned()
    }

    /// Get all timers
    pub async fn get_all_timers(&self) -> Vec<Timer> {
        let timers = self.timers.read().await;
        timers.values().cloned().collect()
    }

    /// Update a timer
    pub async fn update_timer(&self, timer: Timer) -> bool {
        let mut timers = self.timers.write().await;
        if timers.contains_key(&timer.id) {
            timers.insert(timer.id.clone(), timer);
            true
        } else {
            false
        }
    }

    /// Delete a timer by ID
    pub async fn delete_timer(&self, id: &str) -> bool {
        let mut timers = self.timers.write().await;
        timers.remove(id).is_some()
    }

    /// Check if a timer exists
    pub async fn timer_exists(&self, id: &str) -> bool {
        let timers = self.timers.read().await;
        timers.contains_key(id)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}