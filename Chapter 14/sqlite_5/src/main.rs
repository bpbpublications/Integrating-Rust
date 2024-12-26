use rusqlite::{Connection, params};

fn main() {
    let connection = Connection::open("my_database.db").expect("Failed to open database");

    connection.execute(
        "UPDATE users SET age = ?1 WHERE name = ?2",
        params![35, "Alice"]
    ).expect("Update failed");
}
