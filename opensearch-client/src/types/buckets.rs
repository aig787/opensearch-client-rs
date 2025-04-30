use serde::{Deserialize, Serialize};

use super::SubAggregations;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermsBucket {
  pub key: serde_json::Value,
  pub doc_count: u64,
  #[serde(default, skip_serializing_if = "SubAggregations::is_empty")]
  pub aggregations: SubAggregations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeBucket {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub key: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub from: Option<f64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub to: Option<f64>,
  pub doc_count: u64,
  #[serde(default, skip_serializing_if = "SubAggregations::is_empty")]
  pub aggregations: SubAggregations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateRangeBucket {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub key: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub from: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub from_as_string: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub to: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub to_as_string: Option<String>,
  pub doc_count: u64,
  #[serde(default, skip_serializing_if = "SubAggregations::is_empty")]
  pub aggregations: SubAggregations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramBucket {
  pub key: f64,
  pub doc_count: u64,
  #[serde(default, skip_serializing_if = "SubAggregations::is_empty")]
  pub aggregations: SubAggregations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateHistogramBucket {
  pub key: i64,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub key_as_string: Option<String>,
  pub doc_count: u64,
  #[serde(default, skip_serializing_if = "SubAggregations::is_empty")]
  pub aggregations: SubAggregations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoDistanceBucket {
  pub key: String,
  pub from: Option<f64>,
  pub to: Option<f64>,
  pub doc_count: u64,
  #[serde(default, skip_serializing_if = "SubAggregations::is_empty")]
  pub aggregations: SubAggregations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiltersBucket {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub key: Option<String>,
  pub doc_count: u64,
  #[serde(default, skip_serializing_if = "SubAggregations::is_empty")]
  pub aggregations: SubAggregations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixRow {
  pub key: serde_json::Value,
  #[serde(default, skip_serializing_if = "SubAggregations::is_empty")]
  pub aggregations: SubAggregations,
}
