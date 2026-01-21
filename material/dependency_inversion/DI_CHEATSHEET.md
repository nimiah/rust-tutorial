# Dependency Inversion Cheat Sheet

Quick reference for implementing DI patterns in Rust.

## Pattern Selection

```rust
// 1. Known at compile time? Use generics
struct Service<R: Repository> {
    repo: R
}

// 2. Runtime polymorphism? Use Box<dyn>
struct Service {
    repo: Box<dyn Repository>
}

// 3. Async + shared? Use Arc<dyn>
struct Service {
    repo: Arc<dyn Repository>
}
```

## Basic Trait Definition

```rust
// Sync trait
trait Repository {
    fn find(&self, id: u32) -> Option<User>;
}

// Async trait (requires async-trait crate)
#[async_trait]
trait Repository: Send + Sync {
    async fn find(&self, id: u32) -> Result<User, Error>;
}
```

## Implementation Examples

### 1. Static Dispatch (Generics)

```rust
// Define
struct Service<R: Repository> {
    repository: R,
}

impl<R: Repository> Service<R> {
    fn new(repository: R) -> Self {
        Self { repository }
    }
}

// Use
let service = Service::new(PostgresRepo::new());
```

**When to use:** Performance critical, types known at compile time

### 2. Dynamic Dispatch (Box)

```rust
// Define
struct Service {
    repository: Box<dyn Repository>,
}

impl Service {
    fn new(repository: Box<dyn Repository>) -> Self {
        Self { repository }
    }
}

// Use
let repo: Box<dyn Repository> = Box::new(PostgresRepo::new());
let service = Service::new(repo);
```

**When to use:** Need runtime polymorphism, single ownership

### 3. Shared Ownership (Arc)

```rust
// Define
struct Service {
    repository: Arc<dyn Repository>,
}

impl Service {
    fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

// Use
let repo = Arc::new(PostgresRepo::new());
let service1 = Service::new(repo.clone());
let service2 = Service::new(repo.clone()); // Same instance
```

**When to use:** Async code, need to share across tasks/threads

## Testing Patterns

### Mock Implementation

```rust
#[cfg(test)]
struct MockRepository {
    users: Vec<User>,
}

#[async_trait]
impl Repository for MockRepository {
    async fn find(&self, id: u32) -> Result<User, Error> {
        self.users.iter()
            .find(|u| u.id == id)
            .cloned()
            .ok_or(Error::NotFound)
    }
}

#[tokio::test]
async fn test_service() {
    let mock = MockRepository { users: vec![test_user()] };
    let service = Service::new(Arc::new(mock));

    let result = service.get_user(1).await;
    assert!(result.is_ok());
}
```

### Spy Pattern (Verify Calls)

```rust
struct SpyRepository {
    find_called: Arc<Mutex<bool>>,
}

impl SpyRepository {
    fn new() -> Self {
        Self {
            find_called: Arc::new(Mutex::new(false)),
        }
    }

    fn was_find_called(&self) -> bool {
        *self.find_called.lock().unwrap()
    }
}

#[async_trait]
impl Repository for SpyRepository {
    async fn find(&self, id: u32) -> Result<User, Error> {
        *self.find_called.lock().unwrap() = true;
        Err(Error::NotFound)
    }
}

#[test]
fn test_calls_repository() {
    let spy = SpyRepository::new();
    let service = Service::new(Arc::new(spy.clone()));

    service.get_user(1).await;
    assert!(spy.was_find_called());
}
```

## Factory Pattern

```rust
struct ServiceFactory;

impl ServiceFactory {
    // Production
    fn create_prod(db_pool: PgPool) -> Service {
        let repo: Arc<dyn Repository> = Arc::new(PostgresRepo::new(db_pool));
        Service::new(repo)
    }

    // Testing
    fn create_test() -> Service {
        let repo: Arc<dyn Repository> = Arc::new(MockRepository::new());
        Service::new(repo)
    }

    // Testing with specific data
    fn create_test_with_users(users: Vec<User>) -> Service {
        let repo: Arc<dyn Repository> = Arc::new(MockRepository::with_users(users));
        Service::new(repo)
    }
}

// Usage
let prod_service = ServiceFactory::create_prod(pool);
let test_service = ServiceFactory::create_test();
```

## Builder Pattern

```rust
struct ServiceBuilder {
    repository: Option<Arc<dyn Repository>>,
    cache: Option<Arc<dyn Cache>>,
}

impl ServiceBuilder {
    fn new() -> Self {
        Self {
            repository: None,
            cache: None,
        }
    }

    fn with_repository(mut self, repo: Arc<dyn Repository>) -> Self {
        self.repository = Some(repo);
        self
    }

    fn with_cache(mut self, cache: Arc<dyn Cache>) -> Self {
        self.cache = Some(cache);
        self
    }

    fn build(self) -> Result<Service, String> {
        Ok(Service {
            repository: self.repository.ok_or("Repository required")?,
            cache: self.cache.unwrap_or_else(|| Arc::new(NoOpCache)),
        })
    }
}

// Usage
let service = ServiceBuilder::new()
    .with_repository(Arc::new(PostgresRepo::new()))
    .with_cache(Arc::new(RedisCache::new()))
    .build()?;
```

## Multiple Dependencies

```rust
struct Service<R, C, L>
where
    R: Repository,
    C: Cache,
    L: Logger,
{
    repository: R,
    cache: C,
    logger: L,
}

// Or with trait objects:
struct Service {
    repository: Arc<dyn Repository>,
    cache: Arc<dyn Cache>,
    logger: Arc<dyn Logger>,
}
```

## Common Trait Bounds

```rust
// Sync trait
trait Repository {
    fn find(&self, id: u32) -> Option<User>;
}

// Async trait (thread-safe)
#[async_trait]
trait Repository: Send + Sync {
    async fn find(&self, id: u32) -> Option<User>;
}

// Clone support
trait Repository: Clone {
    fn find(&self, id: u32) -> Option<User>;
}

// Debug support (useful for testing)
trait Repository: std::fmt::Debug {
    fn find(&self, id: u32) -> Option<User>;
}
```

## Error Handling

```rust
#[derive(Debug)]
enum Error {
    NotFound,
    Database(String),
    Cache(String),
}

#[async_trait]
trait Repository: Send + Sync {
    async fn find(&self, id: u32) -> Result<User, Error>;
}

// In service
async fn get_user(&self, id: u32) -> Result<User, Error> {
    self.repository.find(id).await
        .map_err(|e| Error::Database(e.to_string()))
}
```

## Common Pitfalls

### ❌ Forgetting Send + Sync for async

```rust
// Won't compile in async context
#[async_trait]
trait Repository {  // Missing Send + Sync
    async fn find(&self, id: u32) -> User;
}
```

### ✅ Correct

```rust
#[async_trait]
trait Repository: Send + Sync {
    async fn find(&self, id: u32) -> User;
}
```

### ❌ Trait object without Box/Arc

```rust
// Won't compile - size not known at compile time
struct Service {
    repository: dyn Repository,  // Error!
}
```

### ✅ Correct

```rust
struct Service {
    repository: Box<dyn Repository>,  // Or Arc<dyn Repository>
}
```

### ❌ Mutable methods in trait object

```rust
trait Repository {
    fn save(&mut self, user: User);  // Problematic with Arc<dyn>
}
```

### ✅ Use interior mutability

```rust
trait Repository {
    fn save(&self, user: User);  // Use Mutex/RwLock inside
}

struct PostgresRepo {
    pool: PgPool,  // Already uses internal mutability
}
```

## Quick Decision Tree

```
Need dependency injection?
├─ Types known at compile time?
│  └─ YES → Use generics <T: Trait>
│
├─ Need to choose at runtime?
│  ├─ Single owner?
│  │  └─ YES → Use Box<dyn Trait>
│  │
│  └─ Multiple owners / async?
│     └─ YES → Use Arc<dyn Trait>
│
└─ Testing?
   └─ Create mock implementations of trait
```

## Performance Comparison

| Pattern | Compile Time | Runtime | Binary Size | Flexibility |
|---------|-------------|---------|-------------|-------------|
| Generic `<T>` | Slower | Fastest | Larger | Compile-time only |
| `Box<dyn>` | Faster | Fast | Smaller | Runtime |
| `Arc<dyn>` | Faster | Fast* | Smaller | Runtime + Shared |

\* Small overhead from reference counting
