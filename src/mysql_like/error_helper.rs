use core::marker::PhantomData;

use diesel::mysql_like::MysqlLikeBackend;
use diesel::result::DatabaseErrorKind;
use diesel::ConnectionError;
use mysql_async::Error;

pub(super) struct ErrorHelper<DB: MysqlLikeBackend>(pub(super) Error, PhantomData<DB>);

impl<DB: MysqlLikeBackend> ErrorHelper<DB> {
    pub(super) fn new(e: Error) -> Self {
        ErrorHelper(e, PhantomData)
    }
}

impl<DB: MysqlLikeBackend> From<ErrorHelper<DB>> for diesel::result::ConnectionError
where
    diesel::result::Error: From<ErrorHelper<DB>>,
{
    fn from(ErrorHelper(e, ph): ErrorHelper<DB>) -> Self {
        match e {
            Error::Driver(e) => ConnectionError::BadConnection(e.to_string()),
            Error::Io(e) => ConnectionError::BadConnection(e.to_string()),
            Error::Other(e) => ConnectionError::BadConnection(e.to_string()),
            Error::Server(_) => {
                ConnectionError::CouldntSetupConfiguration(ErrorHelper(e, ph).into())
            }
            Error::Url(e) => ConnectionError::InvalidConnectionUrl(e.to_string()),
        }
    }
}

impl<DB: MysqlLikeBackend> From<ErrorHelper<DB>> for diesel::result::Error {
    fn from(ErrorHelper(e, _): ErrorHelper<DB>) -> Self {
        match e {
            Error::Server(e) => {
                let kind = DB::map_error_number(e.code.into());
                diesel::result::Error::DatabaseError(kind, Box::new(e.message) as _)
            }
            e => diesel::result::Error::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new(e.to_string()) as _,
            ),
        }
    }
}
