use crate::types::Task;
use dirs::desktop_dir;
use serde_json::{from_reader, to_writer};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind};

const FILE_NAME: &str = "small-worklog-tasks.dat";

pub fn store_tasks(tasks: &[Task]) -> Result<(), Error> {
    let desktop_path =
        desktop_dir().ok_or(Error::new(ErrorKind::Other, "Failed to get desktop path"))?;
    let file_path = desktop_path.join(FILE_NAME);

    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    to_writer(&mut writer, tasks)?;

    Ok(())
}

pub fn load_tasks() -> Result<Vec<Task>, Error> {
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
