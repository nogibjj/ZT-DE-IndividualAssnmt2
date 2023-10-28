#![allow(clippy::non_snake_case)]
// use rusqlite::{Connection, Result};

mod db_operations {
    use rusqlite::{Connection, Result};

    pub fn setup_db() -> Result<Connection, rusqlite::Error> {
        let conn = Connection::open("tasks_db.sqlite3")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, title TEXT NOT NULL, description TEXT)",
            [],
        )?;

        Ok(conn)
    }

    pub fn create(conn: &Connection, title: &str, description: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO tasks (title, description) VALUES (?1, ?2)",
            &[title, description],
        )?;
        Ok(())
    }

    pub fn read(conn: &Connection) -> Result<(), rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT id, title, description FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        for task in task_iter {
            let (id, title, description): (i32, String, String) = task?;
            println!("ID: {}, Title: {}, Description: {}", id, title, description);
        }
        Ok(())
    }

    // This function updates a task based on its ID
    pub fn update(conn: &Connection, id: i32, new_title: &str, new_description: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE tasks SET title = ?1, description = ?2 WHERE id = ?3",
            &[new_title, new_description, &id.to_string()],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM tasks WHERE id = ?1", &[&id.to_string()])?;
        Ok(())
    }
}


fn main() {
    // variable bindings with 'let'
    let greeting = "Hello, user!";
    println!("{}", greeting);

    // control flows: if/else
    let number = 5;
    if number > 10 {
        println!("The number is greater than 10");
    } else {
        println!("The number is not greater than 10");
    }

    // loop
    let mut count = 0;
    loop {
        if count >= 3 {
            break;
        }
        println!("Loop iteration {}", count);
        count += 1;
    }

    // match
    let day = "Monday";
    match day {
        "Monday" => println!("Start of the work week"),
        "Friday" => println!("End of the work week"),
        "Saturday" | "Sunday" => println!("Weekend!"),
        _ => println!("A regular day"),
    }

    // function call
    let result = multiply(5, 4);
    println!("5 multiplied by 4 is {}", result);
    
    // Database initialization with error handling
    let conn = match db_operations::setup_db() {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error setting up the database: {}", e);
            return;
        }
    };

    loop {
        println!("Choose an operation: 1. Create, 2. Read, 3. Update, 4. Delete, 5. Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        match choice.trim().parse::<u8>() {
            Ok(1) => {
                // Logic for Create
                println!("Enter the title of the task:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();

                println!("Enter the description of the task:");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).unwrap();

                if let Err(e) = db_operations::create(&conn, &title.trim(), &description.trim()) {
                    println!("Error creating task: {}", e);
                }
            },
            Ok(2) => {
                // Logic for Read
                if let Err(e) = db_operations::read(&conn) {
                    println!("Error reading tasks: {}", e);
                }
            },
            Ok(3) => {
                // Logic for Update
                println!("Enter the ID of the task to update:");
                let mut id_input = String::new();
                std::io::stdin().read_line(&mut id_input).unwrap();
                let id: i32 = id_input.trim().parse().unwrap();

                println!("Enter the new title:");
                let mut new_title = String::new();
                std::io::stdin().read_line(&mut new_title).unwrap();

                println!("Enter the new description:");
                let mut new_description = String::new();
                std::io::stdin().read_line(&mut new_description).unwrap();

                if let Err(e) = db_operations::update(&conn, id, new_title.trim(), &new_description.trim()) {
                    println!("Error updating task: {}", e);
                }
            },
            Ok(4) => {
                // Logic for Delete
                println!("Enter the ID of the task to delete:");
                let mut id_input = String::new();
                std::io::stdin().read_line(&mut id_input).unwrap();
                let id: i32 = id_input.trim().parse().unwrap();

                if let Err(e) = db_operations::delete(&conn, id) {
                    println!("Error deleting task: {}", e);
                }
            },
            Ok(5) => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Invalid choice! Please try again.");
            },
        }
    }
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
