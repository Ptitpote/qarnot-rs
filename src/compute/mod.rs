/// Low level compute client
pub mod client;
/// Low level compute Models
pub mod models;
/// High level task manipulation
pub mod task;

//TODO job/pools/hwconstraints/paginate

/// Errors that may happen when using Qarnot's compute API
#[derive(Debug)]
pub enum ComputeError {
    /// 401 unauthorized
    Unauthorized,
    /// 403 forbidden
    Forbidden,
    /// 404 not found
    NotFound,
    /// Other kind of error
    Generic,
}
