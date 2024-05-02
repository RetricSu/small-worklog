use crate::types::Task;
use rusqlite::{Connection, Result};

const DATABASE_FILE: &str = "tasks.db";

pub struct Store {
    connection: Connection,
}

impl Store {
    pub fn new(db_path: &str) -> Result<Self> {
        let connection = Connection::open(db_path)?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL,
                created_at INTEGER NOT NULL,
                created_at_date TEXT NOT NULL,
                completed_at INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(Store { connection })
    }

    pub fn default() -> Result<Self> {
        Self::new(DATABASE_FILE) // Creates an in-memory database
    }

    pub fn add_task(&self, task: &Task) -> Result<()> {
        self.connection.execute(
            "INSERT INTO tasks (id, description, completed, created_at, created_at_date, completed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&task.id, &task.description, &task.completed, &(task.created_at as i64), &task.created_at_date, &(task.completed_at as i64)),
        )?;
        Ok(())
    }

    pub fn update_task(&self, updated_task: &Task) -> Result<()> {
        self.connection.execute(
            "UPDATE tasks SET description = ?1, completed = ?2, created_at = ?3, created_at_date = ?4, completed_at = ?5 WHERE id = ?6",
            (&updated_task.description, &updated_task.completed, &(updated_task.created_at as i64), &updated_task.created_at_date, &(updated_task.completed_at as i64), &updated_task.id),
        )?;
        Ok(())
    }

    pub fn _get_task(&self, id: &str) -> Result<Option<Task>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM tasks WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get(2)?,
                created_at: row.get(3)?,
                created_at_date: row.get(4)?,
                completed_at: row.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.connection.prepare("SELECT id, description, completed, created_at, created_at_date, completed_at FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get(2)?,
                created_at: row.get(3)?,
                created_at_date: row.get(4)?,
                completed_at: row.get(5)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task_result in task_iter {
            tasks.push(task_result?);
        }
        Ok(tasks)
    }

    pub fn delete_task_by_id(&self, id: &str) -> Result<()> {
        self.connection
            .execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }

    // Add more methods as needed
}
