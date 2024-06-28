use rusqlite::{config, params, Connection, DatabaseName};
use crate::Password;

pub struct Database {
    conn: Connection
}

impl Database {
    
    // Create database
    pub fn new(key: String) -> Result<Database, rusqlite::Error> {
        // Get path to config
        let config_path = dirs::config_dir().unwrap().join("password_manager");
        // Create connection through this
        let conn = Connection::open(config_path)?;
        // Set password for the database
        conn.pragma_update(Some(DatabaseName::Main), "KEY", key)?;

        // Create database from connection
        let db = Database {conn};
        db.create_table();

        Ok(db)
    }

    pub fn create_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS PASSWORDS(
                    id INTEGER PRIMARY KEY,
                    title TEXT NOT NULL,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL
                )
            "
        )?;
        Ok(())
    }

    pub fn load(&self) -> Vec<Password> {
        let statement = self.conn.prepare("SELECT * FROM PASSWORDS").unwrap();
        let items : Vec<Password> = statement.query_map([], |row| {
            let password = Password::new_with_id(
                row.get("id").unwrap(),
                row.get("title").unwrap(),
                row.get("username").unwrap(),
                row.get("password").unwrap()
            );
            Ok(password)
        }).unwrap().map(|i| i.unwrap()).collect();

        items
    }

    pub fn insert(&self, password: Password) {
        self.conn.execute("INSERT INTO PASSWORDS (title, username, password) VALUES (?1, ?2, ?3)", 
        !params![password.title, password.username, password.password]).unwrap();
    }

    pub fn delete(&self, id: usize) {
        self.conn.execute("DELETE FROM PASSWORDS WHERE id=?1", params![id]).unwrap();
    }

    pub fn update(&self, id: usize, password: Password) {
        self.conn.execute("UPDATE PASSWORDS SET title=?1, username=?2, password=?3 WHERE id=?4", params![password.title, password.username, password.password, id])
        .unwrap();
    }
}