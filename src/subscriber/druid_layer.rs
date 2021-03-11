use tracing_subscriber::Layer;
use tracing::subscriber::Interest;
use tracing::{Metadata, Subscriber, span};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;

pub struct DruidLayer;

impl<S> Layer<S> for DruidLayer where S: Subscriber + for<'lookup> LookupSpan<'lookup> {
	fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest {
		Interest::sometimes()
	}

	fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool {
		/*if let Some(span_ref) = ctx.lookup_current() {
			let span_id = span_ref.
		}*/
		println!("in enabled()");
		true
	}

	fn new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
		println!("in new span");
	}
}