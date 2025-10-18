pub mod config;
pub mod location;
pub mod nomenclator;
pub mod slug;
pub mod uuid;

// Conditionally compile modules for server or CLI features
#[cfg(feature = "server")]
pub mod controllers;
#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod models;
#[cfg(feature = "server")]
pub mod services;
#[cfg(feature = "server")]
pub mod ws;
