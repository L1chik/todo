use super::database::Database;
use crate::model::errors;
use sqlb;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
    pub status: TodoStatus,
}

#[derive(sqlb::Fields, Default, Debug, Clone)]
pub struct TodoPatch {
    pub cid: Option<i64>,
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, Eq, PartialEq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}
sqlb::bindable!(TodoStatus);

pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Database, data: TodoPatch) -> Result<Todo, errors::Error> {
        // let sql = "INSERT INTO todo (cid, title) VALUES ($1, $2) returning id, cid, title, status";
        // let query = sqlx::query_as::<_, Todo>(&sql)
        //     .bind(123 as i64)// TODO: remake from user ctx
        //     .bind(data.title.unwrap_or_else(|| "untitled".to_string()));

        let sb = sqlb::insert()
            .table("todo")
            .data(data.fields())
            .returning(&["id", "cid", "title", "status"]);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Database) -> Result<Vec<Todo>, errors::Error> {
        let sql = "SELECT id, cid, title, status FROM todo ORDER BY id DESC";

        //build  sqlx query
        let query = sqlx::query_as::<_, Todo>(&sql);
        //execute query
        let todos = query.fetch_all(db).await?;

        Ok(todos)
    }
}

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;