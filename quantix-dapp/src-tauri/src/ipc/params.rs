//! Params types used in the IPC methods.
//!
//! The current best practice is to follow a single argument type, called "params" for all method (JSON-RPC's style).
//!

use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct CreateParams<D> {
	pub data: D,
}

#[derive(Deserialize,Debug)]
pub struct UpdateParams<D> {
	pub id: String,
	pub data: D,
}

#[derive(Deserialize,Debug)]
pub struct ListParams<F> {
	pub filter: Option<F>,
}

#[derive(Deserialize,Debug)]
pub struct GetParams {
	pub id: String,
}

#[derive(Deserialize,Debug)]
pub struct DeleteParams {
	pub id: String,
}
