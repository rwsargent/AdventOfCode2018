use std::fs::File;
use std::error;

pub type Result<T> = std::result::Result<T, Box<error::Error>>;
