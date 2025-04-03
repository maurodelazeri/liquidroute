pub mod config;
pub mod plugin;
pub mod version;

// Add file-based debugging
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Once;

static INIT_LOGGER: Once = Once::new();
static mut DEBUG_ENABLED: bool = true;

pub fn init_debug_log() {
    INIT_LOGGER.call_once(|| {
        let message = "LiquidRoute plugin debug log initialized\n";
        let _ = debug_log_to_file(message);
    });
}

pub fn debug_log_to_file(message: &str) -> std::io::Result<()> {
    // Safety: This is only used for debugging and the value is only set once at startup
    if unsafe { !DEBUG_ENABLED } {
        return Ok(());
    }
    
    let debug_file = "/tmp/liquidroute_debug.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(debug_file)?;
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    
    let log_message = format!("[{} ms] {}\n", now, message);
    file.write_all(log_message.as_bytes())?;
    file.flush()?;
    
    Ok(())
}

pub fn get_thread_name() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};

    static ATOMIC_ID: AtomicU64 = AtomicU64::new(0);
    let id = ATOMIC_ID.fetch_add(1, Ordering::Relaxed);
    format!("liquidRouteGeyser{id:02}")
}