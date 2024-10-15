use salvo::prelude::*;
use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    student_id: String,
}

#[tokio::main]
async fn main() {
    // 初始化数据库
    init_db().unwrap();

    let router = Router::new()
        .push(Router::with_path("add_user").get(add_user))
        .push(Router::with_path("delete_user").get(delete_user))
        .push(Router::with_path("update_user").get(update_user))
        .push(Router::with_path("query_user").get(query_user));

    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    Server::new(acceptor).serve(router).await;
}

fn init_db() -> SqliteResult<()> {
    let conn = Connection::open("users.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            student_id TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    Ok(())
}

#[handler]
async fn add_user(req: &mut Request, res: &mut Response) {
    let name = req.query::<String>("name").unwrap_or_default();
    let student_id = req.query::<String>("student_id").unwrap_or_default();

    let conn = Connection::open("users.db").unwrap();
    let result = conn.execute(
        "INSERT INTO users (name, student_id) VALUES (?1, ?2)",
        [&name, &student_id],
    );

    match result {
        Ok(_) => res.render(Text::Plain("用户添加成功")),
        Err(e) => res.render(Text::Plain(format!("添加用户失败: {}", e))),
    }
}

#[handler]
async fn delete_user(req: &mut Request, res: &mut Response) {
    let id = req.query::<i32>("id").unwrap_or_default();

    let conn = Connection::open("users.db").unwrap();
    let result = conn.execute("DELETE FROM users WHERE id = ?1", [id]);

    match result {
        Ok(rows_affected) if rows_affected > 0 => res.render(Text::Plain("用户删除成功")),
        Ok(_) => res.render(Text::Plain("未找到指定用户")),
        Err(e) => res.render(Text::Plain(format!("删除用户失败: {}", e))),
    }
}

#[handler]
async fn update_user(req: &mut Request, res: &mut Response) {
    let id = req.query::<i32>("id").unwrap_or_default();
    let name = req.query::<String>("name").unwrap_or_default();
    let student_id = req.query::<String>("student_id").unwrap_or_default();

    let conn = Connection::open("users.db").unwrap();
    let result = conn.execute(
        "UPDATE users SET name = ?1, student_id = ?2 WHERE id = ?3",
        [&name, &student_id, &id.to_string()],
    );

    match result {
        Ok(rows_affected) if rows_affected > 0 => res.render(Text::Plain("用户更新成功")),
        Ok(_) => res.render(Text::Plain("未找到指定用户")),
        Err(e) => res.render(Text::Plain(format!("更新用户失败: {}", e))),
    }
}

#[handler]
async fn query_user(req: &mut Request, res: &mut Response) {
    let id = req.query::<i32>("id");

    let conn = Connection::open("users.db").unwrap();

    match id {
        Some(user_id) => {
            let mut stmt = conn.prepare("SELECT id, name, student_id FROM users WHERE id = ?1").unwrap();
            let user_result = stmt.query_row([user_id], |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    student_id: row.get(2)?,
                })
            });

            match user_result {
                Ok(user) => res.render(Json(user)),
                Err(_) => res.render(Text::Plain("未找到指定用户")),
            }
        }
        None => {
            let mut stmt = conn.prepare("SELECT id, name, student_id FROM users").unwrap();
            let users: Result<Vec<User>, _> = stmt.query_map([], |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    student_id: row.get(2)?,
                })
            }).and_then(|mapped_rows| mapped_rows.collect());

            match users {
                Ok(users) => res.render(Json(users)),
                Err(e) => res.render(Text::Plain(format!("查询用户失败: {}", e))),
            }
        }
    }
}