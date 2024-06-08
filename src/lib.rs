// Copyright 2022-2024 Gabriel Bjørnager Jensen.

//! Binary (de)serialisation.
//!
//! Contrary to [Serde](https://crates.io/crates/serde/)/[Bincode](https://crates.io/crates/bincode/), the goal of `bzipper` is to serialise data without inflating the resulting binary sequence.
//! As such, one may consider this crate to be more low-level.
//!
//! Keep in mind that this project is still work-in-progress.
//!
//! This crate does not require any dependencies at the moment.

pub mod deserialise;
pub mod error;
pub mod serialise;

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(in crate) use use_mod;
