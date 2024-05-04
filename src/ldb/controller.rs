use super::{issue, IssueCreateArgs, IssueDeleteArgs};

// todo# create seperate controller for Service type

pub fn create(args: IssueCreateArgs) -> Result<(), anyhow::Error> {
    let issue = issue::create(args)?;
    println!("{}", serde_yaml::to_string(&issue).unwrap());

    Ok(())
}

pub fn list() -> Result<(), anyhow::Error> {
    let list = issue::list()?;
    println!("{}", serde_yaml::to_string(&list).unwrap());

    Ok(())
}

pub fn delete(args: IssueDeleteArgs) -> Result<(), anyhow::Error> {
    println!("args: {:?}", args);

    if args.all {
        println!("Are you sure you want to delete all issues? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim() != "y" {
            return Ok(());
        }
        let delete = issue::delete_all()?;
        println!("{}", serde_yaml::to_string(&delete).unwrap());
        return Ok(());
    }
    
    issue::delete_by_id(args.id.unwrap())?;
    Ok(())
}
