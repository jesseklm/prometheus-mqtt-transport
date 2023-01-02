use flate2::bufread::GzDecoder;
use log::{debug, error, info};
use simple_error::bail;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::prelude::*;
use std::string::String;
use std::sync::mpsc;

pub enum Data {
    HTTPRequest,
    MetricData(Vec<global::payload::Message>),
}

fn build_reply_string(metrics: &HashMap<String, global::payload::Message>) -> String {
    // HashSets hold *basenames* of histogram and summaries
    let mut histogram: HashSet<String> = HashSet::new();
    let mut summary: HashSet<String> = HashSet::new();
    let mut result: Vec<String> = Vec::new();
    // HELP and TYPE strings *must* occur only once!
    let mut metrics_type: HashMap<String, String> = HashMap::new();
    let mut metrics_help: HashMap<String, String> = HashMap::new();
    let mut metrics_data: HashMap<String, Vec<String>> = HashMap::new();
    let parse_time = std::time::Instant::now();

    debug!("scanning for histogram and summary metrics");
    // get <basename> of histogram and summary metrics
    for name in metrics.keys() {
        if let Some(mdata) = metrics.get(name) {
            for mtrc in mdata.payload.iter() {
                debug!("checking TYPE for '{:?}'", mtrc);
                match mtrc.data_type.as_str() {
                    "histogram" => {
                        /*
                         * "A histogram with a base metric name of <basename> exposes multiple time series during a scrape:
                         *  cumulative counters for the observation buckets, exposed as <basename>_bucket{le="<upper inclusive bound>"}
                         *  the total sum of all observed values, exposed as <basename>_sum
                         *  the count of events that have been observed, exposed as <basename>_count (identical to <basename>_bucket{le="+Inf"} above)"
                         *
                         *  see: https://prometheus.io/docs/concepts/metric_types/#histogram
                         *
                         *  e.g.:
                         *
                         *   # HELP bind_resolver_query_duration_seconds Resolver query round-trip time in seconds.
                         *   # TYPE bind_resolver_query_duration_seconds histogram
                         *   bind_resolver_query_duration_seconds_bucket{view="_bind",le="0.01"} 0
                         *   bind_resolver_query_duration_seconds_bucket{view="_bind",le="0.1"} 0
                         *   bind_resolver_query_duration_seconds_bucket{view="_bind",le="0.5"} 0
                         *   bind_resolver_query_duration_seconds_bucket{view="_bind",le="0.8"} 0
                         *   bind_resolver_query_duration_seconds_bucket{view="_bind",le="1.6"} 0
                         *   bind_resolver_query_duration_seconds_bucket{view="_bind",le="+Inf"} 0
                         *   bind_resolver_query_duration_seconds_sum{view="_bind"} NaN
                         *   bind_resolver_query_duration_seconds_count{view="_bind"} 0
                         *   bind_resolver_query_duration_seconds_bucket{view="_default",le="0.01"} 109879
                         *   bind_resolver_query_duration_seconds_bucket{view="_default",le="0.1"} 601436
                         *   bind_resolver_query_duration_seconds_bucket{view="_default",le="0.5"} 774852
                         *   bind_resolver_query_duration_seconds_bucket{view="_default",le="0.8"} 775299
                         *   bind_resolver_query_duration_seconds_bucket{view="_default",le="1.6"} 775323
                         *   bind_resolver_query_duration_seconds_bucket{view="_default",le="+Inf"} 775365
                         *   bind_resolver_query_duration_seconds_sum{view="_default"} NaN
                         *   bind_resolver_query_duration_seconds_count{view="_default"} 775365
                         *
                         */
                        histogram.insert(mtrc.metric_name.clone());
                    }
                    "summary" => {
                        /*
                         * "A summary with a base metric name of <basename> exposes multiple time series during a scrape:
                         *  streaming φ-quantiles (0 ≤ φ ≤ 1) of observed events, exposed as <basename>{quantile="<φ>"}
                         *  the total sum of all observed values, exposed as <basename>_sum
                         *  the count of events that have been observed, exposed as <basename>_count"
                         *
                         * see: https://prometheus.io/docs/concepts/metric_types/#summary
                         *
                         * e.g.:
                         *
                         *  # HELP go_gc_duration_seconds A summary of the pause duration of garbage collection cycles.
                         *  # TYPE go_gc_duration_seconds summary
                         *  go_gc_duration_seconds{quantile="0"} 2.499e-05
                         *  go_gc_duration_seconds{quantile="0.25"} 6.8457e-05
                         *  go_gc_duration_seconds{quantile="0.5"} 8.2795e-05
                         *  go_gc_duration_seconds{quantile="0.75"} 0.000126954
                         *  go_gc_duration_seconds{quantile="1"} 0.000683124
                         *  go_gc_duration_seconds_sum 5.7718011449999995
                         *  go_gc_duration_seconds_count 44174
                         *
                         */
                        summary.insert(mtrc.metric_name.clone());
                    }
                    _ => {}
                };
            }
        }
    }
    debug!("found {} histogram types: {:?}", histogram.len(), histogram);
    debug!("found {} summary types: {:?}", summary.len(), summary);

    // collect TYPE and HELP texts
    for name in metrics.keys() {
        if let Some(mdata) = metrics.get(name) {
            for mtrc in mdata.payload.iter() {
                metrics_type.insert(mtrc.metric_name.to_string(), mtrc.data_type.clone());
                metrics_help.insert(mtrc.metric_name.to_string(), mtrc.help.clone());
                let collected_data = metrics_data
                    .entry(mtrc.metric_name.to_string())
                    .or_default();
                collected_data.extend(mtrc.data.clone());
                debug!(
                    "collected type ({}), help ({}) and data ({}) for '{}'",
                    mtrc.data_type.clone(),
                    mtrc.help.clone(),
                    mtrc.data.clone().len(),
                    mtrc.metric_name
                );
            }
        }
    }

    // Build results for non-histogram and non-summary data
    for m in metrics_type.keys() {
        if let Some(mtype) = metrics_type.get(m) {
            if let Some(mhelp) = metrics_help.get(m) {
                if let Some(mdata) = metrics_data.get(m) {
                    // skip histogram and summary data, will be collected later
                    if mtype != "histogram"
                        && mtype != "summary"
                        && !(mtype.is_empty() && mhelp.is_empty())
                    {
                        debug!(
                            "'{}' - TYPE: {} / HELP: {} / len(data) = {}",
                            m,
                            mtype,
                            mhelp,
                            mdata.len()
                        );
                        result.push(format!(
                            "# TYPE {} {}\n{}\n# HELP {} {}",
                            m,
                            mtype,
                            mdata.join("\n"),
                            m,
                            mhelp,
                        ));
                    }
                }
            } else {
                error!("key {} found in metrics_type map but not in metrics_help, invalid metric format?", m);
            }
        }
    }

    // build reply for histogram and summary types
    for m in histogram.iter() {
        if let Some(mtype) = metrics_type.get(m) {
            if let Some(mhelp) = metrics_help.get(m) {
                debug!("adding metric data for histogram type {}", m);
                if let Err(e) = append_histogram_result(&metrics_data, m, mhelp, mtype, &mut result)
                {
                    error!("an error occured while processing histogram data - {}", e);
                }
            }
        }
    }

    for m in summary.iter() {
        if let Some(mtype) = metrics_type.get(m) {
            if let Some(mhelp) = metrics_help.get(m) {
                debug!("adding metric data for histogram type {}", m);
                if let Err(e) = append_summary_result(&metrics_data, m, mhelp, mtype, &mut result) {
                    error!("an error occured while processing summary data - {}", e);
                }
            }
        }
    }

    // enforce final new line otherwise promtool will complain ("unexpected end of input stream")
    result.push(String::new());
    info!(
        "metrics processed in {} seconds",
        parse_time.elapsed().as_secs_f64()
    );
    result.join("\n")
}

fn append_summary_result(
    data: &HashMap<String, Vec<String>>,
    name: &str,
    help: &str,
    mtype: &str,
    result: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    // "summary" type requires "<basename>", "_count" and "_sum" data
    let count = format!("{}_count", name);
    let sum = format!("{}_sum", name);

    let base_data = match data.get(name) {
        Some(v) => v,
        None => bail!(
            "{} is of type summary but nor data found for {}",
            name,
            name
        ),
    };

    let count_data = match data.get(&count) {
        Some(v) => v,
        None => bail!(
            "{} is of type summary but nor data found for {}",
            name,
            count
        ),
    };

    let sum_data = match data.get(&sum) {
        Some(v) => v,
        None => bail!(
            "{} is of type summary but nor data found for {}",
            name,
            count
        ),
    };

    result.push(format!("# TYPE {} {}", name, mtype));
    result.push(base_data.join("\n"));
    result.push(count_data.join("\n"));
    result.push(sum_data.join("\n"));
    result.push(format!("# HELP {} {}", name, help));

    Ok(())
}

fn append_histogram_result(
    data: &HashMap<String, Vec<String>>,
    name: &str,
    help: &str,
    mtype: &str,
    result: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    // "histogram" type requires "_bucket", "_count" and "_sum" data
    let bucket = format!("{}_bucket", name);
    let count = format!("{}_count", name);
    let sum = format!("{}_sum", name);

    let bucket_data = match data.get(&bucket) {
        Some(v) => v,
        None => bail!(
            "{} is of type histogram but nor data found for {}",
            name,
            bucket
        ),
    };

    let count_data = match data.get(&count) {
        Some(v) => v,
        None => bail!(
            "{} is of type histogram but nor data found for {}",
            name,
            count
        ),
    };

    let sum_data = match data.get(&sum) {
        Some(v) => v,
        None => bail!(
            "{} is of type histogram but nor data found for {}",
            name,
            count
        ),
    };

    result.push(format!("# TYPE {} {}", name, mtype));
    result.push(bucket_data.join("\n"));
    result.push(count_data.join("\n"));
    result.push(sum_data.join("\n"));
    result.push(format!("# HELP {} {}", name, help));

    Ok(())
}

pub fn handler(
    data_receiver: mpsc::Receiver<Data>,
    http_reply: mpsc::Sender<String>,
) -> Result<(), Box<dyn Error>> {
    let mut metrics: HashMap<String, global::payload::Message> = HashMap::new();
    let mut metrics_expiration: HashMap<String, i64> = HashMap::new();
    let mut now: i64;

    loop {
        let request = data_receiver.recv()?;
        now = chrono::Local::now().timestamp();
        match request {
            Data::HTTPRequest => {
                debug!("HTTP request received");
                debug!("purging expired data");
                purge_expired(&mut metrics, &mut metrics_expiration, now);

                let reply = build_reply_string(&metrics);
                http_reply.send(reply)?;
            }
            Data::MetricData(msg) => {
                debug!("{} metric messages received", msg.len());
                for m in msg {
                    let mname = m.name.clone();
                    metrics.insert(m.name.clone(), m);
                    metrics_expiration.insert(mname, now);
                }
            }
        };
    }
}

pub fn parse_raw_metrics(raw: Vec<u8>) -> Result<Vec<global::payload::Message>, Box<dyn Error>> {
    if raw.len() < 2 {
        bail!("received payload is too short");
    }

    let prc = std::time::Instant::now();

    let data_str = if raw[0] == 0x1f && raw[1] == 0x8b {
        let dcomp = std::time::Instant::now();

        info!("decompressing gzip compressed data");
        let mut gzd = GzDecoder::new(&raw[..]);
        let mut decompressed = String::new();
        gzd.read_to_string(&mut decompressed)?;
        info!(
            "data decompressed {} bytes -> {} bytes in {} seconds",
            raw.len(),
            decompressed.len(),
            dcomp.elapsed().as_secs_f64()
        );

        decompressed
    } else {
        String::from_utf8(raw)?
    };

    let parsed = serde_json::from_str(&data_str)?;
    info!("payload parsed in {} seconrs", prc.elapsed().as_secs_f64());

    Ok(parsed)
}

fn purge_expired(
    metrics: &mut HashMap<String, global::payload::Message>,
    metrics_expiration: &mut HashMap<String, i64>,
    now: i64,
) {
    let mut expired: Vec<String> = Vec::new();

    for name in metrics.keys() {
        if let Some(data) = metrics.get(name) {
            if let Some(last_update) = metrics_expiration.get(name) {
                if now - last_update >= data.expiration {
                    info!(
                        "{} expired {} seconds ago, removing metrics",
                        name,
                        now - last_update
                    );
                    debug!("'{}' was last updated {} - {} seconds ago, expiration set to {}, adding to removal list", name, last_update, now - last_update, data.expiration);
                    expired.push(name.to_string());
                }
            } else {
                // XXX: Should never happen
                panic!(
                    "BUG: key {} found in metrics map but not in metrics_expiration",
                    name
                );
            }
        }
    }

    for exp in expired.iter() {
        debug!("removing expired data for {} from HashMaps", exp);
        metrics.remove(exp);
        metrics_expiration.remove(exp);
    }
}
