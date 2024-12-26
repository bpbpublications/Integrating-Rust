use rusqlite::Connection;

fn main() {
    let connection = Connection::open("my_database.db").expect("Failed to open database");

    connection.execute(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)",
        [],
    ).expect("Table creation failed");

    connection.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        &[&"Alice", &"30"],
    ).expect("Insertion failed");
}
