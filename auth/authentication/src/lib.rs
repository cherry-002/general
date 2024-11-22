pub fn read_line() -> String {
    let mut input = String::new();
    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                break;
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                continue;
            }
        }
    }

    input.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Agreed(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User,
    SuperAdmin,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users() -> [User; 3] {
    [
        User::new("Akshan", "4431", LoginRole::SuperAdmin),
        User::new("admin", "4431", LoginRole::Admin),
        User::new("user", "4431", LoginRole::User),
    ]
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();
    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Agreed(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }

    None

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        // assert_eq!(login("Admin", "4431"), LoginAction::Agreed(LoginRole::Admin));
        // assert_eq!(login("User", "4431"), LoginAction::Agreed(LoginRole::User));
        // assert_eq!(login("Akshan", "4431"), LoginAction::Agreed(LoginRole::SuperAdmin));
        // assert_eq!(login("hello", "4431"), LoginAction::Denied);
    }
}
