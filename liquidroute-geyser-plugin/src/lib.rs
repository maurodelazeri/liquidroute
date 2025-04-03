pub mod config;
pub mod plugin;
pub mod version;

// Add file-based debugging
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

static INIT_LOGGER: Once = Once::new();
static DEBUG_LOG_MUTEX: Mutex<()> = Mutex::new(());
static mut DEBUG_ENABLED: bool = true;
static mut DEBUG_LOG_FILE: Option<String> = None;

pub fn init_debug_log() {
    INIT_LOGGER.call_once(|| {
        // Try to create the default log directory if it doesn't exist
        let _ = std::fs::create_dir_all("/tmp");
        
        // Set the debug log file location - first try to use environment variable
        let debug_file = std::env::var("LIQUIDROUTE_DEBUG_LOG")
            .unwrap_or_else(|_| "/tmp/liquidroute_debug.log".to_string());
        
        // Safety: This is only set once during initialization and then only read
        unsafe { DEBUG_LOG_FILE = Some(debug_file); }
        
        let message = "LiquidRoute plugin debug log initialized\n";
        let _ = debug_log_to_file(message);
    });
}

pub fn debug_log_to_file(message: &str) -> std::io::Result<()> {
    // Safety: DEBUG_ENABLED is only read here after initialization
    if unsafe { !DEBUG_ENABLED } {
        return Ok(());
    }
    
    // Use a mutex to prevent concurrent writes from corrupting the log file
    let _lock = match DEBUG_LOG_MUTEX.try_lock() {
        Ok(lock) => lock,
        Err(_) => {
            // If we can't get the lock immediately, just skip this log message
            // rather than waiting and potentially deadlocking
            return Ok(());
        }
    };
    
    // Safety: This creates a raw constant pointer instead of a shared reference
    // which avoids the UB of having a shared reference to a mutable static
    let debug_file = unsafe {
        if let Some(file) = &*std::ptr::addr_of!(DEBUG_LOG_FILE) {
            file.as_str()
        } else {
            "/tmp/liquidroute_debug.log" // Fallback
        }
    };
    
    // Use a closure to handle errors gracefully
    let result = (|| -> std::io::Result<()> {
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
    })();
    
    // If we can't write to the log file, disable future logging to avoid
    // repeatedly trying to write to a broken log file
    if result.is_err() {
        // Safety: We're only setting this once if there's an error
        unsafe { DEBUG_ENABLED = false; }
    }
    
    result
}

pub fn get_thread_name() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};

    static ATOMIC_ID: AtomicU64 = AtomicU64::new(0);
    let id = ATOMIC_ID.fetch_add(1, Ordering::Relaxed);
    format!("liquidRouteGeyser{id:02}")
}

/// Allow enabling/disabling debug logging
pub fn set_debug_logging(enabled: bool) {
    // Safety: Only called in controlled places
    unsafe { DEBUG_ENABLED = enabled; }
    let status = if enabled { "enabled" } else { "disabled" };
    if enabled {
        let _ = debug_log_to_file(&format!("Debug logging {}", status));
    }
}