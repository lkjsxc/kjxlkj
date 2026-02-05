//! User-defined functions for scripting.
//!
//! Implements user function registry and call mechanisms as specified in
//! `/docs/spec/scripting/user-functions.md`.

use std::collections::HashMap;

/// A value that can be passed to or returned from a function.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FunctionValue {
    /// Null/empty value.
    #[default]
    Null,
    /// Boolean value.
    Bool(bool),
    /// Integer value.
    Int(i64),
    /// Float value.
    Float(f64),
    /// String value.
    String(String),
    /// List of values.
    List(Vec<FunctionValue>),
    /// Dictionary/map of values.
    Dict(HashMap<String, FunctionValue>),
}

impl FunctionValue {
    /// Check if the value is truthy.
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Bool(b) => *b,
            Self::Int(i) => *i != 0,
            Self::Float(f) => *f != 0.0,
            Self::String(s) => !s.is_empty(),
            Self::List(l) => !l.is_empty(),
            Self::Dict(d) => !d.is_empty(),
        }
    }

    /// Convert to string representation.
    pub fn to_string_value(&self) -> String {
        match self {
            Self::Null => String::new(),
            Self::Bool(b) => if *b { "1" } else { "0" }.to_string(),
            Self::Int(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::String(s) => s.clone(),
            Self::List(l) => l
                .iter()
                .map(|v| v.to_string_value())
                .collect::<Vec<_>>()
                .join(" "),
            Self::Dict(d) => format!("{:?}", d),
        }
    }

    /// Try to convert to integer.
    pub fn to_int(&self) -> Option<i64> {
        match self {
            Self::Null => Some(0),
            Self::Bool(b) => Some(if *b { 1 } else { 0 }),
            Self::Int(i) => Some(*i),
            Self::Float(f) => Some(*f as i64),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }
}

/// Parameter definition for a function.
#[derive(Debug, Clone)]
pub struct FunctionParam {
    /// Parameter name.
    pub name: String,
    /// Optional default value.
    pub default: Option<FunctionValue>,
    /// Whether this is a variadic parameter.
    pub variadic: bool,
}

impl FunctionParam {
    /// Create a required parameter.
    pub fn required(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            default: None,
            variadic: false,
        }
    }

    /// Create an optional parameter with default.
    pub fn optional(name: impl Into<String>, default: FunctionValue) -> Self {
        Self {
            name: name.into(),
            default: Some(default),
            variadic: false,
        }
    }

    /// Create a variadic parameter.
    pub fn variadic(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            default: None,
            variadic: true,
        }
    }
}

/// Result of a function call.
pub type FunctionResult = Result<FunctionValue, FunctionError>;

/// Error from function execution.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionError {
    /// Function not found.
    NotFound(String),
    /// Wrong number of arguments.
    ArgumentCount { expected: usize, got: usize },
    /// Type mismatch.
    TypeMismatch { expected: String, got: String },
    /// Runtime error.
    Runtime(String),
}

impl std::fmt::Display for FunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "Unknown function: {}", name),
            Self::ArgumentCount { expected, got } => {
                write!(f, "Expected {} arguments, got {}", expected, got)
            }
            Self::TypeMismatch { expected, got } => {
                write!(f, "Expected {}, got {}", expected, got)
            }
            Self::Runtime(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for FunctionError {}

/// Trait for callable functions.
pub trait Callable: Send + Sync {
    /// Get the function name.
    fn name(&self) -> &str;

    /// Get the parameter definitions.
    fn params(&self) -> &[FunctionParam];

    /// Call the function with arguments.
    fn call(&self, args: Vec<FunctionValue>) -> FunctionResult;
}

/// A built-in function implementation.
pub struct BuiltinFunction {
    name: String,
    params: Vec<FunctionParam>,
    func: Box<dyn Fn(Vec<FunctionValue>) -> FunctionResult + Send + Sync>,
}

impl BuiltinFunction {
    /// Create a new builtin function.
    pub fn new<F>(
        name: impl Into<String>,
        params: Vec<FunctionParam>,
        func: F,
    ) -> Self
    where
        F: Fn(Vec<FunctionValue>) -> FunctionResult + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            params,
            func: Box::new(func),
        }
    }
}

impl Callable for BuiltinFunction {
    fn name(&self) -> &str {
        &self.name
    }

    fn params(&self) -> &[FunctionParam] {
        &self.params
    }

    fn call(&self, args: Vec<FunctionValue>) -> FunctionResult {
        (self.func)(args)
    }
}

/// Registry of available functions.
#[derive(Default)]
pub struct FunctionRegistry {
    functions: HashMap<String, Box<dyn Callable>>,
}

impl FunctionRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry with standard functions.
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register_builtins();
        registry
    }

    /// Register built-in functions.
    fn register_builtins(&mut self) {
        // String functions
        self.register(BuiltinFunction::new(
            "strlen",
            vec![FunctionParam::required("string")],
            |args| {
                let s = args.first().map(|v| v.to_string_value()).unwrap_or_default();
                Ok(FunctionValue::Int(s.chars().count() as i64))
            },
        ));

        self.register(BuiltinFunction::new(
            "tolower",
            vec![FunctionParam::required("string")],
            |args| {
                let s = args.first().map(|v| v.to_string_value()).unwrap_or_default();
                Ok(FunctionValue::String(s.to_lowercase()))
            },
        ));

        self.register(BuiltinFunction::new(
            "toupper",
            vec![FunctionParam::required("string")],
            |args| {
                let s = args.first().map(|v| v.to_string_value()).unwrap_or_default();
                Ok(FunctionValue::String(s.to_uppercase()))
            },
        ));

        self.register(BuiltinFunction::new(
            "substitute",
            vec![
                FunctionParam::required("string"),
                FunctionParam::required("pattern"),
                FunctionParam::required("replacement"),
                FunctionParam::optional("flags", FunctionValue::String(String::new())),
            ],
            |args| {
                let string = args.first().map(|v| v.to_string_value()).unwrap_or_default();
                let pattern = args.get(1).map(|v| v.to_string_value()).unwrap_or_default();
                let replacement = args.get(2).map(|v| v.to_string_value()).unwrap_or_default();
                let flags = args.get(3).map(|v| v.to_string_value()).unwrap_or_default();

                let result = if flags.contains('g') {
                    string.replace(&pattern, &replacement)
                } else {
                    string.replacen(&pattern, &replacement, 1)
                };
                Ok(FunctionValue::String(result))
            },
        ));

        // List functions
        self.register(BuiltinFunction::new(
            "len",
            vec![FunctionParam::required("object")],
            |args| {
                let len = match args.first() {
                    Some(FunctionValue::String(s)) => s.chars().count(),
                    Some(FunctionValue::List(l)) => l.len(),
                    Some(FunctionValue::Dict(d)) => d.len(),
                    _ => 0,
                };
                Ok(FunctionValue::Int(len as i64))
            },
        ));

        self.register(BuiltinFunction::new(
            "empty",
            vec![FunctionParam::required("object")],
            |args| {
                let is_empty = match args.first() {
                    Some(FunctionValue::Null) => true,
                    Some(FunctionValue::String(s)) => s.is_empty(),
                    Some(FunctionValue::List(l)) => l.is_empty(),
                    Some(FunctionValue::Dict(d)) => d.is_empty(),
                    _ => true,
                };
                Ok(FunctionValue::Bool(is_empty))
            },
        ));

        // Type functions
        self.register(BuiltinFunction::new(
            "type",
            vec![FunctionParam::required("value")],
            |args| {
                let type_num = match args.first() {
                    Some(FunctionValue::Int(_)) => 0,
                    Some(FunctionValue::String(_)) => 1,
                    Some(FunctionValue::List(_)) => 3,
                    Some(FunctionValue::Dict(_)) => 4,
                    Some(FunctionValue::Float(_)) => 5,
                    Some(FunctionValue::Bool(_)) => 6,
                    Some(FunctionValue::Null) | None => 7,
                };
                Ok(FunctionValue::Int(type_num))
            },
        ));

        // Math functions
        self.register(BuiltinFunction::new(
            "abs",
            vec![FunctionParam::required("number")],
            |args| {
                match args.first() {
                    Some(FunctionValue::Int(i)) => Ok(FunctionValue::Int(i.abs())),
                    Some(FunctionValue::Float(f)) => Ok(FunctionValue::Float(f.abs())),
                    _ => Ok(FunctionValue::Int(0)),
                }
            },
        ));

        self.register(BuiltinFunction::new(
            "min",
            vec![FunctionParam::variadic("values")],
            |args| {
                let min = args
                    .iter()
                    .filter_map(|v| v.to_int())
                    .min()
                    .unwrap_or(0);
                Ok(FunctionValue::Int(min))
            },
        ));

        self.register(BuiltinFunction::new(
            "max",
            vec![FunctionParam::variadic("values")],
            |args| {
                let max = args
                    .iter()
                    .filter_map(|v| v.to_int())
                    .max()
                    .unwrap_or(0);
                Ok(FunctionValue::Int(max))
            },
        ));
    }

    /// Register a function.
    pub fn register<C: Callable + 'static>(&mut self, func: C) {
        self.functions.insert(func.name().to_string(), Box::new(func));
    }

    /// Call a function by name.
    pub fn call(&self, name: &str, args: Vec<FunctionValue>) -> FunctionResult {
        match self.functions.get(name) {
            Some(func) => func.call(args),
            None => Err(FunctionError::NotFound(name.to_string())),
        }
    }

    /// Check if a function exists.
    pub fn exists(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get list of registered function names.
    pub fn function_names(&self) -> Vec<&str> {
        self.functions.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_value_truthy() {
        assert!(!FunctionValue::Null.is_truthy());
        assert!(!FunctionValue::Bool(false).is_truthy());
        assert!(FunctionValue::Bool(true).is_truthy());
        assert!(!FunctionValue::Int(0).is_truthy());
        assert!(FunctionValue::Int(1).is_truthy());
        assert!(!FunctionValue::String(String::new()).is_truthy());
        assert!(FunctionValue::String("hello".to_string()).is_truthy());
    }

    #[test]
    fn test_function_value_to_string() {
        assert_eq!(FunctionValue::Null.to_string_value(), "");
        assert_eq!(FunctionValue::Bool(true).to_string_value(), "1");
        assert_eq!(FunctionValue::Bool(false).to_string_value(), "0");
        assert_eq!(FunctionValue::Int(42).to_string_value(), "42");
        assert_eq!(FunctionValue::String("hello".to_string()).to_string_value(), "hello");
    }

    #[test]
    fn test_function_value_to_int() {
        assert_eq!(FunctionValue::Null.to_int(), Some(0));
        assert_eq!(FunctionValue::Bool(true).to_int(), Some(1));
        assert_eq!(FunctionValue::Int(42).to_int(), Some(42));
        assert_eq!(FunctionValue::String("123".to_string()).to_int(), Some(123));
        assert_eq!(FunctionValue::String("abc".to_string()).to_int(), None);
    }

    #[test]
    fn test_function_param_required() {
        let param = FunctionParam::required("arg");
        assert_eq!(param.name, "arg");
        assert!(param.default.is_none());
        assert!(!param.variadic);
    }

    #[test]
    fn test_function_param_optional() {
        let param = FunctionParam::optional("arg", FunctionValue::Int(0));
        assert_eq!(param.name, "arg");
        assert_eq!(param.default, Some(FunctionValue::Int(0)));
    }

    #[test]
    fn test_function_param_variadic() {
        let param = FunctionParam::variadic("args");
        assert!(param.variadic);
    }

    #[test]
    fn test_function_error_display() {
        let err = FunctionError::NotFound("foo".to_string());
        assert!(err.to_string().contains("Unknown function"));

        let err = FunctionError::ArgumentCount { expected: 2, got: 1 };
        assert!(err.to_string().contains("Expected 2"));
    }

    #[test]
    fn test_registry_new() {
        let registry = FunctionRegistry::new();
        assert!(!registry.exists("strlen"));
    }

    #[test]
    fn test_registry_with_builtins() {
        let registry = FunctionRegistry::with_builtins();
        assert!(registry.exists("strlen"));
        assert!(registry.exists("tolower"));
        assert!(registry.exists("toupper"));
        assert!(registry.exists("len"));
        assert!(registry.exists("empty"));
        assert!(registry.exists("type"));
        assert!(registry.exists("abs"));
        assert!(registry.exists("min"));
        assert!(registry.exists("max"));
    }

    #[test]
    fn test_strlen_function() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call("strlen", vec![FunctionValue::String("hello".to_string())]);
        assert_eq!(result, Ok(FunctionValue::Int(5)));
    }

    #[test]
    fn test_strlen_unicode() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call("strlen", vec![FunctionValue::String("日本語".to_string())]);
        assert_eq!(result, Ok(FunctionValue::Int(3)));
    }

    #[test]
    fn test_tolower_function() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call("tolower", vec![FunctionValue::String("HELLO".to_string())]);
        assert_eq!(result, Ok(FunctionValue::String("hello".to_string())));
    }

    #[test]
    fn test_toupper_function() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call("toupper", vec![FunctionValue::String("hello".to_string())]);
        assert_eq!(result, Ok(FunctionValue::String("HELLO".to_string())));
    }

    #[test]
    fn test_substitute_function() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call(
            "substitute",
            vec![
                FunctionValue::String("hello world".to_string()),
                FunctionValue::String("world".to_string()),
                FunctionValue::String("rust".to_string()),
            ],
        );
        assert_eq!(result, Ok(FunctionValue::String("hello rust".to_string())));
    }

    #[test]
    fn test_substitute_global() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call(
            "substitute",
            vec![
                FunctionValue::String("a b a".to_string()),
                FunctionValue::String("a".to_string()),
                FunctionValue::String("x".to_string()),
                FunctionValue::String("g".to_string()),
            ],
        );
        assert_eq!(result, Ok(FunctionValue::String("x b x".to_string())));
    }

    #[test]
    fn test_len_function() {
        let registry = FunctionRegistry::with_builtins();

        let result = registry.call("len", vec![FunctionValue::String("hello".to_string())]);
        assert_eq!(result, Ok(FunctionValue::Int(5)));

        let result = registry.call("len", vec![FunctionValue::List(vec![
            FunctionValue::Int(1),
            FunctionValue::Int(2),
            FunctionValue::Int(3),
        ])]);
        assert_eq!(result, Ok(FunctionValue::Int(3)));
    }

    #[test]
    fn test_empty_function() {
        let registry = FunctionRegistry::with_builtins();

        let result = registry.call("empty", vec![FunctionValue::String(String::new())]);
        assert_eq!(result, Ok(FunctionValue::Bool(true)));

        let result = registry.call("empty", vec![FunctionValue::String("hello".to_string())]);
        assert_eq!(result, Ok(FunctionValue::Bool(false)));
    }

    #[test]
    fn test_type_function() {
        let registry = FunctionRegistry::with_builtins();

        let result = registry.call("type", vec![FunctionValue::Int(42)]);
        assert_eq!(result, Ok(FunctionValue::Int(0)));

        let result = registry.call("type", vec![FunctionValue::String("hello".to_string())]);
        assert_eq!(result, Ok(FunctionValue::Int(1)));
    }

    #[test]
    fn test_abs_function() {
        let registry = FunctionRegistry::with_builtins();

        let result = registry.call("abs", vec![FunctionValue::Int(-5)]);
        assert_eq!(result, Ok(FunctionValue::Int(5)));

        let result = registry.call("abs", vec![FunctionValue::Float(-2.5)]);
        if let Ok(FunctionValue::Float(f)) = result {
            assert!((f - 2.5).abs() < 0.001);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_min_function() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call("min", vec![
            FunctionValue::Int(5),
            FunctionValue::Int(3),
            FunctionValue::Int(7),
        ]);
        assert_eq!(result, Ok(FunctionValue::Int(3)));
    }

    #[test]
    fn test_max_function() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call("max", vec![
            FunctionValue::Int(5),
            FunctionValue::Int(3),
            FunctionValue::Int(7),
        ]);
        assert_eq!(result, Ok(FunctionValue::Int(7)));
    }

    #[test]
    fn test_function_not_found() {
        let registry = FunctionRegistry::with_builtins();
        let result = registry.call("nonexistent", vec![]);
        assert_eq!(result, Err(FunctionError::NotFound("nonexistent".to_string())));
    }

    #[test]
    fn test_function_names() {
        let registry = FunctionRegistry::with_builtins();
        let names = registry.function_names();
        assert!(names.contains(&"strlen"));
        assert!(names.contains(&"len"));
    }

    #[test]
    fn test_custom_function() {
        let mut registry = FunctionRegistry::new();

        registry.register(BuiltinFunction::new(
            "double",
            vec![FunctionParam::required("n")],
            |args| {
                let n = args.first().and_then(|v| v.to_int()).unwrap_or(0);
                Ok(FunctionValue::Int(n * 2))
            },
        ));

        assert!(registry.exists("double"));
        let result = registry.call("double", vec![FunctionValue::Int(21)]);
        assert_eq!(result, Ok(FunctionValue::Int(42)));
    }

    #[test]
    fn test_function_value_default() {
        let val = FunctionValue::default();
        assert_eq!(val, FunctionValue::Null);
    }

    #[test]
    fn test_function_value_list() {
        let list = FunctionValue::List(vec![
            FunctionValue::Int(1),
            FunctionValue::Int(2),
        ]);
        assert!(list.is_truthy());
        assert_eq!(list.to_string_value(), "1 2");
    }
}
