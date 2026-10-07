use std::borrow::Cow;

use diesel::connection::statement_cache::QueryFragmentForCachedStatement;
use diesel::mariadb::Mariadb;
use diesel::prelude::QueryResult;

use crate::mysql_like::UrlHelper;
use crate::stmt_cache::QueryFragmentHelper;

use super::mysql_like::AsyncMysqlLikeConnection;
use super::mysql_like::MysqlLikeCancelToken;

/// The capability to request cancellation of in-progress queries on a
/// connection.
pub type MariadbCancelToken = MysqlLikeCancelToken<Mariadb>;

/// A connection to a Mariadb database. Connection URLs should be in the form
/// `mariadb://[user[:password]@]host/database_name`
pub type AsyncMariadbConnection = AsyncMysqlLikeConnection<Mariadb>;

impl QueryFragmentForCachedStatement<Mariadb> for QueryFragmentHelper {
    fn construct_sql(&self, _backend: &Mariadb) -> QueryResult<String> {
        Ok(self.sql.clone())
    }

    fn is_safe_to_cache_prepared(&self, _backend: &Mariadb) -> QueryResult<bool> {
        Ok(self.safe_to_cache)
    }
}

impl UrlHelper for Mariadb {
    //`mysql_async` exprects `mysql` as schema
    fn check_and_replace_schema<'a>(url: &'a str) -> Result<Cow<'a, str>, mysql_async::UrlError> {
        let mut url = url::Url::parse(url)?;
        let scheme = url.scheme();
        if scheme != "mariadb" {
            return Err(mysql_async::UrlError::UnsupportedScheme {
                scheme: scheme.to_string(),
            });
        }
        let _ = url.set_scheme("mysql");
        Ok(url.to_string().into())
    }
}
