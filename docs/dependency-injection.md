# Dependency injection

## [Dependency injection in Rust](https://chesedo.me/blog/manual-dependency-injection-rust/)
### Compile-Time Safety and Performance
* All errors must be compile-time errors to maintain Rust's safety guarantees.
* The container should support lazy instantiation of dependencies to optimize performance.

### Flexibility and Usability
* Support for returning concrete types, trait objects, and dynamic trait objects. *Overall, we want to keep a clean DX by **avoiding the use of generics**. Generics will just make the codebase harder to read and to pass our container around.*
* Ability to handle deeply nested dependencies without complicating the API for developers.
* Support for conditional dependencies based on runtime configuration.

### Lifetime Management
1. **Singleton Lifetime**: Objects created once and shared throughout the application's lifetime. Examples include *database connections*, *configuration managers*, and *global loggers*.
2. **Scoped Lifetime**: Objects tied to a specific scope of work, such as a single request in a web application or a single run of a batch process. Examples include *request IDs*, *user contexts*, or *scoped loggers*.
3. **Transient Lifetime**: New instances created each time they're requested. These are typically lightweight objects like *DTOs*, *formatters*, or *ID generators*.

### Advanced Features
* Support for asynchronous dependencies.
* Ability to return `dyn Trait` types for runtime polymorphism.

### List of Requirements for Dependency Container

#### Implementing Concrete Type Returns
```rust
use chrono::{DateTime, Utc};

// The container we are going to use to resolve dependencies.
// Like `ServiceCollection` in .NET Core
// And `ApplicationContext` in Spring
pub struct DependencyContainer;

impl DependencyContainer {
    pub fn new() -> Self {
        Self
    }

    // We just make a function to return the concrete type
    pub fn datetime(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
```

#### Implementing Trait Type Returns
```rust
impl DependencyContainer {
    // This time we are returning an abstract type
    // This allows us to change the implementation of this function to change
    // the low-level details without changing the business code
    pub fn data_collector_impl(&self) -> impl DataCollector {
        SimpleDataCollector::new("api_key".to_string())
    }
}


pub trait DataCollector {
    fn collect_data(&self) -> Vec<String>;
}

pub struct SimpleDataCollector {
    api_key: String,
}

impl SimpleDataCollector {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl DataCollector for SimpleDataCollector {
    fn collect_data(&self) -> Vec<String> {
        vec!["data1".to_string(), "data2".to_string()]
    }
}
```

#### Implementing Dynamic Trait Type Returns
```rust
impl DependencyContainer {
    // We can now conditionally choose which [DataCollector] to use at runtime
    fn create_data_collector_dyn(&self) -> Box<dyn DataCollector> {
        if false {
            Box::new(SimpleDataCollector::new("api_key".to_string()))
        } else {
            Box::new(SqlDataCollector::new("connection_string".to_string()))
        }
    }
}


pub struct SqlDataCollector {
    connection_string: String,
}

impl SqlDataCollector {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

impl DataCollector for SqlDataCollector {
    fn collect_data(&self) -> Vec<String> {
        vec!["sql_data1".to_string(), "sql_data2".to_string()]
    }
}
```



