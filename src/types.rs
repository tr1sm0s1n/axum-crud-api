use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::models::Certificate;

pub type Db = Arc<RwLock<HashMap<u32, Certificate>>>;
