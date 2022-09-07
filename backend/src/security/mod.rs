mod security_errors;
use security_errors::Error;

pub struct UserCtx {
    pub user_id: i64,
}

pub async fn utx_from_token(token: &str) -> Result<UserCtx, Error> {
    match token.parse::<i64>() {
        Ok(user_id) => Ok(UserCtx { user_id }),
        Err(_) => Err(Error::InvalidToken(token.to_string())),
    }
}