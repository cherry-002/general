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
    Denied
}


#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
    SuperAdmin,
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();
    if username == "akshan" && password == "4431" {
        LoginAction::Agreed(LoginRole::SuperAdmin)
    } else if username == "admin" && password == "4431" {
        LoginAction::Agreed(LoginRole::Admin)
    } else if username == "user" && password == "4431" {
        LoginAction::Agreed(LoginRole::User)
    } else {
        LoginAction::Denied
    }
}   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        assert_eq!(login("Admin", "4431"), LoginAction::Agreed(LoginRole::Admin));
        assert_eq!(login("User", "4431"), LoginAction::Agreed(LoginRole::User));
        assert_eq!(login("Akshan", "4431"), LoginAction::Agreed(LoginRole::SuperAdmin));
        assert_eq!(login("hello", "4431"), LoginAction::Denied);
    }
}
