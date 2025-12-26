use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
    created_at: SystemTime,
}

#[derive(Debug)]
struct Session {
    user_id: u64,
    token: String,
    expires_at: SystemTime,
}

struct UserSessionManager {
    users: Arc<Mutex<HashMap<u64, User>>>,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    session_duration: Duration,
}

impl UserSessionManager {
    fn new(session_duration_hours: u64) -> Self {
        UserSessionManager {
            users: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            session_duration: Duration::from_secs(session_duration_hours * 3600),
        }
    }

    fn register_user(&self, username: String, email: String) -> Result<u64, String> {
        let mut users = self.users.lock().map_err(|e| e.to_string())?;
        let id = (users.len() + 1) as u64;
        let user = User {
            id,
            username,
            email,
            created_at: SystemTime::now(),
        };
        users.insert(id, user);
        Ok(id)
    }

    fn create_session(&self, user_id: u64, token: String) -> Result<(), String> {
        let users = self.users.lock().map_err(|e| e.to_string())?;
        if !users.contains_key(&user_id) {
            return Err("User not found".to_string());
        }
        drop(users);

        let session = Session {
            user_id,
            token: token.clone(),
            expires_at: SystemTime::now() + self.session_duration,
        };
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        sessions.insert(token, session);
        Ok(())
    }

    fn validate_session(&self, token: &str) -> Result<u64, String> {
        let sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session) = sessions.get(token) {
            if session.expires_at > SystemTime::now() {
                return Ok(session.user_id);
            }
        }
        Err("Invalid or expired session".to_string())
    }
}

fn main() {
    let manager = UserSessionManager::new(24);
    
    match manager.register_user("alice".to_string(), "alice@example.com".to_string()) {
        Ok(user_id) => println!("Registered user with ID: {}", user_id),
        Err(e) => println!("Failed to register user: {}", e),
    }

    match manager.create_session(1, "abc123token".to_string()) {
        Ok(_) => println!("Session created successfully"),
        Err(e) => println!("Failed to create session: {}", e),
    }

    match manager.validate_session("abc123token") {
        Ok(user_id) => println!("Session valid for user ID: {}", user_id),
        Err(e) => println!("Session validation failed: {}", e),
    }
}
