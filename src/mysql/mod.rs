use std::borrow::Cow;

use diesel::connection::statement_cache::QueryFragmentForCachedStatement;
use diesel::mysql::Mysql;
use diesel::prelude::QueryResult;

use crate::mysql_like::UrlHelper;
use crate::stmt_cache::QueryFragmentHelper;

use super::mysql_like::AsyncMysqlLikeConnection;
use super::mysql_like::MysqlLikeCancelToken;

/// The capability to request cancellation of in-progress queries on a
/// connection.
pub type MysqlCancelToken = MysqlLikeCancelToken<Mysql>;

/// A connection to a MySQL database. Connection URLs should be in the form
/// `mysql://[user[:password]@]host/database_name`
pub type AsyncMysqlConnection = AsyncMysqlLikeConnection<Mysql>;

impl QueryFragmentForCachedStatement<Mysql> for QueryFragmentHelper {
    fn construct_sql(&self, _backend: &Mysql) -> QueryResult<String> {
        Ok(self.sql.clone())
    }

    fn is_safe_to_cache_prepared(&self, _backend: &Mysql) -> QueryResult<bool> {
        Ok(self.safe_to_cache)
    }
}

impl UrlHelper for Mysql {
    //`mysql_async` checks this for us
    fn check_and_replace_schema<'a>(url: &'a str) -> Result<Cow<'a, str>, mysql_async::UrlError> {
        Ok(url.into())
    }
}
