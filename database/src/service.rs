use std::env;

use diesel::{Connection, RunQueryDsl, SqliteConnection, connection, query_dsl::methods::FindDsl};
use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use dotenvy::dotenv;

use crate::model::{Dare, DbType, NewDare, Rating, Status, UpdateDare, UpdateTruth};

use crate::schema::dares::dsl::{dares, id as d_id};
use crate::schema::dares::status;
use crate::schema::truths::dsl::{id as t_id, truths};
use crate::{model::NewTruth, model::Truth};
pub struct DbService {
    connection: SqliteConnection,
}

impl DbService {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        let url = env::var("DATABASE_URL").expect("URL NOT SET");
        let connection = SqliteConnection::establish(&url).unwrap();

        DbService { connection }
    }
    //truth stuff

    pub fn create_truth(&mut self, new_truth: NewTruth) -> QueryResult<Truth> {
        diesel::insert_into(crate::schema::truths::table)
            .values(&new_truth)
            .returning(crate::schema::truths::all_columns)
            .get_result(&mut self.connection)
    }
    pub fn list_truths(&mut self) -> Result<Vec<Truth>, diesel::result::Error> {
        let test = crate::schema::truths::table.load::<Truth>(&mut self.connection)?;
        Ok(test)
    }
    pub fn get_truth_by_id(&mut self, truth_id: i32) -> Result<Truth, diesel::result::Error> {
        truths
            .filter(t_id.eq(truth_id))
            .first::<Truth>(&mut self.connection)
    }
    pub fn update_truth(
        &mut self,
        truth_id: i32,
        updated_truth: UpdateTruth,
    ) -> Result<Truth, diesel::result::Error> {
        diesel::update(truths.filter(t_id.eq(truth_id)))
            .set(updated_truth)
            .returning(crate::schema::truths::all_columns)
            .get_result(&mut self.connection)
    }
    pub fn delete_truth(&mut self, truth_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(truths.filter(t_id.eq(truth_id))).execute(&mut self.connection)
    }
    pub fn list_truths_filtered(
        &mut self,
        rating_filter: Option<Rating>,
        status_filter: Option<Status>,
    ) -> QueryResult<Vec<Truth>> {
        let mut query = truths.into_boxed();

        if let Some(rating_val) = rating_filter {
            query = query.filter(crate::schema::truths::rating.eq(rating_val));
        }

        if let Some(status_val) = status_filter {
            query = query.filter(crate::schema::truths::status.eq(status_val));
        }

        query.load::<Truth>(&mut self.connection)
    }
    pub fn get_random_truth(&mut self, rating_filter: Option<Rating>) -> QueryResult<Truth> {
        let mut query = truths.into_boxed();
        if let Some(rating_val) = rating_filter {
            query = query.filter(crate::schema::truths::rating.eq(rating_val));
        } else {
            query = query.filter(crate::schema::truths::rating.eq(Rating::SFW));
        }

        query = query.filter(crate::schema::truths::status.eq(Status::ACCEPTED));

        query
            .order(diesel::dsl::sql::<diesel::sql_types::Integer>("RANDOM()"))
            .limit(1)
            .first::<Truth>(&mut self.connection)
    }
    pub fn accept_truth(&mut self, truth_id: i32) -> Result<Truth, diesel::result::Error> {
        let updated_truth = UpdateTruth {
            content: None,
            rating: None,
            status: Some(Status::ACCEPTED),
        };
        diesel::update(truths.filter(t_id.eq(truth_id)))
            .set(updated_truth)
            .returning(crate::schema::truths::all_columns)
            .get_result(&mut self.connection)
    }
    // dare stuff
    pub fn create_dare(&mut self, new_dare: NewDare) -> QueryResult<Dare> {
        diesel::insert_into(crate::schema::dares::table)
            .values(&new_dare)
            .returning(crate::schema::dares::all_columns)
            .get_result(&mut self.connection)
    }
    pub fn list_dares(&mut self) -> Result<Vec<Dare>, diesel::result::Error> {
        let test = crate::schema::dares::table.load::<Dare>(&mut self.connection)?;
        Ok(test)
    }
    pub fn get_dare_by_id(&mut self, dare_id: i32) -> Result<Dare, diesel::result::Error> {
        dares
            .filter(d_id.eq(dare_id))
            .first::<Dare>(&mut self.connection)
    }
    pub fn update_dare(
        &mut self,
        dare_id: i32,
        updated_dare: UpdateDare,
    ) -> Result<Dare, diesel::result::Error> {
        diesel::update(dares.filter(d_id.eq(dare_id)))
            .set(updated_dare)
            .returning(crate::schema::dares::all_columns)
            .get_result(&mut self.connection)
    }
    pub fn delete_dare(&mut self, dare_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(dares.filter(d_id.eq(dare_id))).execute(&mut self.connection)
    }
    pub fn list_dares_filtered(
        &mut self,
        rating_filter: Option<Rating>,
        status_filter: Option<Status>,
    ) -> QueryResult<Vec<Dare>> {
        let mut query = dares.into_boxed();

        if let Some(rating_val) = rating_filter {
            query = query.filter(crate::schema::dares::rating.eq(rating_val));
        }

        if let Some(status_val) = status_filter {
            query = query.filter(crate::schema::dares::status.eq(status_val));
        }

        query.load::<Dare>(&mut self.connection)
    }
    pub fn get_random_dare(&mut self, rating_filter: Option<Rating>) -> QueryResult<Dare> {
        let mut query = dares.into_boxed();
        if let Some(rating_val) = rating_filter {
            query = query.filter(crate::schema::dares::rating.eq(rating_val));
        } else {
            query = query.filter(crate::schema::dares::rating.eq(Rating::SFW));
        }

        query = query.filter(crate::schema::dares::status.eq(Status::ACCEPTED));

        query
            .order(diesel::dsl::sql::<diesel::sql_types::Integer>("RANDOM()"))
            .limit(1)
            .first::<Dare>(&mut self.connection)
    }
    pub fn accept_dare(&mut self, dare_id: i32) -> Result<Dare, diesel::result::Error> {
        let updated_dare = UpdateDare {
            content: None,
            rating: None,
            status: Some(Status::ACCEPTED),
        };
        diesel::update(dares.filter(d_id.eq(dare_id)))
            .set(updated_dare)
            .returning(crate::schema::dares::all_columns)
            .get_result(&mut self.connection)
    }
    // accept
    pub fn accept(&mut self, db_type: DbType, id: i32) -> Result<(), diesel::result::Error> {
        match db_type {
            DbType::Truth => {
                let truth = self.accept_truth(id)?;
                Ok(())
            }
            DbType::Dare => {
                let dare = self.accept_dare(id)?;
                Ok(())
            }
        }
    }
    pub fn delete(&mut self, db_type: DbType, id: i32) -> Result<(), diesel::result::Error> {
        match db_type {
            DbType::Truth => {
                let truth = self.delete_truth(id)?;
                Ok(())
            }
            DbType::Dare => {
                let dare = self.delete_dare(id)?;
                Ok(())
            }
        }
    }
}
