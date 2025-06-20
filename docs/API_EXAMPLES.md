# TimesMan Authentication API Examples

This document provides complete, working examples for using the TimesMan authentication API in various programming languages.

## Table of Contents

- [Rust Examples](#rust-examples)
- [Python Examples](#python-examples)
- [JavaScript/Node.js Examples](#javascriptnodejs-examples)
- [Go Examples](#go-examples)
- [Java Examples](#java-examples)
- [cURL/CLI Examples](#curlcli-examples)

## Rust Examples

### Complete Rust Client

```rust
// Cargo.toml
[dependencies]
tonic = "0.12"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
timesman-grpc = { path = "../timesman-grpc" }

// src/main.rs
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use timesman_grpc::grpc::times_man_client::TimesManClient;
use timesman_grpc::grpc::{
    LoginRequest, RegisterRequest, TimesTitle, CreatePostParams
};

pub struct AuthenticatedClient {
    client: TimesManClient<Channel>,
    token: String,
}

impl AuthenticatedClient {
    pub async fn register(
        server_url: &str,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(server_url.to_string())?
            .connect()
            .await?;
        let mut client = TimesManClient::new(channel);

        let register_request = tonic::Request::new(RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        });

        let auth_response = client.register(register_request).await?;
        let token = auth_response.into_inner().access_token;

        Ok(Self { client, token })
    }

    pub async fn login(
        server_url: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(server_url.to_string())?
            .connect()
            .await?;
        let mut client = TimesManClient::new(channel);

        let login_request = tonic::Request::new(LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        });

        let auth_response = client.login(login_request).await?;
        let token = auth_response.into_inner().access_token;

        Ok(Self { client, token })
    }

    fn add_auth_header<T>(&self, mut request: tonic::Request<T>) -> tonic::Request<T> {
        request.metadata_mut().insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", self.token))
                .expect("Invalid token format"),
        );
        request
    }

    pub async fn get_times(&mut self) -> Result<Vec<timesman_grpc::grpc::Times>, Box<dyn std::error::Error>> {
        let request = self.add_auth_header(tonic::Request::new(()));
        let response = self.client.get_times(request).await?;
        Ok(response.into_inner().timeses)
    }

    pub async fn create_times(&mut self, title: &str) -> Result<timesman_grpc::grpc::Times, Box<dyn std::error::Error>> {
        let request = self.add_auth_header(tonic::Request::new(TimesTitle {
            title: title.to_string(),
        }));
        let response = self.client.create_times(request).await?;
        Ok(response.into_inner())
    }

    pub async fn create_post(&mut self, times_id: u64, text: &str) -> Result<timesman_grpc::grpc::Post, Box<dyn std::error::Error>> {
        let request = self.add_auth_header(tonic::Request::new(CreatePostParams {
            id: times_id,
            text: text.to_string(),
        }));
        let response = self.client.create_post(request).await?;
        Ok(response.into_inner())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example usage
    let server_url = "http://localhost:50051";

    // Option 1: Register new user
    let mut client = AuthenticatedClient::register(
        server_url,
        "rust_user",
        "rust@example.com",
        "secure_password_123",
    ).await?;

    // Option 2: Login existing user
    // let mut client = AuthenticatedClient::login(
    //     server_url,
    //     "rust_user", 
    //     "secure_password_123"
    // ).await?;

    // Use authenticated client
    println!("Getting times...");
    let times = client.get_times().await?;
    println!("Found {} times entries", times.len());

    if times.is_empty() {
        println!("Creating a new times entry...");
        let new_times = client.create_times("My Rust Project").await?;
        println!("Created times entry with ID: {}", new_times.id);

        println!("Creating a post...");
        let post = client.create_post(new_times.id, "Started working on Rust client").await?;
        println!("Created post with ID: {}", post.id);
    }

    Ok(())
}
```

## Python Examples

### Complete Python Client

```python
# requirements.txt
grpcio==1.60.0
grpcio-tools==1.60.0

# client.py
import grpc
import timesman_pb2
import timesman_pb2_grpc
from typing import Optional

class AuthenticatedClient:
    def __init__(self, server_url: str, token: str):
        self.server_url = server_url
        self.token = token
        self.channel = grpc.insecure_channel(server_url)
        self.stub = timesman_pb2_grpc.TimesManStub(self.channel)

    @classmethod
    def register(cls, server_url: str, username: str, email: str, password: str):
        """Register a new user and return authenticated client"""
        channel = grpc.insecure_channel(server_url)
        stub = timesman_pb2_grpc.TimesManStub(channel)
        
        register_request = timesman_pb2.RegisterRequest(
            username=username,
            email=email,
            password=password
        )
        
        try:
            auth_response = stub.Register(register_request)
            return cls(server_url, auth_response.access_token)
        except grpc.RpcError as e:
            raise Exception(f"Registration failed: {e.details()}")

    @classmethod
    def login(cls, server_url: str, username: str, password: str):
        """Login existing user and return authenticated client"""
        channel = grpc.insecure_channel(server_url)
        stub = timesman_pb2_grpc.TimesManStub(channel)
        
        login_request = timesman_pb2.LoginRequest(
            username=username,
            password=password
        )
        
        try:
            auth_response = stub.Login(login_request)
            return cls(server_url, auth_response.access_token)
        except grpc.RpcError as e:
            raise Exception(f"Login failed: {e.details()}")

    def _get_metadata(self):
        """Get authentication metadata for requests"""
        return [('authorization', f'Bearer {self.token}')]

    def get_times(self):
        """Get all times entries for authenticated user"""
        try:
            response = self.stub.GetTimes(
                timesman_pb2.google_dot_protobuf_dot_empty__pb2.Empty(),
                metadata=self._get_metadata()
            )
            return response.timeses
        except grpc.RpcError as e:
            raise Exception(f"Failed to get times: {e.details()}")

    def create_times(self, title: str):
        """Create a new times entry"""
        request = timesman_pb2.TimesTitle(title=title)
        try:
            response = self.stub.CreateTimes(
                request,
                metadata=self._get_metadata()
            )
            return response
        except grpc.RpcError as e:
            raise Exception(f"Failed to create times: {e.details()}")

    def get_posts(self, times_id: int):
        """Get posts for a times entry"""
        request = timesman_pb2.TimesId(id=times_id)
        try:
            response = self.stub.GetPosts(
                request,
                metadata=self._get_metadata()
            )
            return response.posts
        except grpc.RpcError as e:
            raise Exception(f"Failed to get posts: {e.details()}")

    def create_post(self, times_id: int, text: str):
        """Create a new post"""
        request = timesman_pb2.CreatePostParams(id=times_id, text=text)
        try:
            response = self.stub.CreatePost(
                request,
                metadata=self._get_metadata()
            )
            return response
        except grpc.RpcError as e:
            raise Exception(f"Failed to create post: {e.details()}")

    def close(self):
        """Close the gRPC channel"""
        self.channel.close()

# Example usage
def main():
    server_url = "localhost:50051"
    
    try:
        # Option 1: Register new user
        client = AuthenticatedClient.register(
            server_url,
            "python_user",
            "python@example.com", 
            "secure_password_456"
        )
        
        # Option 2: Login existing user
        # client = AuthenticatedClient.login(
        #     server_url,
        #     "python_user",
        #     "secure_password_456"
        # )
        
        print("Getting times...")
        times_list = client.get_times()
        print(f"Found {len(times_list)} times entries")
        
        if not times_list:
            print("Creating a new times entry...")
            new_times = client.create_times("My Python Project")
            print(f"Created times entry with ID: {new_times.id}")
            
            print("Creating a post...")
            post = client.create_post(new_times.id, "Started working on Python client")
            print(f"Created post with ID: {post.id}")
        else:
            # Work with existing times
            times_entry = times_list[0]
            print(f"Getting posts for times ID: {times_entry.id}")
            posts = client.get_posts(times_entry.id)
            print(f"Found {len(posts)} posts")
        
        client.close()
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
```

## JavaScript/Node.js Examples

### Complete Node.js Client

```javascript
// package.json
{
  "name": "timesman-client",
  "version": "1.0.0",
  "dependencies": {
    "@grpc/grpc-js": "^1.9.0",
    "@grpc/proto-loader": "^0.7.0"
  }
}

// client.js
const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');
const path = require('path');

const PROTO_PATH = path.join(__dirname, 'timesman.proto');

class AuthenticatedClient {
    constructor(serverUrl, token) {
        this.serverUrl = serverUrl;
        this.token = token;
        
        const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
            keepCase: true,
            longs: String,
            enums: String,
            defaults: true,
            oneofs: true
        });
        
        const timesman = grpc.loadPackageDefinition(packageDefinition).timesman;
        this.client = new timesman.TimesMan(serverUrl, grpc.credentials.createInsecure());
    }

    static async register(serverUrl, username, email, password) {
        const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
            keepCase: true,
            longs: String,
            enums: String,
            defaults: true,
            oneofs: true
        });
        
        const timesman = grpc.loadPackageDefinition(packageDefinition).timesman;
        const client = new timesman.TimesMan(serverUrl, grpc.credentials.createInsecure());

        return new Promise((resolve, reject) => {
            client.register({
                username: username,
                email: email,
                password: password
            }, (error, response) => {
                if (error) {
                    reject(new Error(`Registration failed: ${error.message}`));
                } else {
                    resolve(new AuthenticatedClient(serverUrl, response.access_token));
                }
            });
        });
    }

    static async login(serverUrl, username, password) {
        const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
            keepCase: true,
            longs: String,
            enums: String,
            defaults: true,
            oneofs: true
        });
        
        const timesman = grpc.loadPackageDefinition(packageDefinition).timesman;
        const client = new timesman.TimesMan(serverUrl, grpc.credentials.createInsecure());

        return new Promise((resolve, reject) => {
            client.login({
                username: username,
                password: password
            }, (error, response) => {
                if (error) {
                    reject(new Error(`Login failed: ${error.message}`));
                } else {
                    resolve(new AuthenticatedClient(serverUrl, response.access_token));
                }
            });
        });
    }

    _getMetadata() {
        const metadata = new grpc.Metadata();
        metadata.add('authorization', `Bearer ${this.token}`);
        return metadata;
    }

    async getTimes() {
        return new Promise((resolve, reject) => {
            this.client.getTimes({}, this._getMetadata(), (error, response) => {
                if (error) {
                    reject(new Error(`Failed to get times: ${error.message}`));
                } else {
                    resolve(response.timeses);
                }
            });
        });
    }

    async createTimes(title) {
        return new Promise((resolve, reject) => {
            this.client.createTimes({ title: title }, this._getMetadata(), (error, response) => {
                if (error) {
                    reject(new Error(`Failed to create times: ${error.message}`));
                } else {
                    resolve(response);
                }
            });
        });
    }

    async getPosts(timesId) {
        return new Promise((resolve, reject) => {
            this.client.getPosts({ id: timesId }, this._getMetadata(), (error, response) => {
                if (error) {
                    reject(new Error(`Failed to get posts: ${error.message}`));
                } else {
                    resolve(response.posts);
                }
            });
        });
    }

    async createPost(timesId, text) {
        return new Promise((resolve, reject) => {
            this.client.createPost({ 
                id: timesId, 
                text: text 
            }, this._getMetadata(), (error, response) => {
                if (error) {
                    reject(new Error(`Failed to create post: ${error.message}`));
                } else {
                    resolve(response);
                }
            });
        });
    }
}

// Example usage
async function main() {
    const serverUrl = 'localhost:50051';
    
    try {
        // Option 1: Register new user
        const client = await AuthenticatedClient.register(
            serverUrl,
            'js_user',
            'js@example.com',
            'secure_password_789'
        );
        
        // Option 2: Login existing user
        // const client = await AuthenticatedClient.login(
        //     serverUrl,
        //     'js_user',
        //     'secure_password_789'
        // );
        
        console.log('Getting times...');
        const timesList = await client.getTimes();
        console.log(`Found ${timesList.length} times entries`);
        
        if (timesList.length === 0) {
            console.log('Creating a new times entry...');
            const newTimes = await client.createTimes('My JavaScript Project');
            console.log(`Created times entry with ID: ${newTimes.id}`);
            
            console.log('Creating a post...');
            const post = await client.createPost(newTimes.id, 'Started working on JavaScript client');
            console.log(`Created post with ID: ${post.id}`);
        } else {
            // Work with existing times
            const timesEntry = timesList[0];
            console.log(`Getting posts for times ID: ${timesEntry.id}`);
            const posts = await client.getPosts(timesEntry.id);
            console.log(`Found ${posts.length} posts`);
        }
        
    } catch (error) {
        console.error('Error:', error.message);
    }
}

main().catch(console.error);
```

## Go Examples

### Complete Go Client

```go
// go.mod
module timesman-client

go 1.21

require (
    google.golang.org/grpc v1.59.0
    google.golang.org/protobuf v1.31.0
)

// client.go
package main

import (
    "context"
    "fmt"
    "log"
    
    "google.golang.org/grpc"
    "google.golang.org/grpc/credentials/insecure"
    "google.golang.org/grpc/metadata"
    
    pb "timesman/grpc" // Generated protobuf package
)

type AuthenticatedClient struct {
    conn   *grpc.ClientConn
    client pb.TimesManClient
    token  string
}

func Register(serverURL, username, email, password string) (*AuthenticatedClient, error) {
    conn, err := grpc.Dial(serverURL, grpc.WithTransportCredentials(insecure.NewCredentials()))
    if err != nil {
        return nil, fmt.Errorf("failed to connect: %v", err)
    }
    
    client := pb.NewTimesManClient(conn)
    
    resp, err := client.Register(context.Background(), &pb.RegisterRequest{
        Username: username,
        Email:    email,
        Password: password,
    })
    if err != nil {
        conn.Close()
        return nil, fmt.Errorf("registration failed: %v", err)
    }
    
    return &AuthenticatedClient{
        conn:   conn,
        client: client,
        token:  resp.AccessToken,
    }, nil
}

func Login(serverURL, username, password string) (*AuthenticatedClient, error) {
    conn, err := grpc.Dial(serverURL, grpc.WithTransportCredentials(insecure.NewCredentials()))
    if err != nil {
        return nil, fmt.Errorf("failed to connect: %v", err)
    }
    
    client := pb.NewTimesManClient(conn)
    
    resp, err := client.Login(context.Background(), &pb.LoginRequest{
        Username: username,
        Password: password,
    })
    if err != nil {
        conn.Close()
        return nil, fmt.Errorf("login failed: %v", err)
    }
    
    return &AuthenticatedClient{
        conn:   conn,
        client: client,
        token:  resp.AccessToken,
    }, nil
}

func (c *AuthenticatedClient) getContext() context.Context {
    md := metadata.New(map[string]string{
        "authorization": fmt.Sprintf("Bearer %s", c.token),
    })
    return metadata.NewOutgoingContext(context.Background(), md)
}

func (c *AuthenticatedClient) GetTimes() ([]*pb.Times, error) {
    resp, err := c.client.GetTimes(c.getContext(), &pb.Empty{})
    if err != nil {
        return nil, fmt.Errorf("failed to get times: %v", err)
    }
    return resp.Timeses, nil
}

func (c *AuthenticatedClient) CreateTimes(title string) (*pb.Times, error) {
    resp, err := c.client.CreateTimes(c.getContext(), &pb.TimesTitle{
        Title: title,
    })
    if err != nil {
        return nil, fmt.Errorf("failed to create times: %v", err)
    }
    return resp, nil
}

func (c *AuthenticatedClient) GetPosts(timesID uint64) ([]*pb.Post, error) {
    resp, err := c.client.GetPosts(c.getContext(), &pb.TimesId{
        Id: timesID,
    })
    if err != nil {
        return nil, fmt.Errorf("failed to get posts: %v", err)
    }
    return resp.Posts, nil
}

func (c *AuthenticatedClient) CreatePost(timesID uint64, text string) (*pb.Post, error) {
    resp, err := c.client.CreatePost(c.getContext(), &pb.CreatePostParams{
        Id:   timesID,
        Text: text,
    })
    if err != nil {
        return nil, fmt.Errorf("failed to create post: %v", err)
    }
    return resp, nil
}

func (c *AuthenticatedClient) Close() error {
    return c.conn.Close()
}

func main() {
    serverURL := "localhost:50051"
    
    // Option 1: Register new user
    client, err := Register(serverURL, "go_user", "go@example.com", "secure_password_000")
    if err != nil {
        log.Fatalf("Failed to register: %v", err)
    }
    defer client.Close()
    
    // Option 2: Login existing user
    // client, err := Login(serverURL, "go_user", "secure_password_000")
    // if err != nil {
    //     log.Fatalf("Failed to login: %v", err)
    // }
    // defer client.Close()
    
    fmt.Println("Getting times...")
    timesList, err := client.GetTimes()
    if err != nil {
        log.Fatalf("Failed to get times: %v", err)
    }
    fmt.Printf("Found %d times entries\n", len(timesList))
    
    if len(timesList) == 0 {
        fmt.Println("Creating a new times entry...")
        newTimes, err := client.CreateTimes("My Go Project")
        if err != nil {
            log.Fatalf("Failed to create times: %v", err)
        }
        fmt.Printf("Created times entry with ID: %d\n", newTimes.Id)
        
        fmt.Println("Creating a post...")
        post, err := client.CreatePost(newTimes.Id, "Started working on Go client")
        if err != nil {
            log.Fatalf("Failed to create post: %v", err)
        }
        fmt.Printf("Created post with ID: %d\n", post.Id)
    } else {
        // Work with existing times
        timesEntry := timesList[0]
        fmt.Printf("Getting posts for times ID: %d\n", timesEntry.Id)
        posts, err := client.GetPosts(timesEntry.Id)
        if err != nil {
            log.Fatalf("Failed to get posts: %v", err)
        }
        fmt.Printf("Found %d posts\n", len(posts))
    }
}
```

## cURL/CLI Examples

### Using grpcurl

```bash
#!/bin/bash

SERVER="localhost:50051"

# Register new user
echo "Registering user..."
REGISTER_RESPONSE=$(grpcurl -plaintext -d '{
  "username": "cli_user",
  "email": "cli@example.com", 
  "password": "secure_password_cli"
}' $SERVER timesman.TimesMan/Register)

echo "Register response: $REGISTER_RESPONSE"

# Extract token (requires jq)
TOKEN=$(echo $REGISTER_RESPONSE | jq -r '.access_token')
echo "Token: ${TOKEN:0:20}..."

# Login existing user (alternative)
# echo "Logging in..."
# LOGIN_RESPONSE=$(grpcurl -plaintext -d '{
#   "username": "cli_user",
#   "password": "secure_password_cli"
# }' $SERVER timesman.TimesMan/Login)
# TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.access_token')

# Get times with authentication
echo "Getting times..."
TIMES_RESPONSE=$(grpcurl -plaintext \
  -H "authorization: Bearer $TOKEN" \
  $SERVER timesman.TimesMan/GetTimes)

echo "Times response: $TIMES_RESPONSE"

# Create new times entry
echo "Creating times entry..."
CREATE_TIMES_RESPONSE=$(grpcurl -plaintext \
  -H "authorization: Bearer $TOKEN" \
  -d '{"title": "CLI Project"}' \
  $SERVER timesman.TimesMan/CreateTimes)

echo "Create times response: $CREATE_TIMES_RESPONSE"

# Extract times ID
TIMES_ID=$(echo $CREATE_TIMES_RESPONSE | jq -r '.id')
echo "Created times with ID: $TIMES_ID"

# Create post
echo "Creating post..."
CREATE_POST_RESPONSE=$(grpcurl -plaintext \
  -H "authorization: Bearer $TOKEN" \
  -d "{\"id\": $TIMES_ID, \"text\": \"Working via CLI\"}" \
  $SERVER timesman.TimesMan/CreatePost)

echo "Create post response: $CREATE_POST_RESPONSE"

echo "All operations completed successfully!"
```

### Using curl (for HTTP endpoints - future)

```bash
#!/bin/bash

SERVER="http://localhost:8080"

# Register user
echo "Registering user..."
REGISTER_RESPONSE=$(curl -s -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "username": "http_user",
    "email": "http@example.com",
    "password": "secure_password_http"
  }' \
  $SERVER/auth/register)

echo "Register response: $REGISTER_RESPONSE"

# Extract token
TOKEN=$(echo $REGISTER_RESPONSE | jq -r '.access_token')
echo "Token: ${TOKEN:0:20}..."

# Get times with authentication
echo "Getting times..."
TIMES_RESPONSE=$(curl -s -H "Authorization: Bearer $TOKEN" \
  $SERVER/times)

echo "Times response: $TIMES_RESPONSE"

# Create times entry
echo "Creating times entry..."
CREATE_TIMES_RESPONSE=$(curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "HTTP Project"}' \
  $SERVER/times)

echo "Create times response: $CREATE_TIMES_RESPONSE"
```

## Common Patterns

### Error Handling

```rust
// Rust error handling
match client.get_times(request).await {
    Ok(response) => {
        let times = response.into_inner().timeses;
        println!("Success: {} times entries", times.len());
    }
    Err(status) => {
        match status.code() {
            tonic::Code::Unauthenticated => {
                println!("Authentication failed - please login again");
            }
            tonic::Code::PermissionDenied => {
                println!("Access denied - insufficient permissions");
            }
            _ => {
                println!("Error: {}", status.message());
            }
        }
    }
}
```

### Token Refresh Pattern

```rust
pub struct RefreshableClient {
    client: AuthenticatedClient,
    credentials: (String, String), // username, password
}

impl RefreshableClient {
    pub async fn call_with_retry<T, F, Fut>(&mut self, f: F) -> Result<T, Box<dyn std::error::Error>>
    where
        F: Fn(&mut AuthenticatedClient) -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
    {
        match f(&mut self.client).await {
            Ok(result) => Ok(result),
            Err(e) if e.to_string().contains("UNAUTHENTICATED") => {
                // Token expired, refresh and retry
                self.refresh_token().await?;
                f(&mut self.client).await
            }
            Err(e) => Err(e),
        }
    }

    async fn refresh_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client = AuthenticatedClient::login(
            "http://localhost:50051",
            &self.credentials.0,
            &self.credentials.1,
        ).await?;
        Ok(())
    }
}
```

---

These examples provide complete, working implementations for integrating with the TimesMan authentication system across multiple programming languages. Each example includes proper error handling and follows best practices for the respective language.