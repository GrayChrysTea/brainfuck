#[macro_export]
macro_rules! someorreturn {
    ($opt: expr, $otherwise: expr) => {
        {
            match $opt {
                Some(thing) => thing,
                None => return $otherwise,
            }
        }
    };
}

#[macro_export]
macro_rules! okorreturn {
    ($opt: expr, $otherwise: expr) => {
        {
            match $opt {
                Ok(ay) => ay,
                Err(_) => return $otherwise,
            }
        }
    };
}