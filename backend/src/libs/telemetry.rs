use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, util::SubscriberInitExt};

use crate::libs::config::Config;

pub struct TelemetryGuard {
    _enabled: bool,
}

pub fn init(config: &Config) -> anyhow::Result<TelemetryGuard> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(false)
        .with_span_list(false)
        .with_target(false)
        .flatten_event(true)
        .with_file(true)
        .with_line_number(true);

    Registry::default().with(env_filter).with(fmt_layer).init();

    if config.telemetry_enabled {
        tracing::info!(
            service = %config.otel_service_name,
            endpoint = ?config.otel_exporter_otlp_endpoint,
            "Sango platform telemetry enabled"
        );
    } else {
        tracing::info!("Sango platform telemetry disabled");
    }

    Ok(TelemetryGuard {
        _enabled: config.telemetry_enabled,
    })
}

pub fn shutdown(_guard: TelemetryGuard) -> anyhow::Result<()> {
    Ok(())
}
