use rusqlite::types::ToSql;
use rusqlite::Connection;

use batiskaf::SqlResult;
use batiskaf_derive::*;

#[test]
fn test_sql_result() {
    #[derive(Debug, Eq, PartialEq, SqlResult)]
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
    stmt.execute(&[(":name", &"Bob" as &dyn ToSql), (":age", &30)])
        .unwrap();
    let mut select = conn.prepare("select id, name, age from person").unwrap();
    let mut rows = select.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let bob = Person::from_row(&row).unwrap();
    assert_eq!(
        Person {
            id: 1,
            name: "Bob".to_string(),
            age: Some(30)
        },
        bob
    );
}

#[test]
fn test_rename() {
    #[derive(Debug, Eq, PartialEq, SqlResult)]
    struct Person {
        pub id: i64,
        #[batiskaf(column = "full_name")]
        pub name: String,
        pub age: Option<u32>,
    }
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "create table person (id integer primary key, full_name text not null, age integer)",
        [],
    )
    .unwrap();
    let mut stmt = conn
        .prepare("insert into person (full_name, age) values (:name, :age)")
        .unwrap();
    stmt.execute(&[(":name", &"Bob" as &dyn ToSql), (":age", &30)])
        .unwrap();
    let mut select = conn
        .prepare("select id, full_name, age from person")
        .unwrap();
    let mut rows = select.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let bob = Person::from_row(&row).unwrap();
    assert_eq!(
        Person {
            id: 1,
            name: "Bob".to_string(),
            age: Some(30)
        },
        bob
    );
}

#[test]
fn test_skip() {
    #[derive(Debug, Eq, PartialEq, SqlResult)]
    struct Person {
        pub id: i64,
        pub name: String,
        #[batiskaf(skip, default)]
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
    stmt.execute(&[(":name", &"Bob" as &dyn ToSql), (":age", &30)])
        .unwrap();
    let mut select = conn.prepare("select id, name, age from person").unwrap();
    let mut rows = select.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let bob = Person::from_row(&row).unwrap();
    assert_eq!(
        Person {
            id: 1,
            name: "Bob".to_string(),
            age: None
        },
        bob
    );
}

#[test]
fn test_default() {
    #[derive(Debug, Eq, PartialEq, SqlResult)]
    struct Person {
        pub id: i64,
        pub name: String,
        #[batiskaf(default)]
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
    stmt.execute(&[(":name", &"Bob" as &dyn ToSql), (":age", &30)])
        .unwrap();
    let mut select = conn.prepare("select id, name from person").unwrap();
    let mut rows = select.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let bob = Person::from_row(&row).unwrap();
    assert_eq!(
        Person {
            id: 1,
            name: "Bob".to_string(),
            age: None
        },
        bob
    );
}

#[test]
fn test_default_struct() {
    #[derive(Debug, Eq, PartialEq, SqlResult)]
    #[batiskaf(default)]
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
    stmt.execute(&[(":name", &"Bob" as &dyn ToSql), (":age", &30)])
        .unwrap();
    let mut select = conn.prepare("select id, name from person").unwrap();
    let mut rows = select.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let bob = Person::from_row(&row).unwrap();
    assert_eq!(
        Person {
            id: 1,
            name: "Bob".to_string(),
            age: None
        },
        bob
    );
}

#[test]
fn test_generic() {
    #[derive(SqlResult)]
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
    stmt.execute(&[(":key", &"name" as &dyn ToSql), (":value", &"Bob")])
        .unwrap();
    let mut select = conn.prepare("select key, value from key_value").unwrap();
    let mut rows = select.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let bob = KeyValue::<String>::from_row(&row).unwrap();
    assert_eq!("name".to_string(), bob.key);
    assert_eq!("Bob".to_string(), bob.value);
}

#[test]
fn test_generic_default() {
    #[derive(SqlResult)]
    #[batiskaf(default)]
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
    stmt.execute(&[(":key", &"name" as &dyn ToSql), (":value", &"Bob")])
        .unwrap();
    let mut select = conn.prepare("select key, value from key_value").unwrap();
    let mut rows = select.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let bob = KeyValue::<String>::from_row(&row).unwrap();
    assert_eq!("name".to_string(), bob.key);
    assert_eq!("Bob".to_string(), bob.value);
}

#[test]
fn test_custom_from_sql() {
    #[allow(unused)]
    #[derive(Debug, Eq, PartialEq)]
    enum Status {
        New,
        Completed,
    }
    impl ::rusqlite::types::FromSql for Status {
        fn column_result(
            value: ::rusqlite::types::ValueRef,
        ) -> ::rusqlite::types::FromSqlResult<Self> {
            match value {
                ::rusqlite::types::ValueRef::Integer(x) => match x {
                    1 => Ok(Status::New),
                    2 => Ok(Status::Completed),
                    x => Err(::rusqlite::types::FromSqlError::OutOfRange(x)),
                },
                _ => Err(::rusqlite::types::FromSqlError::InvalidType),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq, SqlResult)]
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
    stmt.execute(&[(":status", &1)]).unwrap();
    let mut select = conn.prepare("select id, status from \"order\"").unwrap();
    let mut rows = select.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let order = Order::from_row(&row).unwrap();
    assert_eq!(
        Order {
            id: 1,
            status: Status::New
        },
        order
    );
}
