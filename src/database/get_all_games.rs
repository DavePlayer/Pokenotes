use crate::errors::{self, AnyError, DatabaseError};
use crate::graphql::schemas::game::Game;
use error_stack::{IntoReport, Result, ResultExt};

use super::Database;

impl Database {
    pub async fn get_all_games(&self) -> Result<Vec<Game>, AnyError> {
        let sql = "SELECT id, name, pokemons from games";
        let results = self.connection
            .execute(sql, &self.session, None, false)
            .await
            .into_report()
            .attach_printable(format!("error with database"))
            .change_context(AnyError::DatabaseError(errors::DatabaseError::ExecuteSQL(
                "couldn't CREATE pokemon in table".into(),
                sql.to_string(),
            )))?;
        // Database::print_surreal_response(&results)?;
        let result = results
        .into_iter()
        .next().
        map(|r| r.result)
        .transpose()
        .into_report()
        .change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))?;
        if let Some(res) = result {
           let json = serde_json::to_string(&res).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))?;
           let games: Vec<Game> = serde_json::from_str(&json).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData)).attach_printable("kill me not working aaaaaa")?;
           return Ok(games);
        }
        // let games: Vec<Game> = results.take(0);
        Ok(vec![])
    }
}