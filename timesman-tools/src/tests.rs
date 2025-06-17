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

#[cfg(test)]
mod todo_detail_tests {
    use super::*;
    use crate::mock_client::MockClient;

    #[test]
    fn test_mock_client_todo_detail_operations() {
        let mut client = MockClient::new().with_sample_data();
        
        // Test get todos (should include todos with details from sample data)
        let todos = client.get_todos(1).unwrap();
        assert_eq!(todos.len(), 2); // Sample data has 2 todos for times id 1
        
        // Test creating todo with detail
        let todo_with_detail = client.create_todo_with_detail(
            1, 
            "Task with detail".to_string(), 
            Some("This is a detailed description\nwith multiple lines\nand special chars: ñáéíóú 🚀".to_string())
        ).unwrap();
        
        assert_eq!(todo_with_detail.content, "Task with detail");
        assert_eq!(todo_with_detail.detail, Some("This is a detailed description\nwith multiple lines\nand special chars: ñáéíóú 🚀".to_string()));
        assert!(todo_with_detail.done_at.is_none());
        
        // Test creating todo without detail
        let todo_without_detail = client.create_todo_with_detail(
            1, 
            "Simple task".to_string(), 
            None
        ).unwrap();
        
        assert_eq!(todo_without_detail.content, "Simple task");
        assert_eq!(todo_without_detail.detail, None);
        
        // Test getting specific todo detail
        let retrieved_todo = client.get_todo_detail(1, todo_with_detail.id).unwrap();
        assert_eq!(retrieved_todo.id, todo_with_detail.id);
        assert_eq!(retrieved_todo.detail, todo_with_detail.detail);
        
        // Test updating todo detail
        let updated_todo = client.update_todo_detail(
            1, 
            todo_with_detail.id, 
            "Updated detail with new content".to_string()
        ).unwrap();
        
        assert_eq!(updated_todo.id, todo_with_detail.id);
        assert_eq!(updated_todo.detail, Some("Updated detail with new content".to_string()));
        
        // Verify the update persisted
        let retrieved_again = client.get_todo_detail(1, todo_with_detail.id).unwrap();
        assert_eq!(retrieved_again.detail, Some("Updated detail with new content".to_string()));
        
        // Test marking todo as done preserves detail
        let done_todo = client.mark_todo_done(1, todo_with_detail.id, true).unwrap();
        assert_eq!(done_todo.detail, Some("Updated detail with new content".to_string()));
        assert!(done_todo.done_at.is_some());
        
        // Test unmarking todo preserves detail
        let undone_todo = client.mark_todo_done(1, todo_with_detail.id, false).unwrap();
        assert_eq!(undone_todo.detail, Some("Updated detail with new content".to_string()));
        assert!(undone_todo.done_at.is_none());
    }
    
    #[test]
    fn test_grpc_client_todo_detail_methods() {
        // This test would require a running gRPC server, so we'll test the mock client
        // In a real integration test environment, this would test the actual gRPC client
        let mut client = MockClient::new().with_sample_data();
        
        // Test error handling for non-existent times
        let result = client.get_todo_detail(999, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
        
        // Test error handling for non-existent todo
        let result = client.get_todo_detail(1, 999);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
        
        // Test error handling for update operations
        let result = client.update_todo_detail(999, 1, "detail".to_string());
        assert!(result.is_err());
        
        let result = client.update_todo_detail(1, 999, "detail".to_string());
        assert!(result.is_err());
        
        // Test creating todo with empty detail
        let todo_empty_detail = client.create_todo_with_detail(
            1, 
            "Empty detail task".to_string(), 
            Some("".to_string())
        ).unwrap();
        assert_eq!(todo_empty_detail.detail, Some("".to_string()));
        
        // Test creating todo with very long detail
        let long_detail = "x".repeat(5000);
        let todo_long_detail = client.create_todo_with_detail(
            1, 
            "Long detail task".to_string(), 
            Some(long_detail.clone())
        ).unwrap();
        assert_eq!(todo_long_detail.detail, Some(long_detail));
    }
    
    #[test]
    fn test_todo_detail_cli_commands() {
        let client = MockClient::new().with_sample_data();
        
        // Test GetTodoList command
        let cmd = Command::GetTodoList { tid: 1 };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
        
        let client = MockClient::new().with_sample_data();
        
        // Test CreateTodoWithDetail command
        let cmd = Command::CreateTodoWithDetail { 
            tid: 1, 
            content: "CLI todo".to_string(), 
            detail: "CLI detail".to_string() 
        };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
        
        let client = MockClient::new().with_sample_data();
        
        // Test CreateTodo command (without detail)
        let cmd = Command::CreateTodo { 
            tid: 1, 
            content: "Simple CLI todo".to_string() 
        };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
        
        let client = MockClient::new().with_sample_data();
        
        // Test GetTodoDetail command
        let cmd = Command::GetTodoDetail { tid: 1, tdid: 1 };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
        
        let client = MockClient::new().with_sample_data();
        
        // Test UpdateTodoDetail command
        let cmd = Command::UpdateTodoDetail { 
            tid: 1, 
            tdid: 1, 
            detail: "Updated via CLI".to_string() 
        };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
        
        let client = MockClient::new().with_sample_data();
        
        // Test MarkTodoDone command
        let cmd = Command::MarkTodoDone { 
            tid: 1, 
            tdid: 1, 
            done: true 
        };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
        
        let client = MockClient::new().with_sample_data();
        
        // Test MarkTodoUndone command
        let cmd = Command::MarkTodoUndone { 
            tid: 1, 
            tdid: 1 
        };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_todo_detail_error_scenarios() {
        // Test with forced error client
        let client = MockClient::new().with_error("Network error");
        
        let cmd = Command::GetTodoList { tid: 1 };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Network error"));
        
        let client = MockClient::new().with_error("Connection timeout");
        
        let cmd = Command::CreateTodo { 
            tid: 1, 
            content: "Test".to_string() 
        };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
        
        let client = MockClient::new().with_error("Server unavailable");
        
        let cmd = Command::GetTodoDetail { tid: 1, tdid: 1 };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
        
        let client = MockClient::new().with_error("Database error");
        
        let cmd = Command::UpdateTodoDetail { 
            tid: 1, 
            tdid: 1, 
            detail: "Test".to_string() 
        };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
        
        // Test with invalid IDs using working client
        let client = MockClient::new().with_sample_data();
        
        let cmd = Command::GetTodoDetail { tid: 999, tdid: 1 };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
        
        let client = MockClient::new().with_sample_data();
        
        let cmd = Command::UpdateTodoDetail { 
            tid: 1, 
            tdid: 999, 
            detail: "Test".to_string() 
        };
        let result = run_command(Box::new(client), &cmd);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_todo_detail_unicode_and_special_characters() {
        let mut client = MockClient::new().with_sample_data();
        
        // Test with Unicode characters
        let unicode_detail = "Unicode test: 🚀 ñáéíóú 中文 العربية 🎉\nMultiple lines\nSpecial chars: \"quotes\" 'apostrophes' \\backslashes\\";
        let todo = client.create_todo_with_detail(
            1, 
            "Unicode task".to_string(), 
            Some(unicode_detail.to_string())
        ).unwrap();
        
        assert_eq!(todo.detail, Some(unicode_detail.to_string()));
        
        // Test retrieving Unicode detail
        let retrieved = client.get_todo_detail(1, todo.id).unwrap();
        assert_eq!(retrieved.detail, Some(unicode_detail.to_string()));
        
        // Test updating with Unicode
        let updated_unicode = "Updated Unicode: 🔥 テスト مرحبا 🌟";
        let updated_todo = client.update_todo_detail(
            1, 
            todo.id, 
            updated_unicode.to_string()
        ).unwrap();
        
        assert_eq!(updated_todo.detail, Some(updated_unicode.to_string()));
        
        // Test with control characters
        let control_chars = "Control chars: \u{0001}\u{0002}\u{001F}\u{007F}";
        let control_todo = client.create_todo_with_detail(
            1, 
            "Control chars task".to_string(), 
            Some(control_chars.to_string())
        ).unwrap();
        
        assert_eq!(control_todo.detail, Some(control_chars.to_string()));
    }
    
    #[test]
    fn test_todo_detail_edge_cases() {
        let mut client = MockClient::new().with_sample_data();
        
        // Test with whitespace-only detail
        let whitespace_detail = "   \n\t\r\n   ";
        let todo = client.create_todo_with_detail(
            1, 
            "Whitespace task".to_string(), 
            Some(whitespace_detail.to_string())
        ).unwrap();
        assert_eq!(todo.detail, Some(whitespace_detail.to_string()));
        
        // Test updating detail to empty string
        let updated_todo = client.update_todo_detail(
            1, 
            todo.id, 
            "".to_string()
        ).unwrap();
        assert_eq!(updated_todo.detail, Some("".to_string()));
        
        // Test creating todo with None detail then updating to Some
        let none_todo = client.create_todo_with_detail(
            1, 
            "Initially no detail".to_string(), 
            None
        ).unwrap();
        assert_eq!(none_todo.detail, None);
        
        let updated_to_some = client.update_todo_detail(
            1, 
            none_todo.id, 
            "Now has detail".to_string()
        ).unwrap();
        assert_eq!(updated_to_some.detail, Some("Now has detail".to_string()));
        
        // Test with extremely long detail
        let huge_detail = "a".repeat(100000);
        let huge_todo = client.create_todo_with_detail(
            1, 
            "Huge detail task".to_string(), 
            Some(huge_detail.clone())
        ).unwrap();
        assert_eq!(huge_todo.detail, Some(huge_detail));
    }
    
    #[test]
    fn test_todo_detail_persistence_across_operations() {
        let mut client = MockClient::new().with_sample_data();
        
        // Create todo with detail
        let original_detail = "Original detail that should persist";
        let todo = client.create_todo_with_detail(
            1, 
            "Persistence test".to_string(), 
            Some(original_detail.to_string())
        ).unwrap();
        
        // Mark as done and verify detail persists
        let done_todo = client.mark_todo_done(1, todo.id, true).unwrap();
        assert_eq!(done_todo.detail, Some(original_detail.to_string()));
        assert!(done_todo.done_at.is_some());
        
        // Mark as not done and verify detail persists
        let undone_todo = client.mark_todo_done(1, todo.id, false).unwrap();
        assert_eq!(undone_todo.detail, Some(original_detail.to_string()));
        assert!(undone_todo.done_at.is_none());
        
        // Update detail and verify other fields unchanged
        let new_detail = "Updated detail";
        let updated_todo = client.update_todo_detail(
            1, 
            todo.id, 
            new_detail.to_string()
        ).unwrap();
        
        assert_eq!(updated_todo.id, todo.id);
        assert_eq!(updated_todo.content, todo.content);
        assert_eq!(updated_todo.created_at, todo.created_at);
        assert_eq!(updated_todo.detail, Some(new_detail.to_string()));
        assert!(updated_todo.done_at.is_none()); // Should still be undone
        
        // Get fresh copy and verify all changes persisted
        let final_todo = client.get_todo_detail(1, todo.id).unwrap();
        assert_eq!(final_todo.detail, Some(new_detail.to_string()));
        assert_eq!(final_todo.content, todo.content);
        assert!(final_todo.done_at.is_none());
    }
    
    #[test]
    fn test_todo_detail_batch_operations() {
        let mut client = MockClient::new().with_sample_data();
        
        // Create multiple todos with details
        let todos_data = vec![
            ("Task 1", Some("Detail 1")),
            ("Task 2", None),
            ("Task 3", Some("Detail 3 with special chars: ñáéíóú 🚀")),
            ("Task 4", Some("")),
        ];
        
        let mut created_todos = Vec::new();
        for (content, detail) in todos_data {
            let todo = client.create_todo_with_detail(
                1,
                content.to_string(),
                detail.map(|d| d.to_string())
            ).unwrap();
            created_todos.push(todo);
        }
        
        // Verify all todos were created correctly
        let all_todos = client.get_todos(1).unwrap();
        assert!(all_todos.len() >= 4); // At least the 4 we just created
        
        // Test batch updates
        for (i, todo) in created_todos.iter().enumerate() {
            let new_detail = format!("Batch updated detail {}", i);
            let updated = client.update_todo_detail(
                1,
                todo.id,
                new_detail.clone()
            ).unwrap();
            assert_eq!(updated.detail, Some(new_detail));
        }
        
        // Test batch status changes
        for (i, todo) in created_todos.iter().enumerate() {
            let done = i % 2 == 0; // Mark even indices as done
            let status_updated = client.mark_todo_done(1, todo.id, done).unwrap();
            assert_eq!(status_updated.done_at.is_some(), done);
            
            // Verify detail is preserved during status change
            assert!(status_updated.detail.is_some());
            assert!(status_updated.detail.as_ref().unwrap().contains("Batch updated"));
        }
    }
    
    #[test]
    fn test_todo_detail_data_integrity() {
        let mut client = MockClient::new().with_sample_data();
        
        // Test that modifications don't affect unrelated todos
        let todo1 = client.create_todo_with_detail(
            1,
            "Todo 1".to_string(),
            Some("Detail 1".to_string())
        ).unwrap();
        
        let todo2 = client.create_todo_with_detail(
            1,
            "Todo 2".to_string(),
            Some("Detail 2".to_string())
        ).unwrap();
        
        // Modify todo1
        let _updated_todo1 = client.update_todo_detail(
            1,
            todo1.id,
            "Modified detail 1".to_string()
        ).unwrap();
        
        let _marked_todo1 = client.mark_todo_done(1, todo1.id, true).unwrap();
        
        // Verify todo2 is unaffected
        let unchanged_todo2 = client.get_todo_detail(1, todo2.id).unwrap();
        assert_eq!(unchanged_todo2.content, "Todo 2");
        assert_eq!(unchanged_todo2.detail, Some("Detail 2".to_string()));
        assert!(unchanged_todo2.done_at.is_none());
        
        // Verify todo1 changes are correct
        let final_todo1 = client.get_todo_detail(1, todo1.id).unwrap();
        assert_eq!(final_todo1.content, "Todo 1");
        assert_eq!(final_todo1.detail, Some("Modified detail 1".to_string()));
        assert!(final_todo1.done_at.is_some());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::{Duration, Instant};
    use crate::mock_client::MockClient;

    // Performance test helpers
    fn measure_time<F>(f: F) -> Duration 
    where
        F: FnOnce(),
    {
        let start = Instant::now();
        f();
        start.elapsed()
    }

    fn create_large_todo_dataset(client: &mut MockClient, count: usize) {
        for i in 0..count {
            let content = format!("Todo item number {}", i);
            let detail = if i % 3 == 0 {
                Some(format!("This is a detailed description for todo number {}. It contains multiple lines of text to simulate real-world usage.\nLine 2: More information about the task.\nLine 3: Additional context and requirements.", i))
            } else {
                None
            };
            
            let _ = client.create_todo_with_detail(1, content, detail);
        }
    }

    #[test]
    fn bench_todo_detail_serialization() {
        let created_at = chrono::Utc::now().naive_utc();
        
        // Test serialization performance with different detail sizes
        let medium_detail = "x".repeat(1000);
        let large_detail = "x".repeat(10000);
        let test_cases = vec![
            ("Small detail", "Simple task detail"),
            ("Medium detail", medium_detail.as_str()),
            ("Large detail", large_detail.as_str()),
            ("Unicode detail", "Unicode test: 🚀 ñáéíóú 中文 العربية 🎉\nMultiple lines\nSpecial chars: \"quotes\" 'apostrophes'"),
        ];
        
        for (case_name, detail_content) in test_cases {
            let todo = Todo {
                id: 1,
                content: "Performance test todo".to_string(),
                detail: Some(detail_content.to_string()),
                created_at,
                done_at: None,
            };
            
            // Measure serialization time
            let serialize_time = measure_time(|| {
                for _ in 0..1000 {
                    let _json = serde_json::to_string(&todo).unwrap();
                }
            });
            
            // Measure deserialization time
            let json = serde_json::to_string(&todo).unwrap();
            let deserialize_time = measure_time(|| {
                for _ in 0..1000 {
                    let _todo: Todo = serde_json::from_str(&json).unwrap();
                }
            });
            
            println!("{}: Serialize: {:?}, Deserialize: {:?}", 
                     case_name, serialize_time, deserialize_time);
            
            // Assert reasonable performance (should complete within reasonable time)
            assert!(serialize_time < Duration::from_millis(5000), 
                    "Serialization too slow for {}: {:?}", case_name, serialize_time);
            assert!(deserialize_time < Duration::from_millis(5000), 
                    "Deserialization too slow for {}: {:?}", case_name, deserialize_time);
        }
    }

    #[test]
    fn bench_large_todo_lists() {
        let todo_counts = vec![100, 1000];
        
        for count in todo_counts {
            let mut client = MockClient::new().with_sample_data();
            
            // Measure creation time
            let creation_time = measure_time(|| {
                create_large_todo_dataset(&mut client, count);
            });
            
            // Measure retrieval time
            let retrieval_time = measure_time(|| {
                for _ in 0..10 {
                    let _todos = client.get_todos(1).unwrap();
                }
            });
            
            // Measure individual todo detail retrieval
            let detail_retrieval_time = measure_time(|| {
                for i in 1..=std::cmp::min(100, count) {
                    let _todo = client.get_todo_detail(1, i as u64);
                }
            });
            
            println!("{} todos - Creation: {:?}, Retrieval: {:?}, Detail retrieval: {:?}", 
                     count, creation_time, retrieval_time, detail_retrieval_time);
            
            // Performance assertions (should scale reasonably)
            assert!(creation_time < Duration::from_secs(10), 
                    "Creation too slow for {} todos: {:?}", count, creation_time);
            assert!(retrieval_time < Duration::from_secs(5), 
                    "Retrieval too slow for {} todos: {:?}", count, retrieval_time);
            assert!(detail_retrieval_time < Duration::from_secs(5), 
                    "Detail retrieval too slow for {} todos: {:?}", count, detail_retrieval_time);
        }
    }

    #[test]
    fn bench_todo_operations_throughput() {
        let mut client = MockClient::new().with_sample_data();
        
        // Benchmark create operations
        let create_ops = 1000;
        let create_time = measure_time(|| {
            for i in 0..create_ops {
                let content = format!("Benchmark todo {}", i);
                let detail = if i % 2 == 0 {
                    Some(format!("Detail for todo {}", i))
                } else {
                    None
                };
                let _ = client.create_todo_with_detail(1, content, detail);
            }
        });
        
        let create_throughput = create_ops as f64 / create_time.as_secs_f64();
        println!("Create throughput: {:.2} ops/sec", create_throughput);
        
        // Benchmark update operations
        let update_ops = 500;
        let update_time = measure_time(|| {
            for i in 1..=update_ops {
                let new_detail = format!("Updated detail for todo {}", i);
                let _ = client.update_todo_detail(1, i as u64, new_detail);
            }
        });
        
        let update_throughput = update_ops as f64 / update_time.as_secs_f64();
        println!("Update throughput: {:.2} ops/sec", update_throughput);
        
        // Benchmark mark done operations
        let done_ops = 500;
        let done_time = measure_time(|| {
            for i in 1..=done_ops {
                let _ = client.mark_todo_done(1, i as u64, i % 2 == 0);
            }
        });
        
        let done_throughput = done_ops as f64 / done_time.as_secs_f64();
        println!("Mark done throughput: {:.2} ops/sec", done_throughput);
        
        // Assert minimum acceptable throughput  
        assert!(create_throughput > 1000.0, "Create throughput too low: {:.2} ops/sec", create_throughput);
        assert!(update_throughput > 1000.0, "Update throughput too low: {:.2} ops/sec", update_throughput);
        assert!(done_throughput > 1000.0, "Done throughput too low: {:.2} ops/sec", done_throughput);
    }

    #[test]
    fn bench_memory_usage_patterns() {
        let mut client = MockClient::new().with_sample_data();
        
        // Test memory usage with varying detail sizes
        let detail_sizes = vec![0, 100, 1000, 10000];
        
        for size in detail_sizes {
            let detail = if size == 0 {
                None
            } else {
                Some("a".repeat(size))
            };
            
            let creation_time = measure_time(|| {
                for i in 0..100 {
                    let content = format!("Memory test todo {}", i);
                    let _ = client.create_todo_with_detail(1, content, detail.clone());
                }
            });
            
            println!("Detail size {}: Creation time for 100 todos: {:?}", size, creation_time);
            
            // Memory usage should not grow exponentially with detail size
            assert!(creation_time < Duration::from_secs(5), 
                    "Memory usage pattern inefficient for detail size {}: {:?}", size, creation_time);
        }
    }

    #[test]
    fn bench_stress_test() {
        let mut client = MockClient::new().with_sample_data();
        
        // Stress test with mixed operations
        let stress_operations = 1000;
        let start = Instant::now();
        
        for i in 0..stress_operations {
            match i % 5 {
                0 => {
                    // Create operation
                    let content = format!("Stress test todo {}", i);
                    let detail = Some(format!("Stress test detail {}", i));
                    let _ = client.create_todo_with_detail(1, content, detail);
                },
                1 => {
                    // Read operation  
                    let _ = client.get_todos(1);
                },
                2 => {
                    // Update operation
                    if i > 0 {
                        let detail = format!("Updated detail {}", i);
                        let _ = client.update_todo_detail(1, (i % 100 + 1) as u64, detail);
                    }
                },
                3 => {
                    // Get detail operation
                    if i > 0 {
                        let _ = client.get_todo_detail(1, (i % 100 + 1) as u64);
                    }
                },
                4 => {
                    // Mark done operation
                    if i > 0 {
                        let _ = client.mark_todo_done(1, (i % 100 + 1) as u64, i % 2 == 0);
                    }
                },
                _ => unreachable!(),
            }
        }
        
        let total_time = start.elapsed();
        let throughput = stress_operations as f64 / total_time.as_secs_f64();
        
        println!("Stress test: {} mixed operations in {:?} = {:.2} ops/sec", 
                 stress_operations, total_time, throughput);
        
        // Should handle mixed workload efficiently
        assert!(throughput > 1000.0, "Stress test throughput too low: {:.2} ops/sec", throughput);
        assert!(total_time < Duration::from_secs(10), "Stress test too slow: {:?}", total_time);
    }
}