use std::fmt::Debug;
use serde::{Deserialize, Serialize};

pub trait ModelResponse {
    type Response<'a>: Serialize + Deserialize<'a> + Debug where Self: 'a;
    
    fn to_response(&self) -> Self::Response<'_>;
}