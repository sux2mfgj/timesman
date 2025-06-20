# TimesMan Authentication Migration Guide

This guide helps existing TimesMan users migrate to the new JWT-based authentication system.

## Overview

TimesMan now requires authentication for all data operations. This change improves security by ensuring users can only access their own data.

## What Changed

### Before (v1.0)
- No authentication required
- All users could access all data
- No user management

### After (v2.0)
- JWT-based authentication required
- Users can only access their own data  
- Role-based access control
- Secure password storage

## Breaking Changes

⚠️ **Important**: All existing clients must be updated to include authentication.

### API Changes
1. **New Required Header**: All requests (except Register/Login) must include:
   ```
   authorization: Bearer <jwt_token>
   ```

2. **New Endpoints**:
   - `Register(RegisterRequest) -> AuthResponse`
   - `Login(LoginRequest) -> AuthResponse`

3. **Error Responses**: New `UNAUTHENTICATED` error codes for auth failures

### Data Isolation
- Existing data remains accessible but will be migrated to user ownership
- Admin users can access all data for migration purposes

## Migration Steps

### Step 1: Update Server

1. **Update TimesMan server** to the latest version:
   ```bash
   git pull origin main
   cargo build -p timesman-server
   ```

2. **Set JWT secret** (important for security):
   ```bash
   export TIMESMAN_JWT_SECRET="your-secure-secret-key-here"
   ```

3. **Start server**:
   ```bash
   cargo run -p timesman-server -- --config timesman-server/config.toml
   ```

### Step 2: Create User Accounts

Create accounts for all existing users:

```bash
# Create admin account (for data migration)
grpcurl -plaintext -d '{
  "username": "admin",
  "email": "admin@yourcompany.com",
  "password": "secure_admin_password"
}' localhost:50051 timesman.TimesMan/Register

# Create regular user accounts
grpcurl -plaintext -d '{
  "username": "john_doe", 
  "email": "john@yourcompany.com",
  "password": "johns_password"
}' localhost:50051 timesman.TimesMan/Register
```

### Step 3: Update Client Code

#### Before (No Auth)
```rust
let mut client = TimesManClient::new(channel);
let times = client.get_times(tonic::Request::new(())).await?;
```

#### After (With Auth)
```rust
let mut client = TimesManClient::new(channel);

// 1. Login to get token
let login_request = tonic::Request::new(LoginRequest {
    username: "john_doe".to_string(),
    password: "johns_password".to_string(),
});
let auth_response = client.login(login_request).await?;
let token = auth_response.into_inner().access_token;

// 2. Add token to requests
let mut request = tonic::Request::new(());
request.metadata_mut().insert(
    "authorization",
    MetadataValue::try_from(format!("Bearer {}", token))?
);
let times = client.get_times(request).await?;
```

### Step 4: Data Migration (Optional)

If you need to migrate existing data to specific users:

1. **Login as admin**:
   ```bash
   grpcurl -plaintext -d '{"username": "admin", "password": "secure_admin_password"}' \
     localhost:50051 timesman.TimesMan/Login
   ```

2. **Export existing data**:
   ```bash
   grpcurl -plaintext -H "authorization: Bearer ADMIN_TOKEN" \
     localhost:50051 timesman.TimesMan/GetTimes > existing_data.json
   ```

3. **Reassign data ownership** (future feature - manual for now)

## Client Update Examples

### Rust Client Migration

**Before**:
```rust
use timesman_grpc::grpc::times_man_client::TimesManClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://localhost:50051").connect().await?;
    let mut client = TimesManClient::new(channel);
    
    // Direct access - no auth needed
    let times = client.get_times(tonic::Request::new(())).await?;
    println!("Times: {:?}", times);
    
    Ok(())
}
```

**After**:
```rust
use timesman_grpc::grpc::times_man_client::TimesManClient;
use timesman_grpc::grpc::LoginRequest;
use tonic::metadata::MetadataValue;

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://localhost:50051").connect().await?;
    let mut client = TimesManClient::new(channel);
    
    // 1. Authenticate first
    let login_request = tonic::Request::new(LoginRequest {
        username: "john_doe".to_string(),
        password: "johns_password".to_string(),
    });
    let auth_response = client.login(login_request).await?;
    let token = auth_response.into_inner().access_token;
    
    // 2. Include token in requests
    let mut request = tonic::Request::new(());
    request.metadata_mut().insert(
        "authorization",
        MetadataValue::try_from(format!("Bearer {}", token))?
    );
    let times = client.get_times(request).await?;
    println!("Times: {:?}", times);
    
    Ok(())
}
```

### Python Client Migration

**Before**:
```python
import grpc
import timesman_pb2_grpc

channel = grpc.insecure_channel('localhost:50051')
stub = timesman_pb2_grpc.TimesManStub(channel)

# Direct access
times = stub.GetTimes(timesman_pb2.google_dot_protobuf_dot_empty__pb2.Empty())
print(times)
```

**After**:
```python
import grpc
import timesman_pb2
import timesman_pb2_grpc

channel = grpc.insecure_channel('localhost:50051')
stub = timesman_pb2_grpc.TimesManStub(channel)

# 1. Login first
login_request = timesman_pb2.LoginRequest(
    username="john_doe",
    password="johns_password"
)
auth_response = stub.Login(login_request)
token = auth_response.access_token

# 2. Include token in metadata
metadata = [('authorization', f'Bearer {token}')]
times = stub.GetTimes(
    timesman_pb2.google_dot_protobuf_dot_empty__pb2.Empty(),
    metadata=metadata
)
print(times)
```

## Testing Migration

### Verify Authentication Works
```bash
# Should succeed
grpcurl -plaintext -d '{"username": "john_doe", "password": "johns_password"}' \
  localhost:50051 timesman.TimesMan/Login

# Should fail with UNAUTHENTICATED
grpcurl -plaintext localhost:50051 timesman.TimesMan/GetTimes

# Should succeed with token
grpcurl -plaintext -H "authorization: Bearer YOUR_TOKEN" \
  localhost:50051 timesman.TimesMan/GetTimes
```

### Verify Data Isolation
1. Create two user accounts
2. Login as user A, create some data
3. Login as user B, verify you can't see user A's data
4. Login as admin, verify you can see all data

## Common Migration Issues

### Issue: "Missing authorization header"
**Cause**: Client not updated to include auth header
**Solution**: Add `authorization: Bearer <token>` to all requests

### Issue: "Invalid token format"
**Cause**: Wrong header format
**Solution**: Ensure format is exactly `Bearer <token>`

### Issue: "Token has expired"
**Cause**: Using old token (24h expiry)
**Solution**: Implement token refresh or re-login

### Issue: "User already exists"
**Cause**: Trying to register existing user
**Solution**: Use login instead, or choose different username

### Issue: Can't see old data
**Cause**: Data not yet associated with user
**Solution**: Use admin account to access all data during transition

## Rollback Plan

If you need to temporarily rollback to no authentication:

⚠️ **Not recommended for production**

1. **Checkout previous version**:
   ```bash
   git checkout <previous_commit_hash>
   cargo build -p timesman-server
   ```

2. **Or modify code** to skip authentication (temporary):
   ```rust
   // In grpc.rs, comment out validation:
   // let _claims = self.validate_token(&request)?;
   ```

## Timeline

- **Phase 1** (Current): Basic JWT authentication
- **Phase 2** (Next): Data ownership and user isolation
- **Phase 3** (Future): Advanced features (token refresh, audit logging)

## Support

For migration help:
1. Check error messages in server logs
2. Verify client code follows examples above
3. Test with grpcurl commands
4. Create GitHub issue if problems persist

## Checklist

- [ ] Server updated and running
- [ ] JWT secret configured
- [ ] User accounts created
- [ ] Client code updated
- [ ] Authentication tested
- [ ] Data access verified
- [ ] Error handling implemented
- [ ] Token refresh planned

---

**Need Help?** Check [AUTHENTICATION.md](./AUTHENTICATION.md) for detailed documentation or [AUTH_QUICK_REFERENCE.md](./AUTH_QUICK_REFERENCE.md) for quick commands.