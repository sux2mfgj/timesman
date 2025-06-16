use clap::Parser;

// Import the CLI structures from the main binary
// Since we can't directly import from main.rs, we'll recreate the structures for testing

use clap::{Parser as ClapParser, Subcommand};

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required(true))]
    conn_type: String,
    #[arg(short, long)]
    server: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    GetTimesList,
    CreateTimes {
        #[arg(short, long)]
        title: String,
    },
    DeleteTimes {
        #[arg(short, long)]
        tid: u64,
    },
    UpdateTimes {
        #[arg(short, long)]
        tid: u64,
        #[arg(short = 'T', long)]
        title: String,
    },
    GetPostList {
        #[arg(short, long)]
        tid: u64,
    },
    CreatePost {
        #[arg(short, long)]
        tid: u64,
        #[arg(short = 'T', long)]
        text: String,
    },
    DeletePost {
        #[arg(short, long)]
        tid: u64,
        #[arg(short, long)]
        pid: u64,
    },
    UpdatePost {
        #[arg(short, long)]
        tid: u64,
        #[arg(short, long)]
        pid: u64,
        #[arg(short = 'T', long)]
        text: String,
    },
}

#[cfg(test)]
mod argument_parsing_tests {
    use super::*;

    #[test]
    fn test_parse_get_times_list() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "get-times-list"
        ]).unwrap();
        
        assert_eq!(args.conn_type, "grpc");
        assert!(args.server.is_none());
        assert!(matches!(args.command, Command::GetTimesList));
    }

    #[test]
    fn test_parse_get_times_list_with_server() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "--server", "http://localhost:9090",
            "get-times-list"
        ]).unwrap();
        
        assert_eq!(args.conn_type, "grpc");
        assert_eq!(args.server, Some("http://localhost:9090".to_string()));
        assert!(matches!(args.command, Command::GetTimesList));
    }

    #[test]
    fn test_parse_create_times() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "create-times",
            "--title", "My Project"
        ]).unwrap();
        
        assert_eq!(args.conn_type, "grpc");
        match args.command {
            Command::CreateTimes { title } => assert_eq!(title, "My Project"),
            _ => panic!("Expected CreateTimes command"),
        }
    }

    #[test]
    fn test_parse_delete_times() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "delete-times",
            "--tid", "42"
        ]).unwrap();
        
        match args.command {
            Command::DeleteTimes { tid } => assert_eq!(tid, 42),
            _ => panic!("Expected DeleteTimes command"),
        }
    }

    #[test]
    fn test_parse_update_times() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "update-times",
            "--tid", "1",
            "--title", "Updated Project"
        ]).unwrap();
        
        match args.command {
            Command::UpdateTimes { tid, title } => {
                assert_eq!(tid, 1);
                assert_eq!(title, "Updated Project");
            },
            _ => panic!("Expected UpdateTimes command"),
        }
    }

    #[test]
    fn test_parse_get_post_list() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "get-post-list",
            "--tid", "5"
        ]).unwrap();
        
        match args.command {
            Command::GetPostList { tid } => assert_eq!(tid, 5),
            _ => panic!("Expected GetPostList command"),
        }
    }

    #[test]
    fn test_parse_create_post() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "create-post",
            "--tid", "1",
            "--text", "New post content"
        ]).unwrap();
        
        match args.command {
            Command::CreatePost { tid, text } => {
                assert_eq!(tid, 1);
                assert_eq!(text, "New post content");
            },
            _ => panic!("Expected CreatePost command"),
        }
    }

    #[test]
    fn test_parse_delete_post() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "delete-post",
            "--tid", "1",
            "--pid", "2"
        ]).unwrap();
        
        match args.command {
            Command::DeletePost { tid, pid } => {
                assert_eq!(tid, 1);
                assert_eq!(pid, 2);
            },
            _ => panic!("Expected DeletePost command"),
        }
    }

    #[test]
    fn test_parse_update_post() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "update-post",
            "--tid", "1",
            "--pid", "2",
            "--text", "Updated content"
        ]).unwrap();
        
        match args.command {
            Command::UpdatePost { tid, pid, text } => {
                assert_eq!(tid, 1);
                assert_eq!(pid, 2);
                assert_eq!(text, "Updated content");
            },
            _ => panic!("Expected UpdatePost command"),
        }
    }

    #[test]
    fn test_parse_short_flags() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "-c", "grpc",
            "-s", "http://localhost:8080",
            "create-times",
            "-t", "Short Flag Title"
        ]).unwrap();
        
        assert_eq!(args.conn_type, "grpc");
        assert_eq!(args.server, Some("http://localhost:8080".to_string()));
        
        match args.command {
            Command::CreateTimes { title } => assert_eq!(title, "Short Flag Title"),
            _ => panic!("Expected CreateTimes command"),
        }
    }

    #[test]
    fn test_parse_mixed_flags() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "create-post",
            "-t", "1",
            "--text", "Mixed flags content"
        ]).unwrap();
        
        match args.command {
            Command::CreatePost { tid, text } => {
                assert_eq!(tid, 1);
                assert_eq!(text, "Mixed flags content");
            },
            _ => panic!("Expected CreatePost command"),
        }
    }

    #[test]
    fn test_parse_unicode_content() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "create-times",
            "--title", "プロジェクト名 🚀"
        ]).unwrap();
        
        match args.command {
            Command::CreateTimes { title } => assert_eq!(title, "プロジェクト名 🚀"),
            _ => panic!("Expected CreateTimes command"),
        }
    }

    #[test]
    fn test_parse_spaces_in_strings() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "create-post",
            "--tid", "1",
            "--text", "This is a long post with many spaces and punctuation!"
        ]).unwrap();
        
        match args.command {
            Command::CreatePost { tid, text } => {
                assert_eq!(tid, 1);
                assert_eq!(text, "This is a long post with many spaces and punctuation!");
            },
            _ => panic!("Expected CreatePost command"),
        }
    }

    #[test]
    fn test_parse_special_characters() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "update-times",
            "--tid", "1",
            "--title", "Project: \"Version 2.0\" [FINAL]"
        ]).unwrap();
        
        match args.command {
            Command::UpdateTimes { tid, title } => {
                assert_eq!(tid, 1);
                assert_eq!(title, "Project: \"Version 2.0\" [FINAL]");
            },
            _ => panic!("Expected UpdateTimes command"),
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_missing_conn_type() {
        Args::try_parse_from(&[
            "timesman-tools",
            "get-times-list"
        ]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_missing_title() {
        Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "create-times"
        ]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_missing_tid() {
        Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "delete-times"
        ]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_tid() {
        Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "delete-times",
            "--tid", "not-a-number"
        ]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_negative_tid() {
        Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "delete-times",
            "--tid", "-1"
        ]).unwrap();
    }

    #[test]
    fn test_parse_large_tid() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "delete-times",
            "--tid", "18446744073709551615" // u64::MAX
        ]).unwrap();
        
        match args.command {
            Command::DeleteTimes { tid } => assert_eq!(tid, u64::MAX),
            _ => panic!("Expected DeleteTimes command"),
        }
    }

    #[test]
    fn test_parse_zero_tid() {
        let args = Args::try_parse_from(&[
            "timesman-tools",
            "--conn-type", "grpc",
            "delete-times",
            "--tid", "0"
        ]).unwrap();
        
        match args.command {
            Command::DeleteTimes { tid } => assert_eq!(tid, 0),
            _ => panic!("Expected DeleteTimes command"),
        }
    }
}