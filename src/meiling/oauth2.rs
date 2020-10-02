use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::MysqlConnection;

use crate::database::model;

use super::objects;
use crate::meiling::objects::oauth2::NewTokenResponse;

pub fn get_token(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    new_token_request: objects::oauth2::NewTokenRequest,
) -> Result<objects::oauth2::NewTokenResponse, objects::oauth2::OAuth2Error> {
    let client: model::Client = {
        use crate::database::schema::client::dsl::*;
        use diesel::prelude::*;

        client
            .filter(id.eq(new_token_request.client_id.as_bytes().to_vec()))
            .filter(secret.eq(new_token_request.client_secret.to_string()))
            .get_result(conn)
            .map_err(|_| objects::oauth2::OAuth2Error::Unknown)
    }?;

    Ok(NewTokenResponse {})
}
