use super::TaskCreateArgs;
use super::entities::*;

use crate::error::TodoError;

pub fn create(args: TaskCreateArgs) -> Result<Task, TodoError> {

    Ok(Task {
        id: None,
        title: args.title,
        description: args.desc,
        status: TaskStatus::Todo,
    })
}

pub fn list() -> Result<Vec<Task>, TodoError> {

    Ok(vec![])
}

pub fn delete_by_id(id: i64) -> Result<(), TodoError> {

    Ok(())
}
