/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2025-07-17 11:01:17
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-17 20:36:44
 * @FilePath: /jigsaw-puzzle-rust/src/time.rs
 * @Description:
 *
 * Copyright (c) 2025 by goodpeanuts, All Rights Reserved.
 */
use std::ops::Div;

#[cfg(target_arch = "wasm32")]
pub(crate) fn get_performance_time() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now() // 返回毫秒，精度更高
}

pub(crate) fn get_instant_time() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        get_performance_time()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as f64 // 转换为毫秒
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeDelta(f64);

impl TimeDelta {
    pub fn milliseconds(milliseconds: impl Into<f64>) -> Self {
        TimeDelta(milliseconds.into())
    }

    pub fn seconds(seconds: impl Into<f64>) -> Self {
        TimeDelta(seconds.into() * 1000.0) // 转换为毫秒
    }

    pub fn as_milliseconds(&self) -> f64 {
        self.0
    }

    pub fn as_seconds(&self) -> f64 {
        self.0.div(1000.0)
    }

    pub fn num_minutes(&self) -> i64 {
        (self.as_seconds().div(60.0)) as i64
    }

    pub fn num_seconds(&self) -> i64 {
        (self.as_seconds() % 60.0) as i64
    }
}

impl std::ops::Sub for TimeDelta {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        TimeDelta(self.0 - other.0)
    }
}

#[derive(Clone, Copy)]

/// timestamp as millisecond
pub struct TimeStamp(f64);

impl TimeStamp {
    pub fn instant() -> TimeStamp {
        TimeStamp(get_instant_time())
    }

    pub fn elapsed(&self) -> TimeDelta {
        TimeDelta::milliseconds(get_instant_time() - self.0)
    }
}
