use super::*;
use crate::mock_client::MockClient;

#[cfg(test)]
mod client_tests {
    use super::*;

    #[test]
    fn test_mock_client_get_times_empty() {
        let mut client = MockClient::new();
        let result = client.get_times().unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_mock_client_get_times_with_data() {
        let mut client = MockClient::new().with_sample_data();
        let result = client.get_times().unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].title, "Test Project");
        assert_eq!(result[1].title, "Another Project");
    }

    #[test]
    fn test_mock_client_create_times() {
        let mut client = MockClient::new();
        let title = "New Project".to_string();
        let result = client.create_times(title.clone()).unwrap();
        
        assert_eq!(result.id, 1);
        assert_eq!(result.title, title);
        assert!(result.updated_at.is_none());
        
        let all_times = client.get_times().unwrap();
        assert_eq!(all_times.len(), 1);
    }

    #[test]
    fn test_mock_client_delete_times() {
        let mut client = MockClient::new().with_sample_data();
        
        // Delete existing times
        let result = client.delete_times(1);
        assert!(result.is_ok());
        
        let remaining_times = client.get_times().unwrap();
        assert_eq!(remaining_times.len(), 1);
        assert_eq!(remaining_times[0].id, 2);
        
        // Try to delete non-existent times
        let result = client.delete_times(999);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_mock_client_update_times() {
        let mut client = MockClient::new().with_sample_data();
        
        let updated_times = Times {
            id: 1,
            title: "Updated Project".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        };
        
        let result = client.update_times(updated_times).unwrap();
        assert_eq!(result.title, "Updated Project");
        assert!(result.updated_at.is_some());
        
        // Try to update non-existent times
        let non_existent_times = Times {
            id: 999,
            title: "Non-existent".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        };
        
        let result = client.update_times(non_existent_times);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_client_get_posts() {
        let mut client = MockClient::new().with_sample_data();
        
        let posts = client.get_posts(1).unwrap();
        assert_eq!(posts.len(), 2);
        assert_eq!(posts[0].post, "First post");
        assert_eq!(posts[1].post, "Second post");
        
        let empty_posts = client.get_posts(2).unwrap();
        assert_eq!(empty_posts.len(), 0);
        
        // Try to get posts for non-existent times
        let result = client.get_posts(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_client_create_post() {
        let mut client = MockClient::new().with_sample_data();
        
        let text = "New post content".to_string();
        let result = client.create_post(1, text.clone()).unwrap();
        
        assert_eq!(result.id, 3); // next_post_id starts at 3 in sample data
        assert_eq!(result.post, text);
        assert!(result.updated_at.is_none());
        
        let posts = client.get_posts(1).unwrap();
        assert_eq!(posts.len(), 3);
        
        // Try to create post for non-existent times
        let result = client.create_post(999, "test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_client_delete_post() {
        let mut client = MockClient::new().with_sample_data();
        
        // Delete existing post
        let result = client.delete_post(1, 1);
        assert!(result.is_ok());
        
        let posts = client.get_posts(1).unwrap();
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].id, 2);
        
        // Try to delete non-existent post
        let result = client.delete_post(1, 999);
        assert!(result.is_err());
        
        // Try to delete from non-existent times
        let result = client.delete_post(999, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_client_update_post() {
        let mut client = MockClient::new().with_sample_data();
        
        let updated_post = Post {
            id: 1,
            post: "Updated post content".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
            file: None,
            tag: None,
        };
        
        let result = client.update_post(1, updated_post).unwrap();
        assert_eq!(result.post, "Updated post content");
        assert!(result.updated_at.is_some());
        
        // Try to update non-existent post
        let non_existent_post = Post {
            id: 999,
            post: "Non-existent".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
            file: None,
            tag: None,
        };
        
        let result = client.update_post(1, non_existent_post);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_client_error_handling() {
        let mut client = MockClient::new().with_error("Test error");
        
        assert!(client.get_times().is_err());
        assert!(client.create_times("test".to_string()).is_err());
        assert!(client.delete_times(1).is_err());
        assert!(client.get_posts(1).is_err());
        assert!(client.create_post(1, "test".to_string()).is_err());
        assert!(client.delete_post(1, 1).is_err());
        
        let times = Times {
            id: 1,
            title: "test".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        };
        assert!(client.update_times(times.clone()).is_err());
        
        let post = Post {
            id: 1,
            post: "test".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
            file: None,
            tag: None,
        };
        assert!(client.update_post(1, post).is_err());
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn test_run_command_get_times_list() {
        let client = MockClient::new().with_sample_data();
        let cmd = Command::GetTimesList;
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_create_times() {
        let client = MockClient::new();
        let cmd = Command::CreateTimes {
            title: "Test Project".to_string(),
        };
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_delete_times() {
        let client = MockClient::new().with_sample_data();
        let cmd = Command::DeleteTimes { tid: 1 };
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_update_times() {
        let client = MockClient::new().with_sample_data();
        let cmd = Command::UpdateTimes {
            tid: 1,
            title: "Updated Title".to_string(),
        };
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_get_post_list() {
        let client = MockClient::new().with_sample_data();
        let cmd = Command::GetPostList { tid: 1 };
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_create_post() {
        let client = MockClient::new().with_sample_data();
        let cmd = Command::CreatePost {
            tid: 1,
            text: "New post".to_string(),
        };
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_delete_post() {
        let client = MockClient::new().with_sample_data();
        let cmd = Command::DeletePost { tid: 1, pid: 1 };
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_update_post() {
        let client = MockClient::new().with_sample_data();
        let cmd = Command::UpdatePost {
            tid: 1,
            pid: 1,
            text: "Updated post".to_string(),
        };
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_error_handling() {
        let client = MockClient::new().with_error("Connection failed");
        let cmd = Command::GetTimesList;
        
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Connection failed"));
    }

    #[test]
    fn test_run_command_invalid_ids() {
        let client = MockClient::new().with_sample_data();
        
        // Test with invalid times ID
        let cmd = Command::DeleteTimes { tid: 999 };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
        
        let client = MockClient::new().with_sample_data();
        
        // Test with invalid post ID
        let cmd = Command::DeletePost { tid: 1, pid: 999 };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
    }
}