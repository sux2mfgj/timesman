use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use timesman_type::{
    AuthResponse, Claims, LoginRequest, RegisterRequest, User, UserInfo, UserId, UserRole,
};

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    UserAlreadyExists,
    UserNotFound,
    TokenExpired,
    InvalidToken,
    InternalError(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::UserAlreadyExists => write!(f, "User already exists"),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::TokenExpired => write!(f, "Token has expired"),
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AuthError {}

pub struct AuthService {
    // In-memory user store for now - in production this would be a database
    users: Arc<RwLock<HashMap<String, User>>>,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: String,
    token_expiry_hours: i64,
}

impl AuthService {
    pub fn new(secret: &str) -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            issuer: "timesman-server".to_string(),
            token_expiry_hours: 24, // 24 hours by default
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse, AuthError> {
        let mut users = self.users.write().await;

        // Check if user already exists
        if users.contains_key(&request.username) {
            return Err(AuthError::UserAlreadyExists);
        }

        // Hash the password
        let password_hash = self.hash_password(&request.password)?;

        // Create new user
        let user = User {
            id: Uuid::new_v4(),
            username: request.username.clone(),
            email: request.email,
            password_hash,
            role: UserRole::User, // Default role
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            is_active: true,
        };

        // Store user
        users.insert(request.username, user.clone());

        // Generate token
        let token = self.generate_token(&user)?;

        Ok(AuthResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: (self.token_expiry_hours * 3600) as usize,
            user: UserInfo {
                id: user.id,
                username: user.username,
                email: user.email,
                role: user.role,
            },
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, AuthError> {
        let users = self.users.read().await;

        // Find user
        let user = users
            .get(&request.username)
            .ok_or(AuthError::InvalidCredentials)?;

        // Check if user is active
        if !user.is_active {
            return Err(AuthError::InvalidCredentials);
        }

        // Verify password
        if !self.verify_password(&request.password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Generate token
        let token = self.generate_token(user)?;

        Ok(AuthResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: (self.token_expiry_hours * 3600) as usize,
            user: UserInfo {
                id: user.id,
                username: user.username.clone(),
                email: user.email.clone(),
                role: user.role.clone(),
            },
        })
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.issuer]);

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|_| AuthError::InvalidToken)?;

        // Check if token is expired
        let now = Utc::now().timestamp() as usize;
        if token_data.claims.exp < now {
            return Err(AuthError::TokenExpired);
        }

        Ok(token_data.claims)
    }

    pub async fn get_user_by_id(&self, user_id: &UserId) -> Result<User, AuthError> {
        let users = self.users.read().await;
        
        for user in users.values() {
            if user.id == *user_id {
                return Ok(user.clone());
            }
        }
        
        Err(AuthError::UserNotFound)
    }

    pub async fn create_admin_user(&self, username: &str, email: &str, password: &str) -> Result<(), AuthError> {
        let mut users = self.users.write().await;

        // Check if user already exists
        if users.contains_key(username) {
            return Err(AuthError::UserAlreadyExists);
        }

        // Hash the password
        let password_hash = self.hash_password(password)?;

        // Create admin user
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            role: UserRole::Admin,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            is_active: true,
        };

        users.insert(username.to_string(), user);
        Ok(())
    }

    fn generate_token(&self, user: &User) -> Result<String, AuthError> {
        let now = Utc::now();
        let expiry = now + Duration::hours(self.token_expiry_hours);

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            role: user.role.clone(),
            exp: expiry.timestamp() as usize,
            iat: now.timestamp() as usize,
            iss: self.issuer.clone(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::InternalError(format!("Failed to generate token: {}", e)))
    }

    fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AuthError::InternalError(format!("Failed to hash password: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AuthError::InternalError(format!("Failed to parse password hash: {}", e)))?;

        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_and_login() {
        let auth_service = AuthService::new("test-secret-key");

        // Test registration
        let register_request = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword".to_string(),
        };

        let auth_response = auth_service.register(register_request).await.unwrap();
        assert_eq!(auth_response.user.username, "testuser");
        assert_eq!(auth_response.user.email, "test@example.com");
        assert_eq!(auth_response.user.role, UserRole::User);
        assert!(!auth_response.access_token.is_empty());

        // Test login
        let login_request = LoginRequest {
            username: "testuser".to_string(),
            password: "testpassword".to_string(),
        };

        let login_response = auth_service.login(login_request).await.unwrap();
        assert_eq!(login_response.user.username, "testuser");
        assert!(!login_response.access_token.is_empty());

        // Test token validation
        let claims = auth_service.validate_token(&login_response.access_token).unwrap();
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.role, UserRole::User);
    }

    #[tokio::test]
    async fn test_duplicate_registration() {
        let auth_service = AuthService::new("test-secret-key");

        let register_request = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword".to_string(),
        };

        // First registration should succeed
        auth_service.register(register_request.clone()).await.unwrap();

        // Second registration should fail
        let result = auth_service.register(register_request).await;
        assert!(matches!(result, Err(AuthError::UserAlreadyExists)));
    }

    #[tokio::test]
    async fn test_invalid_login() {
        let auth_service = AuthService::new("test-secret-key");

        // Test login with non-existent user
        let login_request = LoginRequest {
            username: "nonexistent".to_string(),
            password: "password".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(matches!(result, Err(AuthError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_create_admin_user() {
        let auth_service = AuthService::new("test-secret-key");

        // Create admin user
        auth_service
            .create_admin_user("admin", "admin@example.com", "adminpass")
            .await
            .unwrap();

        // Test admin login
        let login_request = LoginRequest {
            username: "admin".to_string(),
            password: "adminpass".to_string(),
        };

        let login_response = auth_service.login(login_request).await.unwrap();
        assert_eq!(login_response.user.role, UserRole::Admin);
    }
}