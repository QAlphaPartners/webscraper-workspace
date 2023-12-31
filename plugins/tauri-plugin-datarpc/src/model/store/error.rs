use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	FailToCreatePool(String),

	// -- Externals
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

}

impl From<sqlx::Error> for Error {
	fn from(val: sqlx::Error) -> Self {
		Self::Sqlx(val)
	}
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
