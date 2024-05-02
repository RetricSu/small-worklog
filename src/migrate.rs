use crate::store::Store;
use crate::types::Task;
use dirs::desktop_dir;
use serde_json::from_reader;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

const FILE_NAME: &str = "small-worklog-tasks.dat";
const DATABASE_FILE: &str = "small-worklog.db";

pub fn try_migrate_v1() {
    let mut db_path = dirs::home_dir().unwrap_or_default();
    db_path.push(DATABASE_FILE);

    let desktop_path = desktop_dir().unwrap_or_default();
    let v1_json_file_path = desktop_path.join(FILE_NAME);

    if !db_path.exists() && v1_json_file_path.exists() {
        migrate_db_from_v1();
    }
}

pub fn migrate_db_from_v1() {
    let tasks = load_tasks().unwrap_or_default();
    let store = Store::default().unwrap();
    for task in tasks.iter() {
        store.add_task(task).unwrap();
    }
    println!("migrate db from v1 succeed!");
}

fn load_tasks() -> Result<Vec<Task>, Error> {
    let desktop_path =
        desktop_dir().ok_or(Error::new(ErrorKind::Other, "Failed to get desktop path"))?;
    let file_path = desktop_path.join(FILE_NAME);

    if file_path.exists() {
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        let tasks: Vec<Task> = from_reader(reader)?;
        Ok(tasks)
    } else {
        Ok(vec![])
    }
}
