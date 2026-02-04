use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Push(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn to_string(&self) -> String {
        match self {
            Operation::Push(val) => format!("push {}", val),
            Operation::Add => "add".to_string(),
            Operation::Subtract => "subtract".to_string(),
            Operation::Multiply => "multiply".to_string(),
            Operation::Divide => "divide".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub operation: Operation,
    pub stack_before: Vec<f64>,
    pub stack_after: Vec<f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub enum CalcError {
    InsufficientOperands,
    DivisionByZero,
    EmptyHistory,
    FileError(String),
    SerializationError(String),
}
impl std::error::Error for CalcError {}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalcError::InsufficientOperands => write!(f, "Insufficient operands on stack"),
            CalcError::DivisionByZero => write!(f, "Division by zero"),
            CalcError::EmptyHistory => write!(f, "No history to undo"),
            CalcError::FileError(msg) => write!(f, "File error: {}", msg),
            CalcError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

pub struct RpnCalculator {
    stack: Vec<f64>,
    history: Vec<HistoryEntry>,
}

impl RpnCalculator {
    pub fn new() -> Self {
        RpnCalculator {
            stack: Vec::new(),
            history: Vec::new(),
        }
    }

    fn record_operation(&mut self, operation: Operation, stack_before: Vec<f64>) {
        let entry = HistoryEntry {
            operation,
            stack_before,
            stack_after: self.stack.clone(),
            timestamp: Utc::now(),
        };
        self.history.push(entry);
    }

    pub fn push(&mut self, value: f64) {
        let stack_before = self.stack.clone();
        self.stack.push(value);
        self.record_operation(Operation::Push(value), stack_before);
    }

    pub fn add(&mut self) -> Result<(), CalcError> {
        if self.stack.len() < 2 {
            return Err(CalcError::InsufficientOperands);
        }
        
        let stack_before = self.stack.clone();
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a + b);
        self.record_operation(Operation::Add, stack_before);
        Ok(())
    }

    pub fn subtract(&mut self) -> Result<(), CalcError> {
        if self.stack.len() < 2 {
            return Err(CalcError::InsufficientOperands);
        }
        
        let stack_before = self.stack.clone();
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a - b);
        self.record_operation(Operation::Subtract, stack_before);
        Ok(())
    }

    pub fn multiply(&mut self) -> Result<(), CalcError> {
        if self.stack.len() < 2 {
            return Err(CalcError::InsufficientOperands);
        }
        
        let stack_before = self.stack.clone();
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a * b);
        self.record_operation(Operation::Multiply, stack_before);
        Ok(())
    }

    pub fn divide(&mut self) -> Result<(), CalcError> {
        if self.stack.len() < 2 {
            return Err(CalcError::InsufficientOperands);
        }
        
        let stack_before = self.stack.clone();
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        
        if b == 0.0 {
            self.stack.push(a);
            self.stack.push(b);
            return Err(CalcError::DivisionByZero);
        }
        
        self.stack.push(a / b);
        self.record_operation(Operation::Divide, stack_before);
        Ok(())
    }

    pub fn display(&self) {
        if self.stack.is_empty() {
            println!("Stack is empty");
        } else {
            println!("Stack: {:?}", self.stack);
        }
    }

    pub fn undo(&mut self) -> Result<(), CalcError> {
        if self.history.is_empty() {
            return Err(CalcError::EmptyHistory);
        }
        
        let last_entry = self.history.pop().unwrap();
        self.stack = last_entry.stack_before;
        Ok(())
    }

    // pub fn display_history(&self) {jsp}

    pub fn save_history(&self, file_path: &str) -> Result<(), CalcError> {
        let json = serde_json::to_string(&self.history)
        .map_err(|e| CalcError::SerializationError(e.to_string()))?;
        
        fs::write(file_path, json).map_err(|e| CalcError::FileError(e.to_string()))?;
        
        println!("History saved to {}", file_path);
        Ok(())
    }

    pub fn load_history(&mut self, file_path: &str) -> Result<(), CalcError> {
        let json = fs::read_to_string(file_path)
            .map_err(|e| CalcError::FileError(e.to_string()))?;
        
        self.history = serde_json::from_str(&json)
            .map_err(|e| CalcError::SerializationError(e.to_string()))?;
        
        if let Some(last_entry) = self.history.last() {
            self.stack = last_entry.stack_after.clone();
        }
        
        println!("History loaded from {}", file_path);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.history.clear();
        println!("Calculator cleared");
    }
}