use tracing::{dispatcher::set_global_default, Dispatch};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// compose multiple layers into a `tracing`'s subscriber
pub fn get_subscriber<Sink>(name: String, env_filter: String, sink: Sink) -> Dispatch
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // the `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .into()
}

/// register a subscriber as global default to process span data
pub fn init_subscriber(subscriber: Dispatch) {
    // redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to init LogTracer");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
