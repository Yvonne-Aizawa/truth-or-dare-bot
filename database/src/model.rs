use chrono::NaiveDateTime;
use diesel::{
    AsChangeset, Queryable,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::{Identifiable, Insertable},
    serialize::{self, Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
};
use poise::ChoiceParameter;
#[derive(ChoiceParameter, Copy, Clone, Debug)]
pub enum DbType {
    #[name = "Truth Question"]
    Truth,
    #[name = "Dare Challange"]
    Dare,
}
//implement display for DbType
impl std::fmt::Display for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbType::Truth => write!(f, "Truth Question"),
            DbType::Dare => write!(f, "Dare Challange"),
        }
    }
}
#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::truths)]
pub struct Truth {
    id: i32,
    content: String,
    author: String,
    rating: Rating,
    status: Status,
    submit_date: NaiveDateTime,
}
impl Truth {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn content(&self) -> String {
        self.content.clone()
    }
    pub fn author(&self) -> String {
        self.author.clone()
    }
    pub fn rating(&self) -> Rating {
        self.rating
    }
    pub fn status(&self) -> Status {
        self.status
    }
    pub fn submit_date(&self) -> NaiveDateTime {
        self.submit_date
    }
}
#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::truths)]
pub struct UpdateTruth {
    pub content: Option<String>,
    pub rating: Option<Rating>,
    pub status: Option<Status>,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::truths)]
pub struct NewTruth<'a> {
    pub content: &'a str,
    pub author: String,
    pub rating: Rating,
    pub status: Status,
    pub submit_date: NaiveDateTime,
}
//dares
use crate::schema::dares;
#[derive(Queryable, Debug, Identifiable)]
pub struct Dare {
    id: i32,
    content: String,
    author: String,
    rating: Rating,
    status: Status,
    submit_date: NaiveDateTime,
}
impl Dare {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn content(&self) -> String {
        self.content.clone()
    }
    pub fn author(&self) -> String {
        self.author.clone()
    }
    pub fn rating(&self) -> Rating {
        self.rating
    }
    pub fn status(&self) -> Status {
        self.status
    }
    pub fn submit_date(&self) -> NaiveDateTime {
        self.submit_date
    }
}
#[derive(AsChangeset)]
#[diesel(table_name = dares)]
pub struct UpdateDare {
    pub content: Option<String>,
    pub rating: Option<Rating>,
    pub status: Option<Status>,
}

#[derive(Insertable)]
#[diesel(table_name = dares)]
pub struct NewDare<'a> {
    pub content: &'a str,
    pub author: String,
    pub rating: Rating,
    pub status: Status,
    pub submit_date: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow, ChoiceParameter, PartialEq)]
#[diesel(sql_type = Text)]
pub enum Rating {
    SFW,
    NSFW,
}
//to string
impl ToString for Rating {
    fn to_string(&self) -> String {
        match self {
            Rating::SFW => "SFW".to_string(),
            Rating::NSFW => "NSFW".to_string(),
        }
    }
}
impl ToSql<Text, Sqlite> for Rating {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let s = match self {
            Rating::SFW => "SFW",
            Rating::NSFW => "NSFW",
        };
        out.set_value(s);
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for Rating {
    fn from_sql<'a, 'b, 'c>(mut value: SqliteValue<'a, 'b, 'c>) -> deserialize::Result<Self> {
        let s = value.read_text();
        match s {
            "SFW" => Ok(Rating::SFW),
            "NSFW" => Ok(Rating::NSFW),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

//for status
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow, ChoiceParameter, PartialEq)]
#[diesel(sql_type = Text)]
pub enum Status {
    PENDING,
    ACCEPTED,
    REJECTED,
}
//to string
impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::PENDING => "PENDING".to_string(),
            Status::ACCEPTED => "ACCEPTED".to_string(),
            Status::REJECTED => "REJECTED".to_string(),
        }
    }
}
impl ToSql<Text, Sqlite> for Status {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let s = match self {
            Status::ACCEPTED => "ACCEPTED",
            Status::PENDING => "PENDING",
            Status::REJECTED => "REJECTED",
        };
        out.set_value(s);
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for Status {
    fn from_sql<'a, 'b, 'c>(mut value: SqliteValue<'a, 'b, 'c>) -> deserialize::Result<Self> {
        let s = value.read_text();
        match s {
            "ACCEPTED" => Ok(Status::ACCEPTED),
            "PENDING" => Ok(Status::PENDING),
            "REJECTED" => Ok(Status::REJECTED),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// Moderation

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::moderation)]
pub struct Moderation {
    id: i32,
    moderation_type: String, // Use String for flexibility
    kind: String,
    item_id: i32,
    moderator_id: String,
    reason: Option<String>,
    timestamp: NaiveDateTime,
}
impl Moderation {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn moderation_type(&self) -> String {
        self.moderation_type.clone()
    }
    pub fn kind(&self) -> String {
        self.kind.clone()
    }
    pub fn item_id(&self) -> i32 {
        self.item_id
    }
    pub fn moderator_id(&self) -> String {
        self.moderator_id.clone()
    }
    pub fn reason(&self) -> Option<String> {
        self.reason.clone()
    }
    pub fn timestamp(&self) -> NaiveDateTime {
        self.timestamp
    }
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::moderation)]
pub struct NewModeration<'a> {
    pub moderation_type: String,
    pub kind: &'a str,
    pub item_id: i32,
    pub moderator_id: String,
    pub reason: Option<&'a str>,
    pub timestamp: NaiveDateTime,
}
