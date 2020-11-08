use postgres::{Client, NoTls,Error,Row};

pub fn test() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:admin@localhost/user_data", NoTls)?;
    client.batch_execute("
    CREATE TABLE accounts (
        id SERIAL PRIMARY KEY,
        username TEXT NOT NULL,
        password TEXT NOT NULL
        token TEXT,
    )
")?;
Ok(())
}

pub fn create_user_acount(username: String,password: String) -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:Aadmin@localhost/user_data", NoTls)?;
    client.execute(
                "INSERT INTO accounts (username, password) VALUES ($1, $2)",
                &[&username, &password],
    )?;

    Ok(())
}

pub fn give_user_token(username: String,token: &String) -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:admin@localhost/user_data", NoTls)?;
    client.execute(
                "UPDATE accounts SET token = ($1) WHERE username = ($2)",
                &[&token, &username],
    )?;
    Ok(())
}

pub fn find_token(token: String) -> Result<Vec<Row>, Error> {
    let mut client = Client::connect("postgresql://postgres:admin@localhost/user_data", NoTls)?;
    let row = client.query("SELECT token FROM accounts WHERE token = ($1)", &[&token])?;
    println!("{:?}",row);
    Ok(row)
}



pub fn find_user_account(username: String) -> Result<Vec<Row>, Error> {
    println!("running");
    let mut client = Client::connect("postgresql://postgres:admin@localhost/user_data", NoTls)?;
    let row = client.query("SELECT password FROM accounts WHERE username = ($1)", &[&username])?;
    Ok(row)
}
