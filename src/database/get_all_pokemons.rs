use crate::errors::{self, AnyError, DatabaseError};
use crate::graphql::schemas::pokemon::Pokemon;
use error_stack::{IntoReport, Result, ResultExt};

use super::Database;

impl Database {
    pub async fn get_all_pokemon(&self) -> Result<Vec<Pokemon>, AnyError> {
        
        let sql = "SELECT id, name, games_occurrence FROM pokemons";
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

        let results= results
        .into_iter()
        .next().
        map(|r| r.result)
        .transpose()
        .into_report()
        .change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))?.ok_or(AnyError::DatabaseError(DatabaseError::ParseData))?;


        let json = serde_json::to_string(&results).into_report().change_context(AnyError::DatabaseError(DatabaseError::ParseData))?;
        let pokemons: Vec<Pokemon> = serde_json::from_str(&json).into_report().change_context(AnyError::DatabaseError(DatabaseError::ParseData))?;

        return Ok(pokemons);
        // Err(AnyError::DatabaseError(DatabaseError::Other)).into_report()
    }
}