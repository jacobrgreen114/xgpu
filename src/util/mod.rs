// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

pub trait FailFast {
    fn fail_fast(self) -> Self;
}

impl<T, E: std::fmt::Debug> FailFast for Result<T, E> {
    fn fail_fast(self) -> Self {
        match self {
            Ok(value) => Ok(value),
            Err(error) => {
                if cfg!(feature = "validation") {
                    panic!("{:?}", error)
                }
                Err(error)
            }
        }
    }
}

pub mod stack_vec;
