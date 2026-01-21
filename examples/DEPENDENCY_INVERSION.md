# Dependency Inversion in Rust

This document explains how to implement dependency inversion (DI) patterns in Rust, with examples relevant to this codebase.

## Table of Contents
1. [What is Dependency Inversion?](#what-is-dependency-inversion)
2. [Why Use DI in Rust?](#why-use-di-in-rust)
3. [Common Patterns](#common-patterns)
4. [Running the Examples](#running-the-examples)
5. [Best Practices](#best-practices)

## What is Dependency Inversion?

Dependency Inversion is one of the SOLID principles. It states:
- **High-level modules should not depend on low-level modules. Both should depend on abstractions.**
- **Abstractions should not depend on details. Details should depend on abstractions.**

In Rust, we achieve this using **traits** as abstractions.

## Why Use DI in Rust?

1. **Testability**: Easily swap real implementations with mocks in tests
2. **Flexibility**: Change implementations without changing business logic
3. **Modularity**: Separate concerns cleanly
4. **Maintainability**: Easier to understand and modify code

## Common Patterns

### 1. Static Dispatch with Generics

**Best for**: Performance-critical code, when types are known at compile time

```rust
trait UserRepository {
    fn find(&self, id: u32) -> Option<User>;
}

struct UserService<R: UserRepository> {
    repository: R,  // Generic type parameter
}

impl<R: UserRepository> UserService<R> {
    fn new(repository: R) -> Self {
        Self { repository }
    }
}

// Usage
let repo = PostgresRepository::new();
let service = UserService::new(repo);  // No heap allocation, zero cost
```

**Pros:**
- Zero runtime cost (monomorphization)
- Best performance
- Compiler optimizations

**Cons:**
- Code bloat (multiple versions generated)
- Can't store different implementations in same collection
- Can't decide implementation at runtime

### 2. Dynamic Dispatch with `Box<dyn Trait>`

**Best for**: Runtime polymorphism, plugin systems

```rust
trait UserRepository {
    fn find(&self, id: u32) -> Option<User>;
}

struct UserService {
    repository: Box<dyn UserRepository>,  // Trait object
}

impl UserService {
    fn new(repository: Box<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

// Usage - decide at runtime
let repo: Box<dyn UserRepository> = if use_postgres {
    Box::new(PostgresRepository::new())
} else {
    Box::new(MockRepository::new())
};
let service = UserService::new(repo);
```

**Pros:**
- Runtime polymorphism
- Smaller binary size
- Can store different implementations in collections

**Cons:**
- Small runtime overhead (vtable lookup)
- Heap allocation required
- No compiler optimizations across trait boundary

### 3. Thread-Safe with `Arc<dyn Trait>`

**Best for**: Async code, shared ownership across threads

```rust
use std::sync::Arc;

#[async_trait]
trait UserRepository: Send + Sync {
    async fn find(&self, id: u32) -> Option<User>;
}

struct UserService {
    repository: Arc<dyn UserRepository>,  // Reference counted, thread-safe
}

impl UserService {
    fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

// Usage - can be cloned cheaply
let repo = Arc::new(PostgresRepository::new());
let service1 = UserService::new(repo.clone());
let service2 = UserService::new(repo.clone());  // Same instance
```

**Pros:**
- Thread-safe shared ownership
- Cheap cloning (just increments counter)
- Perfect for async/await

**Cons:**
- Slight overhead from reference counting
- Requires `Send + Sync` bounds

### 4. Multiple Dependencies

**Best for**: Services with multiple collaborators

```rust
struct UserService<R, C, L>
where
    R: UserRepository,
    C: CacheService,
    L: Logger,
{
    repository: R,
    cache: C,
    logger: L,
}

impl<R, C, L> UserService<R, C, L>
where
    R: UserRepository,
    C: CacheService,
    L: Logger,
{
    fn new(repository: R, cache: C, logger: L) -> Self {
        Self { repository, cache, logger }
    }

    fn get_user(&self, id: u32) -> Option<User> {
        self.logger.log("Fetching user");

        if let Some(user) = self.cache.get(id) {
            return Some(user);
        }

        let user = self.repository.find(id);
        if let Some(ref u) = user {
            self.cache.set(id, u.clone());
        }

        user
    }
}
```

### 5. Factory Pattern

**Best for**: Complex initialization, configuration-based setup

```rust
struct ServiceFactory {
    database_url: String,
    cache_enabled: bool,
}

impl ServiceFactory {
    fn new(database_url: String) -> Self {
        Self {
            database_url,
            cache_enabled: false,
        }
    }

    fn with_cache(mut self, enabled: bool) -> Self {
        self.cache_enabled = enabled;
        self
    }

    fn build_user_service(self) -> UserService {
        let repo = PostgresRepository::new(&self.database_url);

        let cache = if self.cache_enabled {
            Box::new(RedisCache::new()) as Box<dyn CacheService>
        } else {
            Box::new(NoOpCache) as Box<dyn CacheService>
        };

        UserService::new(repo, cache)
    }

    // For testing
    fn build_test_service() -> UserService {
        let repo = MockRepository::new();
        let cache = MockCache::new();
        UserService::new(repo, cache)
    }
}

// Usage
let service = ServiceFactory::new("postgresql://localhost".to_string())
    .with_cache(true)
    .build_user_service();
```

### 6. Async Traits with `async-trait`

**Best for**: Async operations (database queries, API calls)

```rust
use async_trait::async_trait;

#[async_trait]
trait UserRepository: Send + Sync {
    async fn find(&self, id: u32) -> Result<Option<User>, Error>;
    async fn save(&self, user: &User) -> Result<(), Error>;
}

struct PostgresRepository {
    pool: sqlx::PgPool,
}

#[async_trait]
impl UserRepository for PostgresRepository {
    async fn find(&self, id: u32) -> Result<Option<User>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::Database(e))
    }

    async fn save(&self, user: &User) -> Result<(), Error> {
        sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
            .bind(user.id)
            .bind(&user.name)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| Error::Database(e))
    }
}
```

## Running the Examples

### Basic Example
```bash
cargo run --example dependency_inversion
```

### Real-world Example
```bash
cargo run --example real_world_di
```

### Running Tests
```bash
cargo test --example dependency_inversion
cargo test --example real_world_di
```

## Best Practices

### 1. Choose the Right Pattern

| Use Case | Pattern | Example |
|----------|---------|---------|
| Known at compile time | Generic `<T: Trait>` | Configuration objects |
| Runtime decision | `Box<dyn Trait>` | Plugin systems |
| Async + shared | `Arc<dyn Trait>` | Database pools, services |
| Testing | All of the above | Mock implementations |

### 2. Keep Traits Focused

```rust
// ❌ Bad - too many responsibilities
trait UserRepository {
    fn find(&self, id: u32) -> Option<User>;
    fn save(&self, user: User);
    fn send_email(&self, user: &User);  // Not repository's job!
    fn log(&self, msg: &str);           // Not repository's job!
}

// ✅ Good - single responsibility
trait UserRepository {
    fn find(&self, id: u32) -> Option<User>;
    fn save(&self, user: User);
}

trait EmailService {
    fn send(&self, to: &str, subject: &str, body: &str);
}

trait Logger {
    fn log(&self, msg: &str);
}
```

### 3. Use Builder Pattern for Complex Setup

```rust
struct UserServiceBuilder {
    repository: Option<Box<dyn UserRepository>>,
    cache: Option<Box<dyn CacheService>>,
    logger: Option<Box<dyn Logger>>,
}

impl UserServiceBuilder {
    fn new() -> Self {
        Self {
            repository: None,
            cache: None,
            logger: None,
        }
    }

    fn with_repository(mut self, repo: Box<dyn UserRepository>) -> Self {
        self.repository = Some(repo);
        self
    }

    fn with_cache(mut self, cache: Box<dyn CacheService>) -> Self {
        self.cache = Some(cache);
        self
    }

    fn build(self) -> Result<UserService, String> {
        Ok(UserService {
            repository: self.repository.ok_or("Repository required")?,
            cache: self.cache.unwrap_or_else(|| Box::new(NoOpCache)),
            logger: self.logger.unwrap_or_else(|| Box::new(ConsoleLogger)),
        })
    }
}
```

### 4. Make Testing Easy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockRepository {
        users: Vec<User>,
        save_called: std::sync::Arc<std::sync::Mutex<bool>>,
    }

    impl MockRepository {
        fn new() -> Self {
            Self {
                users: vec![],
                save_called: std::sync::Arc::new(std::sync::Mutex::new(false)),
            }
        }

        fn was_save_called(&self) -> bool {
            *self.save_called.lock().unwrap()
        }
    }

    impl UserRepository for MockRepository {
        fn find(&self, id: u32) -> Option<User> {
            self.users.iter().find(|u| u.id == id).cloned()
        }

        fn save(&self, user: User) {
            *self.save_called.lock().unwrap() = true;
            // Mock implementation
        }
    }

    #[test]
    fn test_user_service_saves_user() {
        let mock_repo = MockRepository::new();
        let service = UserService::new(mock_repo);

        let user = User { id: 1, name: "Test".to_string() };
        service.create_user(user);

        assert!(service.repository.was_save_called());
    }
}
```

### 5. Document Trait Requirements

```rust
/// Repository for managing user data.
///
/// # Thread Safety
/// Implementations must be `Send + Sync` for use in async contexts.
///
/// # Error Handling
/// All methods return `Result` with specific error types.
/// Database errors should be wrapped in `Error::Database`.
///
/// # Example
/// ```
/// let repo = PostgresUserRepository::new(pool);
/// let user = repo.find(123).await?;
/// ```
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Find a user by their unique ID.
    ///
    /// Returns `None` if the user doesn't exist.
    async fn find(&self, id: u32) -> Result<Option<User>, Error>;

    /// Save a user to the database.
    ///
    /// If the user already exists, it will be updated.
    async fn save(&self, user: &User) -> Result<(), Error>;
}
```

## Comparison with Your Current Codebase

### Current Pattern in `crates/biz`

Your codebase already uses good DI patterns:

```rust
// crates/biz/src/auth_service.rs
pub struct AuthService {
    pool: PgPool,
    cache: Option<Cache>,
}
```

### Suggested Improvement

Make dependencies more abstract:

```rust
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn create_user(&self, user: &User) -> Result<()>;
}

pub struct AuthService<R: AuthRepository, C: CacheService> {
    repository: R,
    cache: C,
}

// Now you can easily test with mocks:
#[cfg(test)]
struct MockAuthRepository { /* ... */ }

#[async_trait]
impl AuthRepository for MockAuthRepository {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>> {
        // Return controlled test data
        Ok(Some(User::test_user()))
    }
}
```

## Additional Resources

- [Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust by Example - Trait Objects](https://doc.rust-lang.org/rust-by-example/trait/dyn.html)
- [async-trait crate](https://docs.rs/async-trait/)
- [Dependency Injection in Rust](https://kevinlynagh.com/notes/architecture-scale-up/)
