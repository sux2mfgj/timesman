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

    #[test] 
    fn test_tui_command_structure() {
        // Test that TUI command exists and can be parsed
        let client = MockClient::new().with_sample_data();
        let cmd = Command::Tui;
        
        // We can't actually run the TUI in tests due to terminal requirements,
        // but we can verify the command structure is correct
        assert!(matches!(cmd, Command::Tui));
    }
}

#[cfg(test)]
mod tui_tests {
    use super::*;
    use crate::tui::app::{App, AppMode};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_input_mode_key_handling_prevents_help_interference() {
        let client = MockClient::new().with_sample_data();
        let mut app = App::new(Box::new(client));
        
        // Test CreateTimes mode
        app.mode = AppMode::CreateTimes;
        app.input.clear();
        
        // Simulate typing 'h' - should add to input, not trigger help
        let key_event = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        let should_quit = app.handle_key_event(key_event).unwrap();
        
        assert!(!should_quit);
        assert_eq!(app.mode, AppMode::CreateTimes); // Should still be in input mode
        assert_eq!(app.input, "h"); // 'h' should be added to input
        
        // Test typing more characters including 'h'
        let chars = ['e', 'l', 'l', 'o', ' ', 'h', 'e', 'l', 'p'];
        for c in chars {
            let key_event = KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE);
            app.handle_key_event(key_event).unwrap();
        }
        
        assert_eq!(app.input, "hello help");
        assert_eq!(app.mode, AppMode::CreateTimes);
    }

    #[test]
    fn test_input_mode_escape_returns_to_parent_mode() {
        let client = MockClient::new().with_sample_data();
        let mut app = App::new(Box::new(client));
        
        // Test CreateTimes mode escape behavior
        app.mode = AppMode::CreateTimes;
        app.input = "some input".to_string();
        
        let escape_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        app.handle_key_event(escape_event).unwrap();
        
        assert_eq!(app.mode, AppMode::TimesList);
        assert_eq!(app.input, ""); // Input should be cleared
        
        // Test CreatePost mode escape behavior
        app.mode = AppMode::CreatePost;
        app.input = "some post content".to_string();
        
        let escape_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        app.handle_key_event(escape_event).unwrap();
        
        assert_eq!(app.mode, AppMode::PostsList);
        assert_eq!(app.input, "");
    }

    #[test]
    fn test_non_input_mode_help_key_works() {
        let client = MockClient::new().with_sample_data();
        let mut app = App::new(Box::new(client));
        
        // Test help key works in TimesList mode
        app.mode = AppMode::TimesList;
        
        let help_event = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        app.handle_key_event(help_event).unwrap();
        
        assert_eq!(app.mode, AppMode::Help);
        
        // Test help key works in PostsList mode
        app.mode = AppMode::PostsList;
        
        let help_event = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        app.handle_key_event(help_event).unwrap();
        
        assert_eq!(app.mode, AppMode::Help);
    }

    #[test]
    fn test_ctrl_q_works_in_all_modes() {
        let client = MockClient::new().with_sample_data();
        let mut app = App::new(Box::new(client));
        
        let modes_to_test = vec![
            AppMode::TimesList,
            AppMode::PostsList,
            AppMode::CreateTimes,
            AppMode::EditTimes,
            AppMode::CreatePost,
            AppMode::EditPost,
            AppMode::Help,
        ];
        
        for mode in modes_to_test {
            app.mode = mode.clone();
            
            let ctrl_q_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
            let should_quit = app.handle_key_event(ctrl_q_event).unwrap();
            
            assert!(should_quit, "Ctrl+Q should work in mode: {:?}", mode);
        }
    }

    #[test]
    fn test_backspace_works_in_input_modes() {
        let client = MockClient::new().with_sample_data();
        let mut app = App::new(Box::new(client));
        
        app.mode = AppMode::CreateTimes;
        app.input = "hello".to_string();
        
        let backspace_event = KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE);
        app.handle_key_event(backspace_event).unwrap();
        
        assert_eq!(app.input, "hell");
        assert_eq!(app.mode, AppMode::CreateTimes);
    }
}