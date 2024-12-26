use rusqlite::{Connection, Result, params};

fn main() -> Result<()> {
    let mut connection = Connection::open("my_database.db")?;

    // Begin a transaction
    let transaction = connection.transaction()?;

    // Perform database operations within the transaction
    transaction.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params!["Alice", 25],
    ).expect("Insert failed");

    transaction.execute(
        "UPDATE users SET age = ?1 WHERE name = ?2",
        params![50, "Alice"],
    )?;
    
    // Commit the transaction
    transaction.commit()?;

    // If there's an error, the transaction will be rolled back automatically

    Ok(())
}
