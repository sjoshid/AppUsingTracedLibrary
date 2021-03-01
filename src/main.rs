use TracedLibrary::add::{add, fibonacci};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::util::SubscriberInitExt;
//use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

fn main() {
	let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
		.with_service_name("trace_demo_2")
		.install().expect("Error initializing Jaeger exporter");
	let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

	Registry::default().with(telemetry).init();
	let sum = add(1, 2);
	println!("{}", sum);
	let fibonacci = fibonacci(7);
	println!("{}", fibonacci);
	//let sum = add::add(3, 2);
}