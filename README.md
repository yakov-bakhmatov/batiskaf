# Batiskaf

*Batiskaf* - библиотека, предназначенная для избавления от бойлерплейта при работе с [*rusqlite*](https://crates.io/crates/rusqlite). Идея заключается в том, что программист пишет SQL-код, а преобразования структур в параметры для запросов и результаты запросов в структуры выполняет магия библиотеки. Идея честно позаимствована из замечательного фреймворка [MyBatis](http://www.mybatis.org/mybatis-3/).


## Подключение

Добавить в `Cargo.toml`:
```toml
batiskaf = { git = "https://github.com/yakov-bakhmatov/batiskaf", features = ["derive"] }
```
*Batiskaf* зависит от [*rusqlite*](https://crates.io/crates/rusqlite) версии 0.17

## Пример использования

```rust
use rusqlite::{Connection, NO_PARAMS};

use batiskaf::*;

#[derive(Debug, SqlParam, SqlResult)]
struct Person {
    pub id: i64,
    pub name: String,
    pub age: Option<u32>,
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "create table person (id integer primary key, name text not null, age integer)",
        NO_PARAMS,
    )
    .unwrap();

    let person = Person {
        id: 0,
        name: "Bob".to_string(),
        age: Some(30),
    };

    let mut stmt = conn
        .prepare("insert into person (name, age) values (:name, :age)")
        .unwrap();
    let params = person.to_named_params(&stmt);
    stmt.execute_named(&params).unwrap();

    let mut select = conn.prepare("select id, name, age from person").unwrap();
    let mut rows = select.query(NO_PARAMS).unwrap();
    let row = rows.next().unwrap().unwrap();
    let bob = Person::from_row(&row).unwrap();
    println!("{:?}", bob); // Person { id: 1, name: "Bob", age: Some(30) }
}
```

## Краткое описание batiskaf

### trait SqlParam

```rust
fn to_named_params(&self, stmt: &Statement) -> Vec<(&str, &dyn ToSql)>;
```
Функция предназначена для преобразования структуры в именованные параметры SQL-запроса.


### trait SqlResult

```rust
fn from_row(row: &Row) -> rusqlite::Result<Self>;
```
Функция предназначена для преобразования строки результата запроса в структуру.


### trait SqlInsert

```rust
fn insert_statement(table: &str) -> String;
```
Вспомогательный trait для уменьшения бойлерплейта; его единственная функция предназначена для генерации SQL-запроса INSERT для создания записи в указанной таблице.


### trait SqlUpdate

```rust
fn update_statement(table: &str) -> String;
```
Функция генерирует SQL-запрос обновления записи в указанной таблице.


### trait SqlUpsert

```rust
fn upsert_statement(table: &str) -> String;
```
Функция генерирует SQL-запрос вставки или обновления записи в указанной таблице (UPSERT).


### trait SqlDelete

```rust
fn delete_statement(table: &str) -> String;
```
Функция возвращает SQL-запрос удаления записи из указанной таблицы.


### trait BatiskafConnection

Дополняет структуру `rusqlite::Connection` следующими функциями:

```rust
fn select_one<T: SqlResult>(&self, sql: &str, params: &[(&str, &dyn ToSql)]) -> rusqlite::Result<T>;
```
Функция-обёртка над `rusqlite::Connection::query_row_named`, преобразующая результат запроса в тип `T`.

```rust
fn select_many<T: SqlResult>(&self, sql: &str, params: &[(&str, &dyn ToSql)]) -> rusqlite::Result<Vec<T>>;
```
Функция-обёртка над `rusqlite::Connection::query_named`, преобразующая все строки результата запроса в тип `T`.

```rust
fn insert<T: SqlInsert + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<i64>;
```
Функция вставляет строку в таблицу `table`. SQL-код выражения `INSERT` генерируется функцией `T::insert_statement`, параметром для запроса является аргумент `value`. Функция возвращает `rowid` только что вставленной строки.

```rust
fn update<T: SqlUpdate + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<usize>;
```
Функция изменяет строки в таблице `table` на основании запроса `T::insert_statement` с параметрами `value` и возвращает количество изменённых строк.

```rust
fn upsert<T: SqlUpsert + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<i64>;
```
Функция вставляет строку в таблицу `table` или изменяет существующую на основании запроса `T::upsert_statement` с параметрами `value`. Функция возвращает `rowid` только что вставленной строки.

```rust
fn delete<T: SqlDelete + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<usize>;
```
Функция удаляет строки из таблицы `table` при помощи запроса `T::delete_statement` и значения `value` и возвращает количество удалённых строк.


## batiskaf_derive

Библиотека *batiskaf* сама по себе довольно бесполезная. Эту ситуацию исправляет библиотека *batiskaf_derive*, избавляя программиста от кучи бойлерплейта при помощи магии процедурных макросов.

Библиотека умеет выводить реализацию трейтов *batiskaf* для структур с именованными полями.

### Пример использования

```rust
use rusqlite::{Connection, NO_PARAMS};

use batiskaf::*;

#[derive(Debug, SqlParam, SqlResult, SqlInsert, SqlUpdate, SqlUpsert, SqlDelete)]
struct Person {
    #[batiskaf(primary_key, autogenerated)]
    pub id: i64,
    #[batiskaf(column = "full_name")]
    pub name: String,
    pub age: Option<u32>,
    #[batiskaf(skip, default)]
    pub hobby: Option<String>,
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "create table person (\
            id integer primary key, \
            full_name text not null, \
            age integer\
        )",
        NO_PARAMS,
    )
    .unwrap();

    let mut bob = Person {
        id: 0,
        name: "Bob".to_string(),
        age: Some(30),
        hobby: Some("wood carving".to_string()),
    };

    bob.id = conn.insert("person", &bob).unwrap();

    bob.age = Some(31);
    conn.update("person", &bob).unwrap();

    let stored_bob: Person = conn
        .select_one("select id, full_name, age from person where id = :id", &[(":id", &bob.id)])
        .unwrap();
    println!("{:?}", stored_bob); // Person { id: 1, name: "Bob", age: Some(31), hobby: None }

    conn.delete("person", &bob).unwrap();

    let nobody: Vec<Person> = conn
        .select_many("select id, full_name, age from person", &[])
        .unwrap();
    println!("{}", nobody.len()); // 0

    let mut mike = Person {
        id: 100,
        name: "Mike".to_string(),
        age: Some(35),
        hobby: None,
    };
    conn.upsert("person", &mike).unwrap();

    let stored_mike: Person = conn
        .select_one("select id, full_name, age from person where id = :id", &[(":id", &mike.id)])
        .unwrap();
    println!("{:?}", stored_mike); // Person { id: 100, name: "Mike", age: Some(35), hobby: None }

    mike.age = Some(36);
    conn.upsert("person", &mike).unwrap();

    let stored_mike: Person = conn
        .select_one("select id, full_name, age from person where id = :id", &[(":id", &mike.id)])
        .unwrap();
    println!("{:?}", stored_mike); // Person { id: 100, name: "Mike", age: Some(36), hobby: None }
}
```

### Арибуты

#### column = "column_name"
Переименование столбца. По-умолчанию название столбца в БД совпадает с названием поля структуры. Атрибут `column` задаёт другое название для соответствующего столбца в таблице.

Применяется во всех пяти трейтах.

#### primary_key
Поле (и соответствующий столбец) является первичным ключом. В том случае, когда этот атрибут указан для нескольких полей, соответствующие столбцы образуют составной первичный ключ. Первичные ключи используются при выводе `SqlUpdate`, `SqlUpsert` и `SqlDelete` для идентификации изменяемой (удаляемой) строки.

Учитывается при генерации `SqlUpdate`, `SqlUpsert` и `SqlDelete`. Если ни одно поле не будет иметь этот атрибут, будет ошибка компиляции.

Если все поля имеют атрибут `primary_key`, компиляция трейта `SqlUpdate` завершится с ошибкой.

#### autogenerated
Значение соответствуюего столбца является автогенерируемым и пропускается при генерации SQL-кода выражения `INSERT`, возвращаемого функцией `SqlInsert::insert_statement`.

Учитывается при выводе `SqlInsert`.

#### skip
Поле не используется в SQL-выражениях. Применяется ко всем пяти трейтам.

#### default
Если поле пропущено или его нет в результатах запроса (`SqlResult`), использовать значение по-умолчанию.

Может быть применено к структуре в целом; в таком случае каждое поле получает этот атрибут.
При автогенерации реализации `SqlResult` этот атрибут *должен* быть указан, если указан атрибут `skip`.


### Примечание

При выводе трейта `SqlResult` все generic-типы в объявлении структуры получают дополнительные ограничения: `::std::default::Default + ::rusqlite::types::FromSql`.
