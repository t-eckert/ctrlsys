use sqlx::PgPool;
use std::time::Duration;
use tokio::time;

use lib::services::timer::TimerService;

/// Background task that checks for expired timers every second
pub async fn timer_expiration_checker(pool: PgPool) {
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        match TimerService::complete_expired_timers(&pool).await {
            Ok(completed_timers) => {
                if !completed_timers.is_empty() {
                    tracing::info!("Completed {} expired timer(s)", completed_timers.len());
                    for timer in completed_timers {
                        tracing::debug!("Timer '{}' (id: {}) completed", timer.name, timer.id);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Error checking expired timers: {:?}", e);
            }
        }
    }
}
