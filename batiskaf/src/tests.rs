use rusqlite::{Connection, NO_PARAMS};

use super::*;

#[derive(Debug, Eq, PartialEq)]
struct Person {
    pub id: i64,
    pub name: String,
    pub age: Option<u32>,
}

impl SqlParam for Person {
    fn to_named_params(&self, stmt: &Statement) -> Vec<(&str, &dyn ToSql)> {
        let mut params = Vec::new();
        if let Ok(Some(_)) = stmt.parameter_index(":id") {
            params.push((":id", &self.id as &ToSql));
        }
        if let Ok(Some(_)) = stmt.parameter_index(":name") {
            params.push((":name", &self.name as &ToSql));
        }
        if let Ok(Some(_)) = stmt.parameter_index(":age") {
            params.push((":age", &self.age as &ToSql));
        }
        params
    }
}

impl SqlResult for Person {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Person {
            id: row.get("id")?,
            name: row.get("name")?,
            age: row.get("age").unwrap_or_default(),
        })
    }
}

impl SqlInsert for Person {
    fn insert_statement(table: &str) -> String {
        format!("insert into {} (name, age) values (:name, :age)", table)
    }
}

impl SqlUpdate for Person {
    fn update_statement(table: &str) -> String {
        format!(
            "update {} set name = :name, age = :age where id = :id",
            table
        )
    }
}

impl SqlDelete for Person {
    fn delete_statement(table: &str) -> String {
        format!("delete from {} where id = :id", table)
    }
}

fn create_table(conn: &Connection) {
    conn.execute(
        "create table person(id integer primary key, name text not null, age integer)",
        NO_PARAMS,
    )
    .unwrap();
}

#[test]
fn test_sql_param() {
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn);
    let mut stmt = conn
        .prepare("insert into person (name, age) values (:name, :age)")
        .unwrap();
    let person = Person {
        id: 0,
        name: "Bob".to_string(),
        age: Some(30),
    };
    let params = person.to_named_params(&stmt);
    stmt.execute_named(&params).unwrap();
    let mut select = conn.prepare("select id, name, age from person").unwrap();
    let x = select
        .query_row(NO_PARAMS, |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap();
    assert_eq!((1, "Bob".to_string(), 30), x);
}

#[test]
fn test_sql_result() {
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn);
    let mut stmt = conn
        .prepare("insert into person (name, age) values (:name, :age)")
        .unwrap();
    stmt.execute_named(&[(":name", &"Alice" as &ToSql), (":age", &33)])
        .unwrap();
    let mut select = conn.prepare("select id, name, age from person").unwrap();
    let x = select.query_row(NO_PARAMS, Person::from_row).unwrap();
    assert_eq!(
        Person {
            id: 1,
            name: "Alice".to_string(),
            age: Some(33)
        },
        x
    );
}

#[test]
fn test_sql_result_2() {
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn);
    let mut stmt = conn
        .prepare("insert into person (name, age) values (:name, :age)")
        .unwrap();
    stmt.execute_named(&[(":name", &"Alice" as &ToSql), (":age", &33)])
        .unwrap();
    let mut select = conn.prepare("select id, name from person").unwrap();
    let x = select.query_row(NO_PARAMS, Person::from_row).unwrap();
    assert_eq!(
        Person {
            id: 1,
            name: "Alice".to_string(),
            age: None
        },
        x
    );
}

#[test]
fn test_select_one() {
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn);
    let mut stmt = conn
        .prepare("insert into person (name, age) values (:name, :age)")
        .unwrap();
    stmt.execute_named(&[(":name", &"Alice" as &ToSql), (":age", &33)])
        .unwrap();
    let x: Person = conn
        .select_one("select id, name, age from person", &[])
        .unwrap();
    assert_eq!(
        Person {
            id: 1,
            name: "Alice".to_string(),
            age: Some(33)
        },
        x
    );
}

#[test]
fn test_select_many() {
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn);
    let alice = Person {
        id: 1,
        name: "Alice".to_string(),
        age: Some(33),
    };
    let bob = Person {
        id: 2,
        name: "Bob".to_string(),
        age: Some(30),
    };
    let mut stmt = conn
        .prepare("insert into person (name, age) values (:name, :age)")
        .unwrap();
    stmt.execute_named(&alice.to_named_params(&stmt)).unwrap();
    stmt.execute_named(&bob.to_named_params(&stmt)).unwrap();
    let people = conn
        .select_many("select id, name, age from person", &[])
        .unwrap();
    assert_eq!(vec![alice, bob], people);
}

#[test]
fn test_insert() {
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn);
    let mut bob = Person {
        id: 0,
        name: "Bob".to_string(),
        age: Some(30),
    };
    bob.id = conn.insert("person", &bob).unwrap();
    let x: Person = conn
        .select_one("select id, name, age from person", &[])
        .unwrap();
    assert_eq!(bob, x);
}

#[test]
fn test_update() {
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn);
    let mut bob = Person {
        id: 0,
        name: "Bob".to_string(),
        age: Some(30),
    };
    bob.id = conn.insert("person", &bob).unwrap();
    bob.name = "Bob Smith".to_string();
    bob.age = None;
    conn.update("person", &bob).unwrap();
    let x: Person = conn
        .select_one("select id, name, age from person", &[])
        .unwrap();
    assert_eq!(bob, x);
}

#[test]
fn test_delete() {
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn);
    let mut bob = Person {
        id: 0,
        name: "Bob".to_string(),
        age: Some(30),
    };
    bob.id = conn.insert("person", &bob).unwrap();
    let mut select = conn.prepare("select id, name, age from person").unwrap();
    assert!(select.exists(NO_PARAMS).unwrap());
    conn.delete("person", &bob).unwrap();
    assert!(!select.exists(NO_PARAMS).unwrap());
}
