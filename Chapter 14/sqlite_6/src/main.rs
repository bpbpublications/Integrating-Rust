use rusqlite::Connection;

fn main() {
    let connection = Connection::open("my_database.db").expect("Failed to open database");

    connection.execute(
        "DELETE FROM users WHERE age > ?1",
        &[&34],
    ).expect("Deletion failed");
}
