use rusqlite::Connection;

fn main() {
    let connection = Connection::open("my_database.db").expect("Failed to open database");

    connection.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        &[&"Abhishek", &"34"]
    ).expect("Insertion failed");
}
