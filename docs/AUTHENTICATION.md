# TimesMan Authentication System

This document provides comprehensive guidance on using the JWT-based authentication system in TimesMan.

## Overview

TimesMan uses JSON Web Tokens (JWT) for stateless authentication with role-based access control. The system supports user registration, login, and secure access to protected resources.

## Table of Contents

- [Authentication Flow](#authentication-flow)
- [User Roles](#user-roles)
- [API Reference](#api-reference)
- [Client Implementation](#client-implementation)
- [Security Considerations](#security-considerations)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)

## Authentication Flow

### 1. User Registration

```
Client --> Server: Register(username, email, password)
Server --> Client: JWT Token + User Info
```

### 2. User Login

```
Client --> Server: Login(username, password)
Server --> Client: JWT Token + User Info
```

### 3. Accessing Protected Resources

```
Client --> Server: Request + Authorization Header
Server --> Client: Response (if authorized)
```

## User Roles

The system supports three user roles with different permission levels:

### Admin
- **Full access** to all data across all users
- Can view, create, modify, and delete any data
- Future: User management capabilities

### User (Default)
- **CRUD access** to their own data only
- Can create, read, update, and delete their own times, posts, and todos
- Cannot access other users' data

### ReadOnly
- **Read-only access** to their own data
- Can view their own times, posts, and todos
- Cannot create, modify, or delete any data

## API Reference

### gRPC Endpoints

#### Authentication Endpoints (No Auth Required)

##### Register User
```protobuf
rpc Register(RegisterRequest) returns (AuthResponse);

message RegisterRequest {
  string username = 1;
  string email = 2;
  string password = 3;
}
```

**Example Request:**
```json
{
  "username": "john_doe",
  "email": "john@example.com",
  "password": "secure_password_123"
}
```

##### Login User
```protobuf
rpc Login(LoginRequest) returns (AuthResponse);

message LoginRequest {
  string username = 1;
  string password = 2;
}
```

**Example Request:**
```json
{
  "username": "john_doe",
  "password": "secure_password_123"
}
```

##### Auth Response
```protobuf
message AuthResponse {
  string access_token = 1;     // JWT token
  string token_type = 2;       // "Bearer"
  uint64 expires_in = 3;       // Seconds until expiration
  UserInfo user = 4;           // User information
}

message UserInfo {
  string id = 1;               // UUID
  string username = 2;
  string email = 3;
  UserRole role = 4;
}

enum UserRole {
  USER_ROLE_ADMIN = 0;
  USER_ROLE_USER = 1;
  USER_ROLE_READ_ONLY = 2;
}
```

#### Protected Endpoints (Auth Required)

All other endpoints require authentication. Include the JWT token in the `authorization` metadata:

```
authorization: Bearer <jwt_token>
```

**Example Protected Endpoints:**
- `GetTimes()`
- `CreateTimes(TimesTitle)`
- `GetPosts(TimesId)`
- `CreatePost(CreatePostParams)`
- `GetTodos(TimesId)`
- `CreateTodo(CreateTodoParams)`

## Client Implementation

### Rust Client Example

```rust
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use timesman_grpc::grpc::times_man_client::TimesManClient;
use timesman_grpc::grpc::{LoginRequest, RegisterRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://localhost:50051").connect().await?;
    let mut client = TimesManClient::new(channel);

    // 1. Register a new user
    let register_request = tonic::Request::new(RegisterRequest {
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        password: "secure_password_123".to_string(),
    });

    let auth_response = client.register(register_request).await?;
    let token = auth_response.into_inner().access_token;

    // 2. Use token for authenticated requests
    let mut authenticated_request = tonic::Request::new(());
    authenticated_request.metadata_mut().insert(
        "authorization",
        MetadataValue::try_from(format!("Bearer {}", token))?
    );

    let times = client.get_times(authenticated_request).await?;
    println!("Times: {:?}", times.into_inner());

    Ok(())
}
```

### Python Client Example

```python
import grpc
import timesman_pb2
import timesman_pb2_grpc

def main():
    # Create gRPC channel
    channel = grpc.insecure_channel('localhost:50051')
    stub = timesman_pb2_grpc.TimesManStub(channel)

    # 1. Register user
    register_request = timesman_pb2.RegisterRequest(
        username="jane_doe",
        email="jane@example.com",
        password="secure_password_456"
    )
    
    auth_response = stub.Register(register_request)
    token = auth_response.access_token
    print(f"Received token: {token[:20]}...")

    # 2. Create metadata with token
    metadata = [('authorization', f'Bearer {token}')]

    # 3. Make authenticated request
    times_response = stub.GetTimes(
        timesman_pb2.google_dot_protobuf_dot_empty__pb2.Empty(),
        metadata=metadata
    )
    print(f"Times: {times_response}")

if __name__ == "__main__":
    main()
```

### JavaScript/Node.js Client Example

```javascript
const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');

const PROTO_PATH = './timesman.proto';
const packageDefinition = protoLoader.loadSync(PROTO_PATH);
const timesman = grpc.loadPackageDefinition(packageDefinition).timesman;

async function main() {
    const client = new timesman.TimesMan(
        'localhost:50051',
        grpc.credentials.createInsecure()
    );

    // 1. Register user
    const registerRequest = {
        username: 'bob_smith',
        email: 'bob@example.com',
        password: 'secure_password_789'
    };

    const authResponse = await new Promise((resolve, reject) => {
        client.register(registerRequest, (error, response) => {
            if (error) reject(error);
            else resolve(response);
        });
    });

    const token = authResponse.access_token;
    console.log(`Received token: ${token.substring(0, 20)}...`);

    // 2. Create metadata with token
    const metadata = new grpc.Metadata();
    metadata.add('authorization', `Bearer ${token}`);

    // 3. Make authenticated request
    const timesResponse = await new Promise((resolve, reject) => {
        client.getTimes({}, metadata, (error, response) => {
            if (error) reject(error);
            else resolve(response);
        });
    });

    console.log('Times:', timesResponse);
}

main().catch(console.error);
```

## Security Considerations

### Password Requirements
- **Minimum length**: 8 characters (recommended)
- **Complexity**: Use a mix of letters, numbers, and symbols
- **Avoid common passwords**: Dictionary words, personal information

### Token Security
- **Storage**: Store tokens securely (never in plain text files or logs)
- **Transmission**: Always use HTTPS/TLS in production
- **Expiration**: Tokens expire after 24 hours by default
- **Scope**: Tokens are scoped to the issuing server

### Best Practices
1. **Use HTTPS/TLS** for all communication in production
2. **Validate tokens** on every request
3. **Implement token refresh** for long-running applications
4. **Log security events** for monitoring
5. **Rate limit** authentication endpoints

### Security Headers
Always include the authorization header in the correct format:
```
authorization: Bearer <jwt_token>
```

**❌ Incorrect formats:**
- `authorization: <jwt_token>` (missing Bearer prefix)
- `Authorization: Bearer <jwt_token>` (wrong case in gRPC metadata)
- `bearer: <jwt_token>` (wrong header name)

## Testing

### Unit Tests

The authentication service includes comprehensive unit tests:

```bash
# Run authentication tests
cargo test -p timesman-server auth

# Run with output
cargo test -p timesman-server auth -- --nocapture
```

### Integration Testing

Test the full authentication flow:

```bash
# Start the server
cargo run -p timesman-server -- --config timesman-server/config.toml

# In another terminal, run integration tests
cargo test -p timesman-tools integration_tests
```

### Manual Testing with grpcurl

```bash
# 1. Register a user
grpcurl -plaintext -d '{
  "username": "testuser",
  "email": "test@example.com", 
  "password": "testpass123"
}' localhost:50051 timesman.TimesMan/Register

# 2. Login
grpcurl -plaintext -d '{
  "username": "testuser",
  "password": "testpass123"
}' localhost:50051 timesman.TimesMan/Login

# 3. Use token (replace TOKEN with actual token)
grpcurl -plaintext \
  -H "authorization: Bearer TOKEN" \
  localhost:50051 timesman.TimesMan/GetTimes
```

## Troubleshooting

### Common Errors

#### "Missing authorization header"
**Cause**: No authorization metadata included in request
**Solution**: Add `authorization: Bearer <token>` to request metadata

#### "Invalid authorization format"
**Cause**: Authorization header doesn't start with "Bearer "
**Solution**: Ensure format is exactly `Bearer <token>` with space after "Bearer"

#### "Token has expired"
**Cause**: JWT token is past its expiration time
**Solution**: Login again to get a new token

#### "Invalid credentials"
**Cause**: Wrong username/password or user doesn't exist
**Solution**: Check credentials or register if user doesn't exist

#### "User already exists"
**Cause**: Trying to register with existing username
**Solution**: Use different username or login instead

### Debug Steps

1. **Check server logs** for detailed error messages
2. **Verify token format** - should be a valid JWT
3. **Check token expiration** using jwt.io or similar tool
4. **Verify network connectivity** to the server
5. **Ensure proper metadata format** in gRPC requests

### Server Configuration

The authentication system can be configured via environment variables:

```bash
# JWT secret key (make this secure in production!)
export TIMESMAN_JWT_SECRET="your-super-secure-secret-key-here"

# Token expiration in hours (default: 24)
export TIMESMAN_TOKEN_EXPIRY_HOURS=24

# Server listen address
export TIMESMAN_LISTEN="0.0.0.0:50051"
```

### Creating Admin Users

For initial setup, you can create an admin user programmatically:

```rust
use timesman_server::AuthService;

#[tokio::main]
async fn main() {
    let auth_service = AuthService::new("your-secret-key");
    
    auth_service.create_admin_user(
        "admin",
        "admin@example.com", 
        "secure_admin_password"
    ).await.unwrap();
    
    println!("Admin user created successfully");
}
```

## Future Enhancements

### Planned Features
- **Token refresh mechanism** for seamless re-authentication
- **User management endpoints** for admins
- **Rate limiting** to prevent brute force attacks
- **Audit logging** for security monitoring
- **OAuth2 integration** for third-party authentication
- **Multi-factor authentication** support

### API Versioning
The current API is version 1.0. Future versions will maintain backward compatibility where possible.

## Support

For questions or issues:
1. Check this documentation
2. Review the [troubleshooting section](#troubleshooting)
3. Check server logs for error details
4. Create an issue in the project repository

---

**Last Updated**: December 2024  
**API Version**: 1.0  
**Authentication Version**: JWT with HS256