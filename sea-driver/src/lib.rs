use proc_macro::{self, TokenStream};

#[cfg(feature = "rusqlite")]
mod rusqlite;
#[cfg(feature = "sqlx-mysql")]
mod sqlx_mysql;
#[cfg(feature = "sqlx-postgres")]
mod sqlx_postgres;
#[cfg(feature = "sqlx-sqlite")]
mod sqlx_sqlite;
mod utils;

#[cfg(feature = "rusqlite")]
#[proc_macro]
pub fn sea_query_driver_rusqlite(_: TokenStream) -> TokenStream {
    rusqlite::sea_query_driver_rusqlite_impl()
}

#[cfg(feature = "sqlx-mysql")]
#[proc_macro]
pub fn sea_query_driver_mysql(_: TokenStream) -> TokenStream {
    sqlx_mysql::sea_query_driver_mysql_impl()
}

#[cfg(feature = "sqlx-mysql")]
#[proc_macro]
pub fn bind_params_sqlx_mysql(input: TokenStream) -> TokenStream {
    utils::bind_params_sqlx_impl(input)
}

#[cfg(feature = "sqlx-postgres")]
#[proc_macro]
pub fn sea_query_driver_postgres(_: TokenStream) -> TokenStream {
    sqlx_postgres::sea_query_driver_postgres_impl()
}

#[cfg(feature = "sqlx-postgres")]
#[proc_macro]
pub fn bind_params_sqlx_postgres(input: TokenStream) -> TokenStream {
    utils::bind_params_sqlx_impl(input)
}

#[cfg(feature = "sqlx-sqlite")]
#[proc_macro]
pub fn sea_query_driver_sqlite(_: TokenStream) -> TokenStream {
    sqlx_sqlite::sea_query_driver_sqlite_impl()
}

#[cfg(feature = "sqlx-sqlite")]
#[proc_macro]
pub fn bind_params_sqlx_sqlite(input: TokenStream) -> TokenStream {
    utils::bind_params_sqlx_impl(input)
}
