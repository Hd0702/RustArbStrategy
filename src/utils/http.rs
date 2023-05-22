use reqwest::{Client, ClientBuilder};
use std::time::Duration;
use once_cell::sync::Lazy;
use crate::utils::traits::{ApplyTrait, LetTrait};

pub static CLIENT : Lazy<Client> = Lazy::new(|| ClientBuilder::new().timeout(Duration::from_secs(10)).build().expect("Failed to build client"));

impl<T> ApplyTrait for T {}
impl<T> LetTrait for T {}