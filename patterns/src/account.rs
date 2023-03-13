// TypeState + NewType pattern
pub struct NewAccount<'a> {
    pub login: &'a str,
    pub password: &'a str,
}

struct ValidPassword<'a>(&'a str);

impl<'a> ValidPassword<'a> {
    fn build(&self) -> Result<&str, String> {
        if self.0.trim().len() < 8 {
            Err("this very weak password".to_owned())
        } else {
            Ok(self.0)
        }
    }
}

impl<'a> NewAccount<'a> {
    pub fn check_valid(self) -> Result<ValidAccount<'a>, String> {
        let valid_acc = ValidAccount {
            login: self.login,
            password: self.password,
        };
        let binding = ValidPassword(self.password);
        let password = binding.build().unwrap();

        match (self.login.len(), password) {
            (6, p) if p.is_ascii() => Ok(valid_acc),
            _ => Err("Login must be 6 chars, password only ascii symbols".to_string()),
        }
    }
}

pub struct ValidAccount<'a> {
    login: &'a str,
    password: &'a str,
}

impl<'a> ValidAccount<'a> {
    pub fn send_confirmation_email(self) -> SendInstructionAccount<'a> {
        SendInstructionAccount {
            login: self.login,
            password: self.password,
        }
    }
}

pub struct SendInstructionAccount<'a> {
    login: &'a str,
    password: &'a str,
}

impl<'a> SendInstructionAccount<'a> {
    pub fn confirmed_email(self) -> FullAccount<'a> {
        FullAccount {
            login: self.login,
            password: self.password,
        }
    }
}

pub struct FullAccount<'a> {
    pub login: &'a str,
    pub password: &'a str,
}
