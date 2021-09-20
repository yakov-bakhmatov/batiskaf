use rusqlite::Connection;

use batiskaf::SqlParam;
use batiskaf_derive::*;

#[test]
fn test_sql_param() {
    #[derive(SqlParam)]
    struct Person {
        pub id: i64,
        pub name: String,
        pub age: Option<u32>,
    }
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "create table person (id integer primary key, name text not null, age integer)",
        [],
    )
    .unwrap();
    let mut stmt = conn
        .prepare("insert into person (name, age) values (:name, :age)")
        .unwrap();
    let person = Person {
        id: 0,
        name: "Bob".to_string(),
        age: Some(30),
    };
    let params = person.to_named_params(&stmt);
    stmt.execute(&*params).unwrap();
    let mut select = conn.prepare("select id, name, age from person").unwrap();
    let x = select
        .query_row([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap();
    assert_eq!((1, "Bob".to_string(), 30), x);
}

#[test]
fn test_custom_to_sql() {
    #[allow(unused)]
    enum Status {
        New,
        Completed,
    }
    impl ::rusqlite::types::ToSql for Status {
        fn to_sql(&self) -> ::rusqlite::Result<::rusqlite::types::ToSqlOutput> {
            match self {
                Status::New => Ok(::rusqlite::types::ToSqlOutput::Owned(
                    ::rusqlite::types::Value::Integer(1),
                )),
                Status::Completed => Ok(::rusqlite::types::ToSqlOutput::Owned(
                    ::rusqlite::types::Value::Integer(2),
                )),
            }
        }
    }
    #[derive(SqlParam)]
    struct Order {
        pub id: i64,
        pub status: Status,
    }
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "create table \"order\" (id integer primary key, status integer)",
        [],
    )
    .unwrap();
    let mut stmt = conn
        .prepare("insert into \"order\" (status) values (:status)")
        .unwrap();
    let order = Order {
        id: 0,
        status: Status::New,
    };
    let params = order.to_named_params(&stmt);
    stmt.execute(&*params).unwrap();
    let mut select = conn.prepare("select id, status from \"order\"").unwrap();
    let x = select
        .query_row([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap();
    assert_eq!((1, 1), x);
}

#[test]
fn test_skip_param() {
    #[derive(SqlParam)]
    struct Person {
        pub id: i64,
        pub name: String,
        #[batiskaf(skip)]
        pub age: Option<u32>,
    }
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "create table person (id integer primary key, name text not null, age integer)",
        [],
    )
    .unwrap();
    let mut stmt = conn
        .prepare("insert into person (name, age) values (:name, :age)")
        .unwrap();
    let person = Person {
        id: 0,
        name: "Bob".to_string(),
        age: Some(30),
    };
    let params = person.to_named_params(&stmt);
    stmt.execute(&*params).unwrap();
    let mut select = conn.prepare("select id, name, age from person").unwrap();
    let x: (i64, String, Option<u32>) = select
        .query_row([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap();
    assert_eq!((1, "Bob".to_string(), None), x);
}

#[test]
fn test_generic() {
    #[derive(SqlParam)]
    struct KeyValue<T> {
        pub key: String,
        pub value: T,
    }
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "create table key_value (key text not null, value text not null)",
        [],
    )
    .unwrap();
    let mut stmt = conn
        .prepare("insert into key_value (key, value) values (:key, :value)")
        .unwrap();
    let kv = KeyValue::<String> {
        key: "name".to_string(),
        value: "Bob".to_string(),
    };
    let params = kv.to_named_params(&stmt);
    stmt.execute(&*params).unwrap();
    let mut select = conn.prepare("select key, value from key_value").unwrap();
    let x: (String, String) = select
        .query_row([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap();
    assert_eq!(("name".to_string(), "Bob".to_string()), x);
}
