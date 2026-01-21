// Example: Refactoring Existing Code to Use Dependency Inversion
// ==============================================================
//
// This shows how to refactor code similar to your biz crate to use DI

use async_trait::async_trait;
use std::sync::Arc;

// Models
#[derive(Debug, Clone)]
struct User {
    id: String,
    email: String,
    password_hash: String,
    role: String,
}

#[derive(Debug)]
enum Error {
    NotFound,
    AlreadyExists,
    InvalidCredentials,
    Internal(String),
}

// ===================================================================
// BEFORE: Concrete dependencies (hard to test)
// ===================================================================

mod before {
    use super::*;

    // Service directly depends on concrete types
    pub struct AuthService {
        database_pool: String,  // Pretend this is sqlx::PgPool
        redis_url: String,      // Pretend this is redis::Client
    }

    impl AuthService {
        pub fn new(database_pool: String, redis_url: String) -> Self {
            Self {
                database_pool,
                redis_url,
            }
        }

        // ❌ Problem: Can't test without real database
        pub async fn login(&self, email: &str, password: &str) -> Result<String, Error> {
            // Direct database query - hard to mock
            println!("Querying database: {}", self.database_pool);

            // Direct Redis access - hard to mock
            println!("Checking Redis: {}", self.redis_url);

            // Business logic mixed with data access
            Ok("fake_token".to_string())
        }

        // ❌ Problem: Can't test without real database
        pub async fn register(&self, email: &str, password: &str) -> Result<User, Error> {
            println!("Inserting into database: {}", self.database_pool);
            Ok(User {
                id: "1".to_string(),
                email: email.to_string(),
                password_hash: password.to_string(),
                role: "user".to_string(),
            })
        }
    }
}

// ===================================================================
// AFTER: Abstract dependencies (easy to test)
// ===================================================================

mod after {
    use super::*;

    // 1. Define abstractions (traits)
    // ================================

    #[async_trait]
    pub trait UserRepository: Send + Sync {
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
        async fn create(&self, user: User) -> Result<User, Error>;
        async fn update(&self, user: User) -> Result<User, Error>;
    }

    #[async_trait]
    pub trait PasswordHasher: Send + Sync {
        fn hash(&self, password: &str) -> String;
        fn verify(&self, password: &str, hash: &str) -> bool;
    }

    #[async_trait]
    pub trait TokenService: Send + Sync {
        async fn generate(&self, user_id: &str) -> Result<String, Error>;
        async fn validate(&self, token: &str) -> Result<String, Error>;
    }

    #[async_trait]
    pub trait CacheService: Send + Sync {
        async fn get(&self, key: &str) -> Option<String>;
        async fn set(&self, key: &str, value: String, ttl_seconds: Option<u64>);
        async fn delete(&self, key: &str);
    }

    // 2. Service depends on abstractions
    // ===================================

    pub struct AuthService {
        repository: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
        token_service: Arc<dyn TokenService>,
        cache: Arc<dyn CacheService>,
    }

    impl AuthService {
        pub fn new(
            repository: Arc<dyn UserRepository>,
            hasher: Arc<dyn PasswordHasher>,
            token_service: Arc<dyn TokenService>,
            cache: Arc<dyn CacheService>,
        ) -> Self {
            Self {
                repository,
                hasher,
                token_service,
                cache,
            }
        }

        // ✅ Easy to test with mocks
        pub async fn login(&self, email: &str, password: &str) -> Result<String, Error> {
            // Check cache first
            let cache_key = format!("user:email:{}", email);
            if let Some(_cached_user) = self.cache.get(&cache_key).await {
                // In real code, deserialize and use cached user
            }

            // Find user
            let user = self
                .repository
                .find_by_email(email)
                .await?
                .ok_or(Error::InvalidCredentials)?;

            // Verify password
            if !self.hasher.verify(password, &user.password_hash) {
                return Err(Error::InvalidCredentials);
            }

            // Generate token
            let token = self.token_service.generate(&user.id).await?;

            // Cache the user
            self.cache.set(&cache_key, user.id.clone(), Some(3600)).await;

            Ok(token)
        }

        // ✅ Easy to test with mocks
        pub async fn register(&self, email: &str, password: &str) -> Result<User, Error> {
            // Check if user exists
            if let Some(_) = self.repository.find_by_email(email).await? {
                return Err(Error::AlreadyExists);
            }

            // Hash password
            let password_hash = self.hasher.hash(password);

            // Create user
            let user = User {
                id: uuid::Uuid::new_v4().to_string(),
                email: email.to_string(),
                password_hash,
                role: "user".to_string(),
            };

            // Save to database
            let created_user = self.repository.create(user).await?;

            Ok(created_user)
        }

        // ✅ New feature: Change password (easy to add)
        pub async fn change_password(
            &self,
            user_id: &str,
            old_password: &str,
            new_password: &str,
        ) -> Result<(), Error> {
            // This would be hard in the "before" version
            // but easy here because we have abstractions

            let user = self
                .repository
                .find_by_email(user_id)
                .await?
                .ok_or(Error::NotFound)?;

            if !self.hasher.verify(old_password, &user.password_hash) {
                return Err(Error::InvalidCredentials);
            }

            let new_hash = self.hasher.hash(new_password);
            let updated_user = User {
                password_hash: new_hash,
                ..user
            };

            self.repository.update(updated_user).await?;

            // Invalidate cache
            self.cache.delete(&format!("user:email:{}", user_id)).await;

            Ok(())
        }
    }

    // 3. Production implementations
    // ==============================

    pub struct PostgresUserRepository {
        pool_url: String, // In real code: sqlx::PgPool
    }

    #[async_trait]
    impl UserRepository for PostgresUserRepository {
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
            println!("PostgreSQL: Finding user by email: {}", email);
            // Real implementation would use sqlx
            Ok(None)
        }

        async fn create(&self, user: User) -> Result<User, Error> {
            println!("PostgreSQL: Creating user: {:?}", user);
            Ok(user)
        }

        async fn update(&self, user: User) -> Result<User, Error> {
            println!("PostgreSQL: Updating user: {:?}", user);
            Ok(user)
        }
    }

    pub struct BcryptHasher;

    #[async_trait]
    impl PasswordHasher for BcryptHasher {
        fn hash(&self, password: &str) -> String {
            format!("hashed_{}", password)
        }

        fn verify(&self, password: &str, hash: &str) -> bool {
            hash == &format!("hashed_{}", password)
        }
    }

    pub struct JwtTokenService {
        secret: String,
    }

    #[async_trait]
    impl TokenService for JwtTokenService {
        async fn generate(&self, user_id: &str) -> Result<String, Error> {
            Ok(format!("jwt_token_for_{}", user_id))
        }

        async fn validate(&self, token: &str) -> Result<String, Error> {
            Ok("user_id".to_string())
        }
    }

    pub struct RedisCache {
        url: String,
    }

    #[async_trait]
    impl CacheService for RedisCache {
        async fn get(&self, key: &str) -> Option<String> {
            println!("Redis GET: {}", key);
            None
        }

        async fn set(&self, key: &str, value: String, _ttl: Option<u64>) {
            println!("Redis SET: {} = {}", key, value);
        }

        async fn delete(&self, key: &str) {
            println!("Redis DEL: {}", key);
        }
    }

    // 4. Mock implementations for testing
    // ====================================

    pub struct MockUserRepository {
        users: std::sync::Mutex<Vec<User>>,
    }

    impl MockUserRepository {
        pub fn new() -> Self {
            Self {
                users: std::sync::Mutex::new(Vec::new()),
            }
        }

        pub fn with_user(user: User) -> Self {
            Self {
                users: std::sync::Mutex::new(vec![user]),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
            let users = self.users.lock().unwrap();
            Ok(users.iter().find(|u| u.email == email).cloned())
        }

        async fn create(&self, user: User) -> Result<User, Error> {
            let mut users = self.users.lock().unwrap();
            users.push(user.clone());
            Ok(user)
        }

        async fn update(&self, user: User) -> Result<User, Error> {
            let mut users = self.users.lock().unwrap();
            if let Some(existing) = users.iter_mut().find(|u| u.id == user.id) {
                *existing = user.clone();
                Ok(user)
            } else {
                Err(Error::NotFound)
            }
        }
    }

    pub struct MockPasswordHasher;

    #[async_trait]
    impl PasswordHasher for MockPasswordHasher {
        fn hash(&self, password: &str) -> String {
            format!("mock_hash_{}", password)
        }

        fn verify(&self, password: &str, hash: &str) -> bool {
            hash == &format!("mock_hash_{}", password)
        }
    }

    pub struct MockTokenService;

    #[async_trait]
    impl TokenService for MockTokenService {
        async fn generate(&self, user_id: &str) -> Result<String, Error> {
            Ok(format!("mock_token_{}", user_id))
        }

        async fn validate(&self, _token: &str) -> Result<String, Error> {
            Ok("mock_user_id".to_string())
        }
    }

    pub struct MockCache {
        cache: std::sync::Mutex<std::collections::HashMap<String, String>>,
    }

    impl MockCache {
        pub fn new() -> Self {
            Self {
                cache: std::sync::Mutex::new(std::collections::HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl CacheService for MockCache {
        async fn get(&self, key: &str) -> Option<String> {
            self.cache.lock().unwrap().get(key).cloned()
        }

        async fn set(&self, key: &str, value: String, _ttl: Option<u64>) {
            self.cache.lock().unwrap().insert(key.to_string(), value);
        }

        async fn delete(&self, key: &str) {
            self.cache.lock().unwrap().remove(key);
        }
    }

    // 5. Factory for easy setup
    // ==========================

    pub struct AuthServiceFactory;

    impl AuthServiceFactory {
        pub fn create_production(
            db_url: String,
            redis_url: String,
            jwt_secret: String,
        ) -> AuthService {
            let repository: Arc<dyn UserRepository> =
                Arc::new(PostgresUserRepository { pool_url: db_url });
            let hasher: Arc<dyn PasswordHasher> = Arc::new(BcryptHasher);
            let token_service: Arc<dyn TokenService> =
                Arc::new(JwtTokenService { secret: jwt_secret });
            let cache: Arc<dyn CacheService> = Arc::new(RedisCache { url: redis_url });

            AuthService::new(repository, hasher, token_service, cache)
        }

        pub fn create_test() -> AuthService {
            let repository: Arc<dyn UserRepository> = Arc::new(MockUserRepository::new());
            let hasher: Arc<dyn PasswordHasher> = Arc::new(MockPasswordHasher);
            let token_service: Arc<dyn TokenService> = Arc::new(MockTokenService);
            let cache: Arc<dyn CacheService> = Arc::new(MockCache::new());

            AuthService::new(repository, hasher, token_service, cache)
        }

        pub fn create_test_with_user(user: User) -> AuthService {
            let repository: Arc<dyn UserRepository> =
                Arc::new(MockUserRepository::with_user(user));
            let hasher: Arc<dyn PasswordHasher> = Arc::new(MockPasswordHasher);
            let token_service: Arc<dyn TokenService> = Arc::new(MockTokenService);
            let cache: Arc<dyn CacheService> = Arc::new(MockCache::new());

            AuthService::new(repository, hasher, token_service, cache)
        }
    }
}

// ===================================================================
// DEMONSTRATION & TESTS
// ===================================================================

#[tokio::main]
async fn main() {
    println!("=== Production Setup (AFTER) ===");
    let service = after::AuthServiceFactory::create_production(
        "postgresql://localhost".to_string(),
        "redis://localhost".to_string(),
        "secret".to_string(),
    );

    println!("\n=== Test Setup (AFTER) ===");
    let test_service = after::AuthServiceFactory::create_test();

    match test_service.register("test@example.com", "password123").await {
        Ok(user) => println!("Registered user: {:?}", user),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use after::*;

    #[tokio::test]
    async fn test_register_new_user() {
        let service = AuthServiceFactory::create_test();

        let result = service.register("new@example.com", "password123").await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, "new@example.com");
    }

    #[tokio::test]
    async fn test_register_duplicate_email() {
        let existing_user = User {
            id: "1".to_string(),
            email: "existing@example.com".to_string(),
            password_hash: "hash".to_string(),
            role: "user".to_string(),
        };

        let service = AuthServiceFactory::create_test_with_user(existing_user);

        let result = service.register("existing@example.com", "password").await;

        assert!(matches!(result, Err(Error::AlreadyExists)));
    }

    #[tokio::test]
    async fn test_login_success() {
        let user = User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "mock_hash_password123".to_string(),
            role: "user".to_string(),
        };

        let service = AuthServiceFactory::create_test_with_user(user);

        let result = service.login("test@example.com", "password123").await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(token.starts_with("mock_token_"));
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let user = User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "mock_hash_correct_password".to_string(),
            role: "user".to_string(),
        };

        let service = AuthServiceFactory::create_test_with_user(user);

        let result = service.login("test@example.com", "wrong_password").await;

        assert!(matches!(result, Err(Error::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let service = AuthServiceFactory::create_test();

        let result = service.login("nonexistent@example.com", "password").await;

        assert!(matches!(result, Err(Error::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_change_password_success() {
        let user = User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "mock_hash_old_password".to_string(),
            role: "user".to_string(),
        };

        let service = AuthServiceFactory::create_test_with_user(user);

        let result = service
            .change_password("test@example.com", "old_password", "new_password")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_change_password_wrong_old_password() {
        let user = User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "mock_hash_old_password".to_string(),
            role: "user".to_string(),
        };

        let service = AuthServiceFactory::create_test_with_user(user);

        let result = service
            .change_password("test@example.com", "wrong_old_password", "new_password")
            .await;

        assert!(matches!(result, Err(Error::InvalidCredentials)));
    }
}
