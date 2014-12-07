use std::error::Error;

#[deriving(PartialEq, Eq, Clone, Show)]
pub struct BeanstalkdError;

impl Error for BeanstalkdError {
    fn description(&self) -> &str { "Error with beanstalkd occured." }
}

pub type BeanstalkdResult<T> = Result<T, BeanstalkdError>;
