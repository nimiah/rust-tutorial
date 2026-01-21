// Example 1: Basic Trait-based Dependency Inversion
// ===================================================

use std::collections::HashMap;

// 1. Define the abstraction (trait)
trait UserRepository {
    fn find_by_id(&self, id: u32) -> Option<String>;
    fn save(&mut self, id: u32, name: String);
}

// 2. Concrete implementation #1: In-memory repository
struct InMemoryUserRepository {
    users: HashMap<u32, String>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: u32) -> Option<String> {
        self.users.get(&id).cloned()
    }

    fn save(&mut self, id: u32, name: String) {
        self.users.insert(id, name);
    }
}

// 3. Concrete implementation #2: Mock repository (for testing)
struct MockUserRepository {
    should_fail: bool,
}

impl MockUserRepository {
    fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

impl UserRepository for MockUserRepository {
    fn find_by_id(&self, _id: u32) -> Option<String> {
        if self.should_fail {
            None
        } else {
            Some("Mock User".to_string())
        }
    }

    fn save(&mut self, _id: u32, _name: String) {
        // Mock implementation does nothing
    }
}

// 4. Service that depends on the abstraction (not concrete implementation)
struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    fn new(repository: R) -> Self {
        Self { repository }
    }

    fn get_user(&self, id: u32) -> Option<String> {
        self.repository.find_by_id(id)
    }

    fn create_user(&mut self, id: u32, name: String) {
        self.repository.save(id, name);
    }
}

// Example 2: Using Box<dyn Trait> for Runtime Polymorphism
// =========================================================

struct DynamicUserService {
    repository: Box<dyn UserRepository>,
}

impl DynamicUserService {
    fn new(repository: Box<dyn UserRepository>) -> Self {
        Self { repository }
    }

    fn get_user(&self, id: u32) -> Option<String> {
        self.repository.find_by_id(id)
    }
}

// Example 3: Using Arc<dyn Trait> for Thread-Safe Shared Ownership
// =================================================================

use std::sync::{Arc, Mutex};

// Thread-safe version of the trait
trait ThreadSafeUserRepository: Send + Sync {
    fn find_by_id(&self, id: u32) -> Option<String>;
    fn save(&self, id: u32, name: String);
}

struct ThreadSafeInMemoryRepository {
    users: Mutex<HashMap<u32, String>>,
}

impl ThreadSafeInMemoryRepository {
    fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

impl ThreadSafeUserRepository for ThreadSafeInMemoryRepository {
    fn find_by_id(&self, id: u32) -> Option<String> {
        self.users.lock().unwrap().get(&id).cloned()
    }

    fn save(&self, id: u32, name: String) {
        self.users.lock().unwrap().insert(id, name);
    }
}

struct ThreadSafeUserService {
    repository: Arc<dyn ThreadSafeUserRepository>,
}

impl ThreadSafeUserService {
    fn new(repository: Arc<dyn ThreadSafeUserRepository>) -> Self {
        Self { repository }
    }

    fn get_user(&self, id: u32) -> Option<String> {
        self.repository.find_by_id(id)
    }
}

// Example 4: Async Trait Pattern
// ===============================

use async_trait::async_trait;

#[async_trait]
trait AsyncUserRepository: Send + Sync {
    async fn find_by_id(&self, id: u32) -> Option<String>;
    async fn save(&self, id: u32, name: String);
}

struct AsyncInMemoryRepository {
    users: Mutex<HashMap<u32, String>>,
}

impl AsyncInMemoryRepository {
    fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AsyncUserRepository for AsyncInMemoryRepository {
    async fn find_by_id(&self, id: u32) -> Option<String> {
        // Simulate async operation
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        self.users.lock().unwrap().get(&id).cloned()
    }

    async fn save(&self, id: u32, name: String) {
        // Simulate async operation
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        self.users.lock().unwrap().insert(id, name);
    }
}

struct AsyncUserService {
    repository: Arc<dyn AsyncUserRepository>,
}

impl AsyncUserService {
    fn new(repository: Arc<dyn AsyncUserRepository>) -> Self {
        Self { repository }
    }

    async fn get_user(&self, id: u32) -> Option<String> {
        self.repository.find_by_id(id).await
    }

    async fn create_user(&self, id: u32, name: String) {
        self.repository.save(id, name).await;
    }
}

// Example 5: Builder Pattern with Dependency Injection
// ====================================================

struct UserServiceBuilder<R> {
    repository: Option<R>,
}

impl<R: UserRepository> UserServiceBuilder<R> {
    fn new() -> Self {
        Self { repository: None }
    }

    fn with_repository(mut self, repository: R) -> Self {
        self.repository = Some(repository);
        self
    }

    fn build(self) -> Result<UserService<R>, String> {
        Ok(UserService::new(
            self.repository.ok_or("Repository not set")?,
        ))
    }
}

// Example 6: Multiple Dependencies
// =================================

trait Logger {
    fn log(&self, message: &str);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[LOG] {}", message);
    }
}

struct MockLogger {
    messages: Mutex<Vec<String>>,
}

impl MockLogger {
    fn new() -> Self {
        Self {
            messages: Mutex::new(Vec::new()),
        }
    }

    fn get_messages(&self) -> Vec<String> {
        self.messages.lock().unwrap().clone()
    }
}

impl Logger for MockLogger {
    fn log(&self, message: &str) {
        self.messages.lock().unwrap().push(message.to_string());
    }
}

// Service with multiple dependencies
struct AdvancedUserService<R: UserRepository, L: Logger> {
    repository: R,
    logger: L,
}

impl<R: UserRepository, L: Logger> AdvancedUserService<R, L> {
    fn new(repository: R, logger: L) -> Self {
        Self { repository, logger }
    }

    fn get_user(&self, id: u32) -> Option<String> {
        self.logger.log(&format!("Fetching user with id: {}", id));
        let user = self.repository.find_by_id(id);
        if user.is_some() {
            self.logger.log("User found");
        } else {
            self.logger.log("User not found");
        }
        user
    }
}

// DEMONSTRATION
// =============

fn main() {
    println!("=== Example 1: Static Dispatch with Generics ===");
    let repo = InMemoryUserRepository::new();
    let mut service = UserService::new(repo);
    service.create_user(1, "Alice".to_string());
    println!("User: {:?}", service.get_user(1));

    println!("\n=== Example 2: Dynamic Dispatch with Box<dyn Trait> ===");
    let repo: Box<dyn UserRepository> = Box::new(InMemoryUserRepository::new());
    let service = DynamicUserService::new(repo);
    println!("User: {:?}", service.get_user(1));

    println!("\n=== Example 3: Mock for Testing ===");
    let mock_repo = MockUserRepository::new(false);
    let service = UserService::new(mock_repo);
    println!("Mock User: {:?}", service.get_user(999));

    println!("\n=== Example 4: Thread-Safe with Arc ===");
    let repo = Arc::new(ThreadSafeInMemoryRepository::new());
    repo.save(1, "Bob".to_string());
    let service = ThreadSafeUserService::new(repo);
    println!("User: {:?}", service.get_user(1));

    println!("\n=== Example 5: Builder Pattern ===");
    let repo = InMemoryUserRepository::new();
    let service = UserServiceBuilder::new()
        .with_repository(repo)
        .build()
        .unwrap();
    println!("User: {:?}", service.get_user(1));

    println!("\n=== Example 6: Multiple Dependencies ===");
    let repo = InMemoryUserRepository::new();
    let logger = ConsoleLogger;
    let mut service = AdvancedUserService::new(repo, logger);
    service.create_user(1, "Charlie".to_string());
    service.get_user(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_mock_repository() {
        let mock_repo = MockUserRepository::new(false);
        let service = UserService::new(mock_repo);

        let user = service.get_user(1);
        assert_eq!(user, Some("Mock User".to_string()));
    }

    #[test]
    fn test_with_mock_logger() {
        let repo = InMemoryUserRepository::new();
        let logger = MockLogger::new();
        let service = AdvancedUserService::new(repo, logger);

        service.get_user(999);

        let messages = service.logger.get_messages();
        assert!(messages.contains(&"Fetching user with id: 999".to_string()));
        assert!(messages.contains(&"User not found".to_string()));
    }

    #[tokio::test]
    async fn test_async_repository() {
        let repo = Arc::new(AsyncInMemoryRepository::new());
        let service = AsyncUserService::new(repo.clone());

        repo.save(1, "Test User".to_string()).await;
        let user = service.get_user(1).await;

        assert_eq!(user, Some("Test User".to_string()));
    }
}
