use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScalarValue {
    Boolean(bool),
    Number(f64),
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDeclaration {
    #[serde(rename = "type")]
    pub value_type: String,
    pub initial: ScalarValue,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub allowed: Option<Vec<ScalarValue>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOperator {
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "is-not")]
    IsNot,
    #[serde(rename = "greater-than")]
    GreaterThan,
    #[serde(rename = "less-than")]
    LessThan,
    #[serde(rename = "at-least")]
    AtLeast,
    #[serde(rename = "at-most")]
    AtMost,
    #[serde(rename = "contains")]
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub path: String,
    pub operator: ConditionOperator,
    pub value: ScalarValue,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateRegistry {
    pub values: HashMap<String, ScalarValue>,
}

impl StateRegistry {
    pub fn get(&self, path: &str) -> Option<&ScalarValue> {
        self.values.get(path)
    }

    pub fn set(&mut self, path: String, value: ScalarValue) {
        self.values.insert(path, value);
    }

    pub fn change_number(&mut self, path: &str, delta: f64) -> Result<(), &'static str> {
        match self.values.get_mut(path) {
            Some(ScalarValue::Number(n)) => {
                *n += delta;
                Ok(())
            }
            _ => Err("Target state path is not a valid declared numeric variable."),
        }
    }

    pub fn evaluate_condition(&self, cond: &Condition) -> bool {
        let current = match self.get(&cond.path) {
            Some(val) => val,
            None => return false,
        };

        match (&cond.operator, current, &cond.value) {
            (ConditionOperator::Is, a, b) => a == b,
            (ConditionOperator::IsNot, a, b) => a != b,
            (ConditionOperator::GreaterThan, ScalarValue::Number(a), ScalarValue::Number(b)) => a > b,
            (ConditionOperator::LessThan, ScalarValue::Number(a), ScalarValue::Number(b)) => a < b,
            (ConditionOperator::AtLeast, ScalarValue::Number(a), ScalarValue::Number(b)) => a >= b,
            (ConditionOperator::AtMost, ScalarValue::Number(a), ScalarValue::Number(b)) => a <= b,
            (ConditionOperator::Contains, ScalarValue::Text(a), ScalarValue::Text(b)) => a.contains(b),
            _ => false,
        }
    }
}