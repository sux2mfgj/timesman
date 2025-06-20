# TimesMan Documentation

This directory contains comprehensive documentation for the TimesMan authentication system.

## Documents Overview

### 📚 [AUTHENTICATION.md](./AUTHENTICATION.md)
**Complete Authentication Guide**
- Detailed explanation of the JWT-based authentication system
- API reference with all endpoints and message types
- Security considerations and best practices
- Testing and troubleshooting guides
- Future roadmap and enhancements

### 🚀 [AUTH_QUICK_REFERENCE.md](./AUTH_QUICK_REFERENCE.md)
**Quick Reference Guide**
- Essential commands and examples
- Common error codes and solutions
- Security checklist
- Environment variables
- Perfect for developers who need quick answers

### 🔄 [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md)
**Migration from v1.0 to v2.0**
- Step-by-step migration instructions
- Breaking changes documentation
- Before/after code examples
- Data migration strategies
- Rollback procedures

### 💻 [API_EXAMPLES.md](./API_EXAMPLES.md)
**Complete Code Examples**
- Working examples in Rust, Python, JavaScript, Go, Java
- Full client implementations
- Error handling patterns
- Token refresh strategies
- CLI/cURL examples

### 📦 [PACKAGING.md](./PACKAGING.md)
**Cross-Platform Distribution**
- Build scripts for Linux, Windows, macOS
- Native package generation (AppImage, MSI, DMG)
- GitHub Actions CI/CD automation
- Container deployment guide
- Distribution and installation instructions

## Quick Start

### 1. Choose Your Path

**New Users**: Start with [AUTHENTICATION.md](./AUTHENTICATION.md)  
**Existing Users**: Check [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md)  
**Quick Help**: Use [AUTH_QUICK_REFERENCE.md](./AUTH_QUICK_REFERENCE.md)  
**Code Examples**: Browse [API_EXAMPLES.md](./API_EXAMPLES.md)

### 2. Basic Commands

```bash
# Start server
cargo run -p timesman-server -- --config timesman-server/config.toml

# Register user
grpcurl -plaintext -d '{"username":"user","email":"user@example.com","password":"pass123"}' \
  localhost:50051 timesman.TimesMan/Register

# Login and get token
grpcurl -plaintext -d '{"username":"user","password":"pass123"}' \
  localhost:50051 timesman.TimesMan/Login

# Use authenticated endpoint
grpcurl -plaintext -H "authorization: Bearer YOUR_TOKEN" \
  localhost:50051 timesman.TimesMan/GetTimes
```

## Authentication Flow Summary

```
1. Register/Login → Get JWT Token
2. Add "authorization: Bearer <token>" to all requests
3. Server validates token and processes request
4. Token expires after 24 hours - need to re-login
```

## Security Features

✅ **JWT Tokens** with HS256 signing  
✅ **Argon2 Password Hashing** with salts  
✅ **Role-Based Access Control** (Admin, User, ReadOnly)  
✅ **Token Expiration** (24 hours default)  
✅ **Bearer Token Authentication**  
✅ **Structured Error Handling**  

## User Roles

| Role | Description |
|------|-------------|
| **Admin** | Full access to all data across users |
| **User** | CRUD access to own data only |
| **ReadOnly** | Read-only access to own data |

## Common Issues & Solutions

| Issue | Solution |
|-------|----------|
| "Missing authorization header" | Add `authorization: Bearer <token>` metadata |
| "Invalid token format" | Ensure "Bearer " prefix with space |
| "Token has expired" | Login again to get fresh token |
| "User already exists" | Use different username or login instead |

## Example Client Code

### Rust
```rust
let mut request = tonic::Request::new(());
request.metadata_mut().insert(
    "authorization",
    MetadataValue::try_from(format!("Bearer {}", token))?
);
let response = client.get_times(request).await?;
```

### Python
```python
metadata = [('authorization', f'Bearer {token}')]
response = stub.GetTimes(Empty(), metadata=metadata)
```

### JavaScript
```javascript
const metadata = new grpc.Metadata();
metadata.add('authorization', `Bearer ${token}`);
const response = await client.getTimes({}, metadata);
```

## Development Workflow

1. **Read Documentation**: Start with appropriate guide above
2. **Set Up Environment**: Configure server and JWT secret
3. **Create User Account**: Register or use existing credentials
4. **Implement Client**: Follow examples for your language
5. **Test Integration**: Verify authentication and data access
6. **Handle Errors**: Implement proper error handling
7. **Plan Token Refresh**: For long-running applications

## Testing Tools

- **grpcurl**: Manual gRPC testing
- **Postman**: If using HTTP endpoints (future)
- **Unit Tests**: `cargo test -p timesman-server auth`
- **Integration Tests**: `cargo test -p timesman-tools`

## Support

For help with authentication:

1. **Check error messages** in server logs
2. **Review appropriate documentation** above
3. **Test with grpcurl** to isolate issues
4. **Verify token format** using jwt.io
5. **Create GitHub issue** if problems persist

## Contributing

To improve this documentation:

1. Fork the repository
2. Update relevant markdown files
3. Test examples work correctly
4. Submit pull request with changes

---

**📅 Last Updated**: December 2024  
**🔐 Auth Version**: v2.0 (JWT-based)  
**📖 API Version**: v1.0