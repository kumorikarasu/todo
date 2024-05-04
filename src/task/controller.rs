use crate::{error::TodoError, traits::System};

use super::{task, TaskCreateArgs, TaskDeleteArgs};

// todo# create seperate controller for Service type

pub fn create(args: TaskCreateArgs) -> Result<(), TodoError> {
    let task = task::create(args)?;
    println!("{}", serde_yaml::to_string(&task).unwrap());

    Ok(())
}

pub async fn list(backend: Box<dyn System>) -> Result<(), TodoError> {
    println!("Listing tasks");
    backend.get_tasks().await;

    Ok(())
}

pub fn delete(args: TaskDeleteArgs) -> Result<(), TodoError> {
    println!("args: {:?}", args);
    
    task::delete_by_id(args.id.unwrap())?;
    Ok(())
}
