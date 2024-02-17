use prometheus::{Encoder, TextEncoder};

pub(crate) async fn handler() -> Result<impl warp::Reply, warp::Rejection> {
  let encoder = TextEncoder::new();
  let metric_families = prometheus::gather();
  let mut buffer = Vec::new();
  encoder.encode(&metric_families, &mut buffer).unwrap();
  Ok(warp::reply::with_header(buffer, "Content-Type", encoder.format_type()))
}
