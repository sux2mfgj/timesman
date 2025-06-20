# TimesMan Authentication Quick Reference

## Quick Start

### 1. Start Server
```bash
cargo run -p timesman-server -- --config timesman-server/config.toml
```

### 2. Register User
```bash
grpcurl -plaintext -d '{
  "username": "myuser",
  "email": "user@example.com",
  "password": "mypassword123"
}' localhost:50051 timesman.TimesMan/Register
```

### 3. Login
```bash
grpcurl -plaintext -d '{
  "username": "myuser", 
  "password": "mypassword123"
}' localhost:50051 timesman.TimesMan/Login
```

### 4. Use Token
```bash
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_JWT_TOKEN_HERE" \
  localhost:50051 timesman.TimesMan/GetTimes
```

## User Roles

| Role | Permissions |
|------|-------------|
| **Admin** | Full access to all data |
| **User** | CRUD on own data only |
| **ReadOnly** | Read-only on own data |

## Authentication Endpoints

| Endpoint | Auth Required | Description |
|----------|---------------|-------------|
| `Register` | ❌ | Create new user account |
| `Login` | ❌ | Authenticate and get token |
| `GetTimes` | ✅ | Get user's time entries |
| `CreateTimes` | ✅ | Create new time entry |
| All others | ✅ | Require valid JWT token |

## Token Format

```
authorization: Bearer <jwt_token>
```

**⚠️ Important**: 
- Must include "Bearer " prefix
- Use lowercase "authorization" in gRPC metadata
- Token expires after 24 hours

## Common Error Codes

| Error | Code | Solution |
|-------|------|----------|
| Missing auth header | `UNAUTHENTICATED` | Add authorization metadata |
| Invalid token format | `UNAUTHENTICATED` | Check "Bearer " prefix |
| Token expired | `UNAUTHENTICATED` | Login again for new token |
| Wrong credentials | `UNAUTHENTICATED` | Check username/password |
| User exists | `INVALID_ARGUMENT` | Use different username |

## Code Examples

### Rust
```rust
// Register
let request = tonic::Request::new(RegisterRequest {
    username: "user".to_string(),
    email: "user@example.com".to_string(), 
    password: "password".to_string(),
});
let response = client.register(request).await?;
let token = response.into_inner().access_token;

// Authenticated request
let mut request = tonic::Request::new(());
request.metadata_mut().insert(
    "authorization",
    MetadataValue::try_from(format!("Bearer {}", token))?
);
let times = client.get_times(request).await?;
```

### Python
```python
# Register
auth_response = stub.Register(timesman_pb2.RegisterRequest(
    username="user",
    email="user@example.com",
    password="password"
))
token = auth_response.access_token

# Authenticated request
metadata = [('authorization', f'Bearer {token}')]
times = stub.GetTimes(Empty(), metadata=metadata)
```

### JavaScript
```javascript
// Register
const authResponse = await client.register({
    username: 'user',
    email: 'user@example.com', 
    password: 'password'
});
const token = authResponse.access_token;

// Authenticated request
const metadata = new grpc.Metadata();
metadata.add('authorization', `Bearer ${token}`);
const times = await client.getTimes({}, metadata);
```

## Security Checklist

- ✅ Use HTTPS/TLS in production
- ✅ Store tokens securely 
- ✅ Set strong passwords
- ✅ Handle token expiration
- ✅ Validate server certificates
- ✅ Don't log tokens
- ✅ Use environment variables for secrets

## Testing Commands

```bash
# Unit tests
cargo test -p timesman-server auth

# Integration tests  
cargo test -p timesman-tools integration

# Manual testing
grpcurl -plaintext localhost:50051 list
grpcurl -plaintext localhost:50051 describe timesman.TimesMan
```

## Environment Variables

```bash
export TIMESMAN_JWT_SECRET="your-secret-key"
export TIMESMAN_TOKEN_EXPIRY_HOURS=24
export TIMESMAN_LISTEN="0.0.0.0:50051"
```

---
📖 **Full Documentation**: See [AUTHENTICATION.md](./AUTHENTICATION.md) for complete details