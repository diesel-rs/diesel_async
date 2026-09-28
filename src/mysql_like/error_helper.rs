use core::marker::PhantomData;

#[cfg(feature = "mariadb")]
use diesel::mariadb::Mariadb;
#[cfg(feature = "mysql")]
use diesel::mysql::Mysql;
use diesel::mysql_like::MysqlLikeBackend;
use diesel::result::DatabaseErrorKind;
use diesel::ConnectionError;
use mysql_async::Error;

pub(super) struct ErrorHelper<DB: MysqlLikeBackend>(pub(super) Error, pub(super) PhantomData<DB>);

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

#[cfg(feature = "mysql")]
impl From<ErrorHelper<Mysql>> for diesel::result::Error {
    fn from(ErrorHelper(e, _): ErrorHelper<Mysql>) -> Self {
        match e {
            Error::Server(e) => {
                let kind = match e.code {
                    1062 | 1586 | 1859 => DatabaseErrorKind::UniqueViolation,
                    1216 | 1217 | 1451 | 1452 | 1830 | 1834 => {
                        DatabaseErrorKind::ForeignKeyViolation
                    }
                    1792 => DatabaseErrorKind::ReadOnlyTransaction,
                    1048 | 1364 => DatabaseErrorKind::NotNullViolation,
                    3819 => DatabaseErrorKind::CheckViolation,
                    _ => DatabaseErrorKind::Unknown,
                };
                diesel::result::Error::DatabaseError(kind, Box::new(e.message) as _)
            }
            e => diesel::result::Error::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new(e.to_string()) as _,
            ),
        }
    }
}

#[cfg(feature = "mariadb")]
impl From<ErrorHelper<Mariadb>> for diesel::result::Error {
    fn from(ErrorHelper(e, _): ErrorHelper<Mariadb>) -> Self {
        match e {
            Error::Server(e) => {
                let kind = match e.code {
                    1022 | 1062 | 1586 | 1859 => DatabaseErrorKind::UniqueViolation,
                    1216 | 1217 | 1451 | 1557 | 1452 | 1830 | 1834 => {
                        DatabaseErrorKind::ForeignKeyViolation
                    }
                    1792 => DatabaseErrorKind::ReadOnlyTransaction,
                    1048 | 1364 => DatabaseErrorKind::NotNullViolation,
                    4025 => DatabaseErrorKind::CheckViolation,
                    1213 => DatabaseErrorKind::SerializationFailure,
                    _ => DatabaseErrorKind::Unknown,
                };
                diesel::result::Error::DatabaseError(kind, Box::new(e.message) as _)
            }
            e => diesel::result::Error::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new(e.to_string()) as _,
            ),
        }
    }
}
