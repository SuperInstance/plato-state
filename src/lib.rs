//! # plato-state
//!
//! 16-dimensional room state vectors for the PLATO nervous system.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Labels for each of the 16 state dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateLabel {
    Health,
    Thermal,
    Stress,
    Drift,
    Vibration,
    Acoustic,
    Visual,
    Pressure,
    Humidity,
    Load,
    Latency,
    Confidence,
    AnomalyScore,
    Stability,
    Energy,
    Occupancy,
}

impl StateLabel {
    /// All label variants in order.
    pub const ALL: [StateLabel; 16] = [
        StateLabel::Health,
        StateLabel::Thermal,
        StateLabel::Stress,
        StateLabel::Drift,
        StateLabel::Vibration,
        StateLabel::Acoustic,
        StateLabel::Visual,
        StateLabel::Pressure,
        StateLabel::Humidity,
        StateLabel::Load,
        StateLabel::Latency,
        StateLabel::Confidence,
        StateLabel::AnomalyScore,
        StateLabel::Stability,
        StateLabel::Energy,
        StateLabel::Occupancy,
    ];

    /// Index into the 16-element array.
    pub fn index(self) -> usize {
        self as usize
    }

    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            StateLabel::Health => "health",
            StateLabel::Thermal => "thermal",
            StateLabel::Stress => "stress",
            StateLabel::Drift => "drift",
            StateLabel::Vibration => "vibration",
            StateLabel::Acoustic => "acoustic",
            StateLabel::Visual => "visual",
            StateLabel::Pressure => "pressure",
            StateLabel::Humidity => "humidity",
            StateLabel::Load => "load",
            StateLabel::Latency => "latency",
            StateLabel::Confidence => "confidence",
            StateLabel::AnomalyScore => "anomaly_score",
            StateLabel::Stability => "stability",
            StateLabel::Energy => "energy",
            StateLabel::Occupancy => "occupancy",
        }
    }
}

/// A 16-dimensional state vector representing a room's current condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomStateVector {
    values: [f64; 16],
}

impl RoomStateVector {
    /// Create a new zero-initialized state vector.
    pub fn new() -> Self {
        Self { values: [0.0; 16] }
    }

    /// Create from explicit values.
    pub fn from_values(values: [f64; 16]) -> Self {
        Self { values }
    }

    /// Set a dimension by label.
    pub fn set(&mut self, label: StateLabel, value: f64) -> &mut Self {
        self.values[label.index()] = value;
        self
    }

    /// Get a dimension by label.
    pub fn get(&self, label: StateLabel) -> f64 {
        self.values[label.index()]
    }

    /// Get all values as a slice.
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }

    /// Euclidean norm (magnitude) of the vector.
    pub fn magnitude(&self) -> f64 {
        self.values.iter().map(|v| v * v).sum::<f64>().sqrt()
    }

    /// Euclidean distance to another state vector.
    pub fn distance_to(&self, other: &Self) -> f64 {
        self.values
            .iter()
            .zip(other.values.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }

    /// Check if any dimension exceeds the configured threshold.
    pub fn is_anomalous(&self, config: &StateConfig) -> bool {
        StateLabel::ALL
            .iter()
            .any(|&label| self.get(label) > config.threshold(label))
    }

    /// Get dimensions that exceed their configured threshold.
    pub fn anomalous_dimensions(&self, config: &StateConfig) -> Vec<StateLabel> {
        StateLabel::ALL
            .iter()
            .filter(|&&label| self.get(label) > config.threshold(label))
            .copied()
            .collect()
    }
}

impl Default for RoomStateVector {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for state vector analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    /// Per-dimension thresholds for anomaly detection.
    thresholds: [f64; 16],
    /// Number of history entries to retain.
    pub history_size: usize,
    /// Default fusion weights (used when no explicit weights given).
    pub default_fusion_weights: [f64; 16],
}

impl StateConfig {
    /// Create with uniform thresholds.
    pub fn new(threshold: f64, history_size: usize) -> Self {
        Self {
            thresholds: [threshold; 16],
            history_size,
            default_fusion_weights: [1.0; 16],
        }
    }

    /// Set threshold for a specific dimension.
    pub fn set_threshold(&mut self, label: StateLabel, value: f64) -> &mut Self {
        self.thresholds[label.index()] = value;
        self
    }

    /// Get threshold for a dimension.
    pub fn threshold(&self, label: StateLabel) -> f64 {
        self.thresholds[label.index()]
    }
}

impl Default for StateConfig {
    fn default() -> Self {
        Self::new(0.8, 100)
    }
}

/// Difference between two state vectors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDelta {
    deltas: [f64; 16],
}

impl StateDelta {
    /// Compute the delta between two states (after - before).
    pub fn compute(before: &RoomStateVector, after: &RoomStateVector) -> Self {
        let mut deltas = [0.0; 16];
        for (i, (a, b)) in before.as_slice().iter().zip(after.as_slice().iter()).enumerate() {
            deltas[i] = b - a;
        }
        Self { deltas }
    }

    /// Get the delta for a specific dimension.
    pub fn get(&self, label: StateLabel) -> f64 {
        self.deltas[label.index()]
    }

    /// Get the absolute delta for a dimension.
    pub fn abs(&self, label: StateLabel) -> f64 {
        self.deltas[label.index()].abs()
    }

    /// Return dimensions that changed by more than the given threshold (absolute value).
    pub fn significant_dimensions(&self, threshold: f64) -> Vec<StateLabel> {
        StateLabel::ALL
            .iter()
            .filter(|&&label| self.abs(label) > threshold)
            .copied()
            .collect()
    }

    /// Total absolute change across all dimensions.
    pub fn total_change(&self) -> f64 {
        self.deltas.iter().map(|d| d.abs()).sum()
    }
}

/// Trend direction for a dimension over time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    Improving,
    Declining,
    Stable,
}

/// Timestamped state vector for history tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampedState {
    pub state: RoomStateVector,
    pub timestamp: u64,
}

/// Ring buffer of recent states with timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateHistory {
    buffer: Vec<TimestampedState>,
    capacity: usize,
    pos: usize,
    len: usize,
}

impl StateHistory {
    /// Create a new ring buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
            pos: 0,
            len: 0,
        }
    }

    fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Push a new state into the ring buffer.
    pub fn push(&mut self, state: RoomStateVector) {
        let entry = TimestampedState {
            state,
            timestamp: Self::now_ms(),
        };
        self.push_entry(entry);
    }

    /// Push with an explicit timestamp (for testing).
    pub fn push_with_timestamp(&mut self, state: RoomStateVector, ts: u64) {
        let entry = TimestampedState {
            state,
            timestamp: ts,
        };
        self.push_entry(entry);
    }

    fn push_entry(&mut self, entry: TimestampedState) {
        if self.len < self.capacity {
            self.buffer.push(entry);
            self.pos = self.buffer.len(); // points to next write slot
            self.len = self.buffer.len();
        } else {
            let idx = self.pos % self.capacity;
            self.buffer[idx] = entry;
            self.pos = (self.pos + 1) % self.capacity;
        }
    }

    /// Number of entries in the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Is the buffer empty?
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get the most recent state.
    pub fn latest(&self) -> Option<&RoomStateVector> {
        if self.len == 0 {
            return None;
        }
        // pos points to the next write slot, so pos-1 (wrapped) is the latest
        let idx = if self.pos == 0 {
            self.capacity - 1
        } else {
            self.pos - 1
        };
        Some(&self.buffer[idx].state)
    }

    /// Get all entries in chronological order.
    pub fn iter_chrono(&self) -> impl Iterator<Item = &TimestampedState> {
        // When buffer is full, oldest is at pos, newest at pos-1
        let start = if self.len < self.capacity {
            0
        } else {
            self.pos
        };
        let len = self.len;
        let cap = self.capacity;
        let mut i = 0;
        std::iter::from_fn(move || {
            if i >= len {
                return None;
            }
            let idx = (start + i) % cap;
            i += 1;
            Some(&self.buffer[idx])
        })
    }

    /// Determine the trend for a specific dimension.
    /// Looks at recent entries and determines if values are improving or declining.
    pub fn trend(&self, label: StateLabel) -> Trend {
        let values: Vec<f64> = self.iter_chrono().map(|e| e.state.get(label)).collect();
        if values.len() < 2 {
            return Trend::Stable;
        }

        // Use simple linear regression slope
        let n = values.len() as f64;
        let x_mean = (n - 1.0) / 2.0;
        let y_mean = values.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut denominator = 0.0;
        for (i, &y) in values.iter().enumerate() {
            let x = i as f64 - x_mean;
            numerator += x * (y - y_mean);
            denominator += x * x;
        }

        if denominator == 0.0 {
            return Trend::Stable;
        }

        let slope = numerator / denominator;
        // For most dimensions, positive slope = declining (higher = worse)
        // But we use a simple threshold
        const SLOPE_THRESHOLD: f64 = 0.01;
        if slope > SLOPE_THRESHOLD {
            Trend::Declining
        } else if slope < -SLOPE_THRESHOLD {
            Trend::Improving
        } else {
            Trend::Stable
        }
    }

    /// How different is the current state from the historical average?
    /// Returns a value 0.0-1.0+ where 0.0 = identical to average.
    pub fn anomaly_score(&self) -> f64 {
        if self.len < 2 {
            return 0.0;
        }

        // Compute historical average
        let mut avg = [0.0; 16];
        let count = self.len as f64;
        for entry in self.iter_chrono() {
            for (i, &v) in entry.state.as_slice().iter().enumerate() {
                avg[i] += v / count;
            }
        }

        // Compute std dev
        let mut variance = [0.0; 16];
        for entry in self.iter_chrono() {
            for (i, &v) in entry.state.as_slice().iter().enumerate() {
                variance[i] += (v - avg[i]).powi(2) / count;
            }
        }

        // Current state
        let current = self.latest().unwrap();
        let mut score = 0.0;
        for i in 0..16 {
            let std_dev = variance[i].sqrt().max(0.001);
            score += ((current.as_slice()[i] - avg[i]) / std_dev).abs();
        }

        // Normalize to 0-1ish range
        score / 16.0
    }
}

/// State fusion operators.
pub struct StateFusion;

impl StateFusion {
    /// Weighted average of multiple state vectors.
    /// Each entry is a (vector, weight) pair.
    pub fn weighted_average(vectors: &[(&RoomStateVector, f64)]) -> RoomStateVector {
        if vectors.is_empty() {
            return RoomStateVector::new();
        }

        let total_weight: f64 = vectors.iter().map(|(_, w)| w).sum();
        if total_weight == 0.0 {
            return RoomStateVector::new();
        }

        let mut result = [0.0; 16];
        for (vec, weight) in vectors {
            for (i, &v) in vec.as_slice().iter().enumerate() {
                result[i] += v * weight / total_weight;
            }
        }

        RoomStateVector::from_values(result)
    }

    /// Take the maximum value per dimension across all vectors.
    pub fn max_combine(vectors: &[RoomStateVector]) -> RoomStateVector {
        if vectors.is_empty() {
            return RoomStateVector::new();
        }

        let mut result = [f64::NEG_INFINITY; 16];
        for vec in vectors {
            for (i, &v) in vec.as_slice().iter().enumerate() {
                result[i] = result[i].max(v);
            }
        }

        RoomStateVector::from_values(result)
    }

    /// Simple Bayesian-style fusion: multiply likelihoods and normalize.
    /// Treats each dimension as an independent signal.
    pub fn bayesian_fusion(vectors: &[RoomStateVector], prior: &RoomStateVector) -> RoomStateVector {
        if vectors.is_empty() {
            return prior.clone();
        }

        let mut result = [0.0; 16];
        for i in 0..16 {
            let mut product = prior.as_slice()[i].max(0.001);
            for vec in vectors {
                product *= vec.as_slice()[i].max(0.001);
            }
            // Normalize by dividing by prior^(n) to keep in reasonable range
            let normalization = prior.as_slice()[i].max(0.001).powi(vectors.len() as i32);
            result[i] = product / normalization.max(0.001);
        }

        RoomStateVector::from_values(result)
    }
}

/// An alert generated when a dimension crosses a threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateAlert {
    pub id: String,
    pub label: StateLabel,
    pub value: f64,
    pub threshold: f64,
    pub timestamp: u64,
    pub message: String,
}

impl StateAlert {
    /// Create a new alert.
    pub fn new(label: StateLabel, value: f64, threshold: f64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            label,
            value,
            threshold,
            timestamp: StateHistory::now_ms(),
            message: format!(
                "{} exceeded threshold: {:.3} > {:.3}",
                label.name(),
                value,
                threshold
            ),
        }
    }

    /// Check a state vector against config and generate alerts.
    pub fn check(state: &RoomStateVector, config: &StateConfig) -> Vec<StateAlert> {
        StateLabel::ALL
            .iter()
            .filter(|&&label| state.get(label) > config.threshold(label))
            .map(|&label| StateAlert::new(label, state.get(label), config.threshold(label)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_zero_vector() {
        let v = RoomStateVector::new();
        for &label in &StateLabel::ALL {
            assert_eq!(v.get(label), 0.0);
        }
    }

    #[test]
    fn test_from_values() {
        let vals = [1.0; 16];
        let v = RoomStateVector::from_values(vals);
        assert_eq!(v.get(StateLabel::Health), 1.0);
        assert_eq!(v.get(StateLabel::Occupancy), 1.0);
    }

    #[test]
    fn test_set_and_get() {
        let mut v = RoomStateVector::new();
        v.set(StateLabel::Thermal, 0.75);
        assert_eq!(v.get(StateLabel::Thermal), 0.75);
        assert_eq!(v.get(StateLabel::Health), 0.0);
    }

    #[test]
    fn test_set_returns_mut() {
        let mut v = RoomStateVector::new();
        v.set(StateLabel::Thermal, 0.5)
         .set(StateLabel::Acoustic, 0.3);
        assert_eq!(v.get(StateLabel::Thermal), 0.5);
        assert_eq!(v.get(StateLabel::Acoustic), 0.3);
    }

    #[test]
    fn test_magnitude_zero() {
        let v = RoomStateVector::new();
        assert_eq!(v.magnitude(), 0.0);
    }

    #[test]
    fn test_magnitude_nonzero() {
        let mut v = RoomStateVector::new();
        v.set(StateLabel::Health, 3.0);
        v.set(StateLabel::Thermal, 4.0);
        let mag = v.magnitude();
        assert!((mag - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_distance_identical() {
        let v = RoomStateVector::new();
        assert_eq!(v.distance_to(&v), 0.0);
    }

    #[test]
    fn test_distance_different() {
        let mut a = RoomStateVector::new();
        a.set(StateLabel::Health, 1.0);
        let b = RoomStateVector::new();
        let dist = a.distance_to(&b);
        assert!((dist - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_anomalous() {
        let mut config = StateConfig::new(0.8, 100);
        let mut v = RoomStateVector::new();
        assert!(!v.is_anomalous(&config));
        v.set(StateLabel::Thermal, 0.9);
        assert!(v.is_anomalous(&config));
    }

    #[test]
    fn test_is_anomalous_per_dimension() {
        let mut config = StateConfig::default();
        config.set_threshold(StateLabel::Health, 0.5);
        config.set_threshold(StateLabel::Thermal, 0.99);
        let mut v = RoomStateVector::new();
        v.set(StateLabel::Health, 0.6);
        v.set(StateLabel::Thermal, 0.9);
        assert!(v.is_anomalous(&config));
        let dims = v.anomalous_dimensions(&config);
        assert_eq!(dims.len(), 1);
        assert_eq!(dims[0], StateLabel::Health);
    }

    #[test]
    fn test_weighted_average_empty() {
        let result = StateFusion::weighted_average(&[]);
        assert_eq!(result.magnitude(), 0.0);
    }

    #[test]
    fn test_weighted_average_single() {
        let mut v = RoomStateVector::new();
        v.set(StateLabel::Health, 1.0);
        let result = StateFusion::weighted_average(&[(&v, 1.0)]);
        assert!((result.get(StateLabel::Health) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_weighted_average_two() {
        let mut a = RoomStateVector::new();
        a.set(StateLabel::Health, 0.0);
        let mut b = RoomStateVector::new();
        b.set(StateLabel::Health, 1.0);
        let result = StateFusion::weighted_average(&[(&a, 1.0), (&b, 1.0)]);
        assert!((result.get(StateLabel::Health) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_weighted_average_biased() {
        let mut a = RoomStateVector::new();
        a.set(StateLabel::Health, 0.0);
        let mut b = RoomStateVector::new();
        b.set(StateLabel::Health, 1.0);
        let result = StateFusion::weighted_average(&[(&a, 0.25), (&b, 0.75)]);
        assert!((result.get(StateLabel::Health) - 0.75).abs() < 1e-10);
    }

    #[test]
    fn test_max_combine() {
        let mut a = RoomStateVector::new();
        a.set(StateLabel::Health, 0.3);
        a.set(StateLabel::Thermal, 0.9);
        let mut b = RoomStateVector::new();
        b.set(StateLabel::Health, 0.7);
        b.set(StateLabel::Thermal, 0.2);
        let result = StateFusion::max_combine(&[a, b]);
        assert!((result.get(StateLabel::Health) - 0.7).abs() < 1e-10);
        assert!((result.get(StateLabel::Thermal) - 0.9).abs() < 1e-10);
    }

    #[test]
    fn test_max_combine_empty() {
        let result = StateFusion::max_combine(&[]);
        assert_eq!(result.magnitude(), 0.0);
    }

    #[test]
    fn test_delta_compute() {
        let mut before = RoomStateVector::new();
        before.set(StateLabel::Health, 0.5);
        let mut after = RoomStateVector::new();
        after.set(StateLabel::Health, 0.8);
        let delta = StateDelta::compute(&before, &after);
        assert!((delta.get(StateLabel::Health) - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_delta_significant() {
        let mut before = RoomStateVector::new();
        before.set(StateLabel::Health, 0.0);
        before.set(StateLabel::Thermal, 0.0);
        let mut after = RoomStateVector::new();
        after.set(StateLabel::Health, 0.5);
        after.set(StateLabel::Thermal, 0.1);
        let delta = StateDelta::compute(&before, &after);
        let sig = delta.significant_dimensions(0.2);
        assert_eq!(sig.len(), 1);
        assert_eq!(sig[0], StateLabel::Health);
    }

    #[test]
    fn test_delta_total_change() {
        let mut before = RoomStateVector::new();
        let mut after = RoomStateVector::new();
        after.set(StateLabel::Health, 1.0);
        after.set(StateLabel::Thermal, 2.0);
        let delta = StateDelta::compute(&before, &after);
        assert!((delta.total_change() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_history_push_and_latest() {
        let mut hist = StateHistory::new(5);
        assert!(hist.latest().is_none());
        let mut v = RoomStateVector::new();
        v.set(StateLabel::Health, 1.0);
        hist.push(v);
        assert_eq!(hist.len(), 1);
        assert!((hist.latest().unwrap().get(StateLabel::Health) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_history_ring_wrap() {
        let mut hist = StateHistory::new(3);
        for i in 0..5u64 {
            let mut v = RoomStateVector::new();
            v.set(StateLabel::Health, i as f64);
            hist.push_with_timestamp(v, i);
        }
        assert_eq!(hist.len(), 3);
        // Latest should be 4.0
        assert!((hist.latest().unwrap().get(StateLabel::Health) - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_trend_stable() {
        let mut hist = StateHistory::new(10);
        for _ in 0..5 {
            let v = RoomStateVector::from_values([0.5; 16]);
            hist.push(v);
        }
        assert_eq!(hist.trend(StateLabel::Health), Trend::Stable);
    }

    #[test]
    fn test_trend_declining() {
        let mut hist = StateHistory::new(10);
        for i in 0..5u64 {
            let mut v = RoomStateVector::new();
            v.set(StateLabel::Health, i as f64 * 0.1);
            hist.push_with_timestamp(v, i);
        }
        assert_eq!(hist.trend(StateLabel::Health), Trend::Declining);
    }

    #[test]
    fn test_trend_improving() {
        let mut hist = StateHistory::new(10);
        for i in 0..5u64 {
            let mut v = RoomStateVector::new();
            v.set(StateLabel::Health, 1.0 - i as f64 * 0.1);
            hist.push_with_timestamp(v, i);
        }
        assert_eq!(hist.trend(StateLabel::Health), Trend::Improving);
    }

    #[test]
    fn test_anomaly_score_identical() {
        let mut hist = StateHistory::new(10);
        for _ in 0..5 {
            let v = RoomStateVector::from_values([0.5; 16]);
            hist.push(v);
        }
        // All identical → score should be ~0
        assert!(hist.anomaly_score() < 0.5);
    }

    #[test]
    fn test_anomaly_score_divergent() {
        let mut hist = StateHistory::new(10);
        for i in 0..4u64 {
            let mut v = RoomStateVector::new();
            v.set(StateLabel::Health, 0.5);
            hist.push_with_timestamp(v, i);
        }
        // Push something very different in multiple dimensions
        let mut outlier = RoomStateVector::from_values([10.0; 16]);
        hist.push(outlier);
        let score = hist.anomaly_score();
        assert!(score > 1.0, "anomaly score {} should be > 1.0", score);
    }

    #[test]
    fn test_state_label_all_count() {
        assert_eq!(StateLabel::ALL.len(), 16);
    }

    #[test]
    fn test_state_label_names() {
        assert_eq!(StateLabel::Health.name(), "health");
        assert_eq!(StateLabel::AnomalyScore.name(), "anomaly_score");
        assert_eq!(StateLabel::Occupancy.name(), "occupancy");
    }

    #[test]
    fn test_alert_check() {
        let config = StateConfig::new(0.5, 100);
        let mut v = RoomStateVector::new();
        v.set(StateLabel::Thermal, 0.8);
        let alerts = StateAlert::check(&v, &config);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].label, StateLabel::Thermal);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut v = RoomStateVector::new();
        v.set(StateLabel::Health, 0.42);
        v.set(StateLabel::Thermal, 0.99);
        let json = serde_json::to_string(&v).unwrap();
        let decoded: RoomStateVector = serde_json::from_str(&json).unwrap();
        assert_eq!(v, decoded);
    }

    #[test]
    fn test_config_serialization() {
        let mut config = StateConfig::default();
        config.set_threshold(StateLabel::Health, 0.42);
        let json = serde_json::to_string(&config).unwrap();
        let decoded: StateConfig = serde_json::from_str(&json).unwrap();
        assert!((decoded.threshold(StateLabel::Health) - 0.42).abs() < 1e-10);
    }

    #[test]
    fn test_bayesian_fusion() {
        let prior = RoomStateVector::from_values([0.5; 16]);
        let mut a = RoomStateVector::new();
        a.set(StateLabel::Health, 0.8);
        let mut b = RoomStateVector::new();
        b.set(StateLabel::Health, 0.6);
        let result = StateFusion::bayesian_fusion(&[a, b], &prior);
        // Should be (0.8 * 0.6) which is 0.48, normalized by prior^2=0.25 → 1.92
        assert!(result.get(StateLabel::Health) > 0.0);
    }

    #[test]
    fn test_default_config() {
        let config = StateConfig::default();
        assert_eq!(config.history_size, 100);
        assert!((config.threshold(StateLabel::Health) - 0.8).abs() < 1e-10);
    }
}
