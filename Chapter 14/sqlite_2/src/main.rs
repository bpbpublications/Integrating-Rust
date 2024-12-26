use rusqlite::{Connection, Result};

#[derive(Debug)]
#[allow(dead_code)] 
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    let connection = Connection::open("my_database.db").expect("Failed to open database"); //

    let mut stmt = connection.prepare("SELECT id, name, age FROM users").expect("Query failed");
    
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for user in user_iter {
        println!("Found user {:?}", user.unwrap());
    }

    Ok(())
}