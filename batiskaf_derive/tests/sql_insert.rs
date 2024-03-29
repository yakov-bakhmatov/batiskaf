use batiskaf::SqlInsert;
use batiskaf_derive::*;

#[test]
fn test_simple() {
    #[allow(unused)]
    #[derive(SqlInsert)]
    struct Person {
        id: i64,
        name: String,
        age: Option<u32>,
    }
    let sql = Person::insert_statement("person");
    assert_eq!(
        "INSERT INTO person (id, name, age) VALUES (:id, :name, :age)",
        sql
    );
}

#[test]
fn test_skip_column() {
    #[allow(unused)]
    #[derive(SqlInsert)]
    struct Person {
        id: i64,
        name: String,
        #[batiskaf(skip)]
        age: Option<u32>,
    }
    let sql = Person::insert_statement("person");
    assert_eq!("INSERT INTO person (id, name) VALUES (:id, :name)", sql);
}

#[test]
fn test_skip_autogenerated() {
    #[allow(unused)]
    #[derive(SqlInsert)]
    struct Person {
        #[batiskaf(autogenerated)]
        id: i64,
        name: String,
        age: Option<u32>,
    }
    let sql = Person::insert_statement("person");
    assert_eq!("INSERT INTO person (name, age) VALUES (:name, :age)", sql);
}

#[test]
fn test_rename_column() {
    #[allow(unused)]
    #[derive(SqlInsert)]
    struct Request {
        id: i64,
        #[batiskaf(column = "type")]
        request_type: String,
        data: String,
    }
    let sql = Request::insert_statement("request");
    assert_eq!(
        "INSERT INTO request (id, type, data) VALUES (:id, :type, :data)",
        sql
    );
}
