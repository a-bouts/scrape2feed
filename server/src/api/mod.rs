use rocket::http::Status;

mod model;
pub(crate) mod v1;

#[get("/healthz")]
pub(crate) fn healthz() -> Status {
    Status::Ok
}
