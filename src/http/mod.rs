// for not importing the method module in the main but directly use struct or enum from http
pub use method::Method;
pub use request::Request;

pub mod request;
pub mod method;