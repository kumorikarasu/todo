use std::str::FromStr;

use super::IssueCreateArgs;
use super::entities::*;

use rusqlite::Error;
use rusqlite::params;
use rusqlite::Connection;

pub fn init() {
    let conn = rusqlite::Connection::open(".todo/.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS issues (
            id INTEGER PRIMARY KEY
        )",
        [],
    ).unwrap();

    //We don't care if this fails
    let _ = conn.execute(
        "ALTER TABLE issues ADD COLUMN title TEXT default null",
        [],
    );
    let _ = conn.execute(
        "ALTER TABLE issues ADD COLUMN description TEXT default null",
        [],
    );
    let _ = conn.execute(
        "ALTER TABLE issues ADD COLUMN status TEXT default null",
        [],
    );
}

pub fn create(args: IssueCreateArgs) -> Result<Issue, Error> {
    let mut issue = Issue{
        id: None,
        title: args.title,
        description: args.desc,
        status: IssueStatus::default(),
    };

    let conn = Connection::open(".todo/.db")?;
    conn.execute(
        "INSERT INTO issues (title, description, status) values (?1, ?2, ?3)",
        params![&issue.title, &issue.description, &issue.status.to_string()]
    ).unwrap();

    let id = conn.last_insert_rowid();

    issue.id = Some(id);
    Ok(issue)
}

pub fn list() -> Result<Vec<Issue>, Error> {
    let conn = Connection::open(".todo/.db")?;
    let mut stmt = conn.prepare("SELECT id, title, description, status FROM issues")?;
    let issue_iter = stmt.query_map([], |row| {
        let status = row.get(3).unwrap_or("Todo".to_string());
        Ok(Issue {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            description: row.get(2).ok(),
            status: IssueStatus::from_str(&status).unwrap(),
        })
    })?;

    let mut issues = Vec::new();
    for issue in issue_iter {
        issues.push(issue?);
    }

    Ok(issues)
}

pub fn delete_all() -> Result<(), Error> {
    let conn = Connection::open(".todo/.db")?;
    conn.execute("DELETE FROM issues", [])?;
    Ok(())
}

pub fn delete_by_id(id: i64) -> Result<(), Error> {
    let conn = Connection::open(".todo/.db")?;
    conn.execute("DELETE FROM issues WHERE id = ?1", params![id])?;
    Ok(())
}
