// Real-world Dependency Inversion Examples for Your Rust API
// ===========================================================
//
// This example shows how to apply DI patterns similar to your existing codebase

use std::sync::Arc;
use async_trait::async_trait;

// Example 1: Repository Pattern (Like your db crate)
// ==================================================

#[derive(Debug, Clone)]
struct User {
    id: String,
    email: String,
    name: String,
}

// Define the abstraction
#[async_trait]
trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn create(&self, user: User) -> Result<User, String>;
    async fn update(&self, user: User) -> Result<User, String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}

// Concrete implementation #1: PostgreSQL
struct PostgresUserRepository {
    // In real code, this would be sqlx::PgPool
    connection_string: String,
}

impl PostgresUserRepository {
    fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, String> {
        // Simulate database query
        println!("Querying PostgreSQL for user id: {}", id);
        Ok(Some(User {
            id: id.to_string(),
            email: "user@example.com".to_string(),
            name: "John Doe".to_string(),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        println!("Querying PostgreSQL for user email: {}", email);
        Ok(None)
    }

    async fn create(&self, user: User) -> Result<User, String> {
        println!("Creating user in PostgreSQL: {:?}", user);
        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, String> {
        println!("Updating user in PostgreSQL: {:?}", user);
        Ok(user)
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        println!("Deleting user from PostgreSQL: {}", id);
        Ok(())
    }
}

// Concrete implementation #2: Mock (for testing)
struct MockUserRepository {
    users: std::sync::Mutex<Vec<User>>,
}

impl MockUserRepository {
    fn new() -> Self {
        Self {
            users: std::sync::Mutex::new(Vec::new()),
        }
    }

    fn with_users(users: Vec<User>) -> Self {
        Self {
            users: std::sync::Mutex::new(users),
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let users = self.users.lock().unwrap();
        Ok(users.iter().find(|u| u.id == id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let users = self.users.lock().unwrap();
        Ok(users.iter().find(|u| u.email == email).cloned())
    }

    async fn create(&self, user: User) -> Result<User, String> {
        let mut users = self.users.lock().unwrap();
        users.push(user.clone());
        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, String> {
        let mut users = self.users.lock().unwrap();
        if let Some(existing) = users.iter_mut().find(|u| u.id == user.id) {
            *existing = user.clone();
            Ok(user)
        } else {
            Err("User not found".to_string())
        }
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        users.retain(|u| u.id != id);
        Ok(())
    }
}

// Example 2: Service Layer with Multiple Dependencies
// ===================================================

#[async_trait]
trait CacheService: Send + Sync {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: String, ttl_seconds: Option<u64>);
    async fn delete(&self, key: &str);
}

struct RedisCacheService {
    redis_url: String,
}

#[async_trait]
impl CacheService for RedisCacheService {
    async fn get(&self, key: &str) -> Option<String> {
        println!("Getting from Redis: {}", key);
        None
    }

    async fn set(&self, key: &str, value: String, _ttl_seconds: Option<u64>) {
        println!("Setting in Redis: {} = {}", key, value);
    }

    async fn delete(&self, key: &str) {
        println!("Deleting from Redis: {}", key);
    }
}

struct InMemoryCacheService {
    cache: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

impl InMemoryCacheService {
    fn new() -> Self {
        Self {
            cache: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait]
impl CacheService for InMemoryCacheService {
    async fn get(&self, key: &str) -> Option<String> {
        self.cache.lock().unwrap().get(key).cloned()
    }

    async fn set(&self, key: &str, value: String, _ttl_seconds: Option<u64>) {
        self.cache.lock().unwrap().insert(key.to_string(), value);
    }

    async fn delete(&self, key: &str) {
        self.cache.lock().unwrap().remove(key);
    }
}

// Business logic service that depends on repository and cache
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<dyn CacheService>,
}

impl UserService {
    fn new(repository: Arc<dyn UserRepository>, cache: Arc<dyn CacheService>) -> Self {
        Self { repository, cache }
    }

    async fn get_user(&self, id: &str) -> Result<Option<User>, String> {
        // Try cache first
        let cache_key = format!("user:{}", id);
        if let Some(_cached) = self.cache.get(&cache_key).await {
            println!("Cache hit!");
            // In real code, deserialize from cache
        }

        // Fetch from repository
        let user = self.repository.find_by_id(id).await?;

        // Update cache
        if let Some(ref user) = user {
            self.cache.set(&cache_key, user.id.clone(), Some(3600)).await;
        }

        Ok(user)
    }

    async fn create_user(&self, user: User) -> Result<User, String> {
        // Check if email exists
        if let Some(_) = self.repository.find_by_email(&user.email).await? {
            return Err("Email already exists".to_string());
        }

        // Create user
        let created_user = self.repository.create(user).await?;

        // Cache the new user
        let cache_key = format!("user:{}", created_user.id);
        self.cache.set(&cache_key, created_user.id.clone(), Some(3600)).await;

        Ok(created_user)
    }

    async fn delete_user(&self, id: &str) -> Result<(), String> {
        // Delete from repository
        self.repository.delete(id).await?;

        // Invalidate cache
        let cache_key = format!("user:{}", id);
        self.cache.delete(&cache_key).await;

        Ok(())
    }
}

// Example 3: Axum Handler with Dependency Injection
// =================================================

use std::sync::Mutex;

// Application state that holds dependencies
struct AppState {
    user_service: Arc<UserService>,
}

// In Axum, you would use this like:
// async fn get_user_handler(
//     State(state): State<Arc<AppState>>,
//     Path(id): Path<String>,
// ) -> Result<Json<User>, StatusCode> {
//     let user = state.user_service.get_user(&id).await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//     user.ok_or(StatusCode::NOT_FOUND).map(Json)
// }

// Example 4: Factory Pattern for Creating Services
// ================================================

struct ServiceFactory {
    database_url: String,
    redis_url: Option<String>,
    use_cache: bool,
}

impl ServiceFactory {
    fn new(database_url: String) -> Self {
        Self {
            database_url,
            redis_url: None,
            use_cache: false,
        }
    }

    fn with_redis(mut self, redis_url: String) -> Self {
        self.redis_url = Some(redis_url);
        self.use_cache = true;
        self
    }

    fn build_user_service(self) -> UserService {
        // Create repository
        let repository: Arc<dyn UserRepository> =
            Arc::new(PostgresUserRepository::new(self.database_url));

        // Create cache
        let cache: Arc<dyn CacheService> = if self.use_cache {
            if let Some(redis_url) = self.redis_url {
                Arc::new(RedisCacheService { redis_url })
            } else {
                Arc::new(InMemoryCacheService::new())
            }
        } else {
            Arc::new(InMemoryCacheService::new())
        };

        UserService::new(repository, cache)
    }

    fn build_test_user_service(mock_users: Vec<User>) -> UserService {
        let repository: Arc<dyn UserRepository> =
            Arc::new(MockUserRepository::with_users(mock_users));
        let cache: Arc<dyn CacheService> =
            Arc::new(InMemoryCacheService::new());

        UserService::new(repository, cache)
    }
}

// Example 5: Extension Trait Pattern
// ===================================

// This pattern allows you to add methods to existing types

trait UserRepositoryExt: UserRepository {
    async fn find_active_users(&self) -> Result<Vec<User>, String> {
        // Default implementation
        Ok(vec![])
    }

    async fn count_users(&self) -> Result<usize, String> {
        Ok(0)
    }
}

// Any type implementing UserRepository automatically gets these methods
impl<T: UserRepository + ?Sized> UserRepositoryExt for T {}

// DEMONSTRATION
// =============

#[tokio::main]
async fn main() {
    println!("=== Production Setup ===");
    let service = ServiceFactory::new("postgresql://localhost".to_string())
        .with_redis("redis://localhost".to_string())
        .build_user_service();

    let user = service.get_user("123").await.unwrap();
    println!("User: {:?}", user);

    println!("\n=== Test Setup ===");
    let test_users = vec![
        User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        },
    ];
    let test_service = ServiceFactory::build_test_user_service(test_users);

    let user = test_service.get_user("1").await.unwrap();
    println!("Test User: {:?}", user);

    println!("\n=== Creating new user ===");
    let new_user = User {
        id: "2".to_string(),
        email: "new@example.com".to_string(),
        name: "New User".to_string(),
    };
    let created = test_service.create_user(new_user).await.unwrap();
    println!("Created: {:?}", created);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_service_get_user() {
        let mock_users = vec![User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        }];

        let service = ServiceFactory::build_test_user_service(mock_users);

        let user = service.get_user("1").await.unwrap();
        assert!(user.is_some());
        assert_eq!(user.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_user_service_create_duplicate_email() {
        let existing_user = User {
            id: "1".to_string(),
            email: "existing@example.com".to_string(),
            name: "Existing".to_string(),
        };

        let service = ServiceFactory::build_test_user_service(vec![existing_user]);

        let new_user = User {
            id: "2".to_string(),
            email: "existing@example.com".to_string(),
            name: "New".to_string(),
        };

        let result = service.create_user(new_user).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email already exists");
    }

    #[tokio::test]
    async fn test_cache_invalidation_on_delete() {
        let mock_users = vec![User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "Test".to_string(),
        }];

        let service = ServiceFactory::build_test_user_service(mock_users);

        // Get user to populate cache
        let _ = service.get_user("1").await.unwrap();

        // Delete user
        service.delete_user("1").await.unwrap();

        // Verify user is deleted
        let user = service.get_user("1").await.unwrap();
        assert!(user.is_none());
    }
}
