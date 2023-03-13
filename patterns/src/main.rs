use account::NewAccount;

mod account;
fn main() {
    let acc = NewAccount {
        login: "login1",
        password: "password",
    };
    let acc = acc.check_valid().unwrap();
    let acc = acc.send_confirmation_email();
    let acc = acc.confirmed_email();

    println!(
        "New user, login: {} - password: {}",
        acc.login, acc.password
    );
}
