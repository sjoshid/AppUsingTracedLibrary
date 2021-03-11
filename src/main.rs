mod subscriber;

use TracedLibrary::add::{add, fibonacci};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use subscriber::druid_layer;

fn main() {
	let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
		.with_service_name("trace_demo_2")
		.install().expect("Error initializing Jaeger exporter");
	let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
	let druid_layer = druid_layer::DruidLayer;

	let subscriber = Registry::default().with(telemetry)
		.with(druid_layer);
	tracing::subscriber::set_global_default(subscriber);

	let root = tracing::info_span!("Fibonacci");
	let mut enter = root.enter();
	let fibonacci = fibonacci(7);
	println!("{}", fibonacci);
	std::mem::drop(enter);

	let root = tracing::info_span!("Add");
	let mut enter = root.enter();
	let sum = add(1, 2);
	println!("{}", sum);
	std::mem::drop(enter);

}