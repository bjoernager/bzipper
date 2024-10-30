// Copyright 2024 Gabriel Bjørnager Jensen.

use rand::random;

// Bincode uses so much memory that it crashes if
// we set `VALUE_COUNT` too high.
const VALUE_COUNT: usize = 0x0FFFFFFF;

use std::time::Instant;

macro_rules! benchmark {
	{
		$($name:ident: {
			bincode: $bincode_op:block$(,)?

			borsh: $borsh_op:block$(,)?

			bzipper: $bzipper_op:block$(,)?

			ciborium: $ciborium_op:block$(,)?

			postcard: $postcard_op:block$(,)?
		}$(,)?)+
	 } => {{
		use ::std::{concat, eprint, eprintln, stringify};

		macro_rules! time {
			{ $op: block } => {{
				let start = Instant::now();
				$op
				let stop = Instant::now();

				let duration = stop - start;
				(duration.as_nanos() as f64) / 1_000_000_000.0
			}};
		}

		fn format_score(duration: f64, ref_duration: f64) -> String {
			let vps = (VALUE_COUNT as f64 / duration).round();

			let difference = (duration / ref_duration - 1.0) * 100.0;

			let colour: u8 = if difference >= 0.0 {
				0x20
			} else if difference < 0.0 {
				0x1F
			} else {
				0x00
			};

			format!("{duration:.3}s ({vps:.0} vps) => \u{001B}[{colour}m{difference:+.2}%\u{001B}[000m")
		}

		let mut total_bincode_duration  = 0.0;
		let mut total_borsh_duration    = 0.0;
		let mut total_bzipper_duration  = 0.0;
		let mut total_ciborium_duration = 0.0;
		let mut total_postcard_duration = 0.0;

		$({
			eprintln!();
			eprintln!(concat!("\u{001B}[001mrunning benchmark `", stringify!($name), "`...\u{001B}[022m"));
			eprint!("\u{001B}[093m");

			let bincode_duration  = time! { $bincode_op };
			let borsh_duration    = time! { $borsh_op };
			let bzipper_duration  = time! { $bzipper_op };
			let ciborium_duration = time! { $ciborium_op };
			let postcard_duration = time! { $postcard_op };

			eprint!("\u{001B}[000m");
			eprintln!("bincode:  {}", format_score(bincode_duration,  bzipper_duration));
			eprintln!("borsh:    {}", format_score(borsh_duration,    bzipper_duration));
			eprintln!("bzipper:  {}", format_score(bzipper_duration,  bzipper_duration));
			eprintln!("ciborium: {}", format_score(ciborium_duration, bzipper_duration));
			eprintln!("postcard: {}", format_score(postcard_duration, bzipper_duration));

			total_bincode_duration  += bincode_duration;
			total_borsh_duration    += borsh_duration;
			total_bzipper_duration  += bzipper_duration;
			total_ciborium_duration += ciborium_duration;
			total_postcard_duration += postcard_duration;
		})*

		eprintln!();
		eprintln!("\u{001B}[001mtotal score:\u{001B}[022m");
		eprintln!("bincode:  {}", format_score(total_bincode_duration,  total_bzipper_duration));
		eprintln!("borsh:    {}", format_score(total_borsh_duration,    total_bzipper_duration));
		eprintln!("bzipper:  {}", format_score(total_bzipper_duration,  total_bzipper_duration));
		eprintln!("ciborium: {}", format_score(total_ciborium_duration, total_bzipper_duration));
		eprintln!("postcard: {}", format_score(total_postcard_duration, total_bzipper_duration));
	}};
}

#[derive(bzipper::Decode, bzipper::SizedEncode)]
#[derive(borsh::BorshSerialize)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Unit;

#[derive(bzipper::Decode, bzipper::SizedEncode)]
#[derive(borsh::BorshSerialize)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Unnamed(u32);

impl Unnamed {
	#[inline(always)]
	#[must_use]
	pub const fn from_char(value: char) -> Self {
		Self(value as u32)
	}
}

#[derive(bzipper::Decode, bzipper::SizedEncode)]
#[derive(borsh::BorshSerialize)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Named { buf: [u8; 0x8] }

#[derive(bzipper::Decode, bzipper::SizedEncode)]
#[derive(borsh::BorshSerialize)]
#[derive(serde::Deserialize, serde::Serialize)]
enum Enum {
	Unit(Unit),
	Unnamed(Unnamed),
	Named(Named),
}

impl Named {
	#[inline(always)]
	#[must_use]
	pub const fn from_u64(value: u64) -> Self {
		let buf = [
			 (value & 0x00_00_00_00_00_00_00_FF)         as u8,
			((value & 0x00_00_00_00_00_00_FF_00) >> 0x2) as u8,
			((value & 0x00_00_00_00_00_FF_00_00) >> 0x4) as u8,
			((value & 0x00_00_00_00_FF_00_00_00) >> 0x6) as u8,
			((value & 0x00_00_00_FF_00_00_00_00) >> 0x8) as u8,
			((value & 0x00_00_FF_00_00_00_00_00) >> 0xA) as u8,
			((value & 0x00_FF_00_00_00_00_00_00) >> 0xC) as u8,
			((value & 0xFF_00_00_00_00_00_00_00) >> 0xE) as u8,
		];

		Self { buf }
	}
}

fn main() {
	println!("######################");
	println!("# BZIPPER BENCHMARKS #");
	println!("######################");
	println!();
	println!("Each benchmark has a version written for the following crates:");
	println!();
	println!("- Bincode:  <https://crates.io/crates/bincode/>");
	println!("- Borsh:    <https://crates.io/crates/borsh/>");
	println!("- Ciborium: <https://crates.io/crates/ciborium/>");
	println!("- bzipper:  <https://crates.io/crates/bzipper/>");
	println!("- Postcard: <https://crates.io/crates/postcard/>");
	println!();
	println!("The total time the benchmark took (including memory allocations and dealloca-");
	println!("tions) is printed when the benchmark has completed. The `vps` counter signifies");
	println!("the amount of values per second that the benchmark has handled (during the en-");
	println!("tirety of the benchmark).");
	println!();
	println!("When every benchmark has concluded, the total run time and vps is listed for");
	println!("each crate. A percantage additionally compares the run time between the listed");
	println!("crate and bzipper (which should always be `0%` for bzipper itself). DO NOTE THAT");
	println!("THESE FINAL RESULTS INDICATE A NON-WEIGHTED AVERAGE ACROSS BENCHMARKS. It can");
	println!("therefore be skewed relative to real-world performance by the similarity of some");
	println!("benchmarks.");
	println!();

	eprintln!("value_count: {VALUE_COUNT}");

	benchmark! {
		encode_u8: {
			bincode: {
				// Requires `std`.

				use bincode::serialize_into;
				use std::io::Cursor;

				let buf_size = size_of::<u8>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT]);

				for _ in 0x0..VALUE_COUNT {
					serialize_into(&mut buf, &random::<u8>()).unwrap();
				}
			}

			borsh: {
				use std::io::Cursor;

				let buf_size = size_of::<u8>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT]);

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &random::<u8>()).unwrap();
				}
			}

			bzipper: {
				use bzipper::{Encode, OStream, SizedEncode};

				let buf_size = u8::MAX_ENCODED_SIZE; // value

				let mut buf = vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice();
				let mut stream = OStream::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					random::<u8>().encode(&mut stream).unwrap();
				}
			}

			ciborium: {
				use std::io::Cursor;

				let buf_size =
					size_of::<u8>()    // header
					+ size_of::<u8>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT]);

				for _ in 0x0..VALUE_COUNT {
					ciborium::into_writer(&random::<u8>(), &mut buf).unwrap();
				}
			}

			postcard: {
				use std::io::Cursor;

				let buf_size = size_of::<u8>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT]);

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&random::<u8>(), &mut buf).unwrap();
				}
			}
		}

		encode_struct_unit: {
			bincode: {
				use bincode::serialize_into;
				use std::io::Cursor;

				let buf_size = size_of::<Unit>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT]);

				for _ in 0x0..VALUE_COUNT {
					serialize_into(&mut buf, &Unit).unwrap();
				}
			}

			borsh: {
				use std::io::Cursor;

				let buf_size = size_of::<Unit>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &Unit).unwrap();
				}
			}

			bzipper: {
				use bzipper::{Encode, OStream, SizedEncode};

				let buf_size = Unit::MAX_ENCODED_SIZE; // value

				let mut buf = vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice();
				let mut stream = OStream::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					Unit.encode(&mut stream).unwrap();
				}
			}

			ciborium: {
				use std::io::Cursor;

				let buf_size =
					size_of::<u8>()      // header
					+ size_of::<Unit>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					ciborium::into_writer(&Unit, &mut buf).unwrap();
				}
			}

			postcard: {
				use std::io::Cursor;

				let buf_size = size_of::<Unit>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&Unit, &mut buf).unwrap();
				}
			}
		}

		encode_struct_unnamed: {
			bincode: {
				use bincode::serialize_into;
				use std::io::Cursor;

				let buf_size = size_of::<Unnamed>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT]);

				for _ in 0x0..VALUE_COUNT {
					serialize_into(&mut buf, &Unnamed::from_char(random())).unwrap();
				}
			}

			borsh: {
				use std::io::Cursor;

				let buf_size = size_of::<Unnamed>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &Unnamed::from_char(random())).unwrap();
				}
			}

			bzipper: {
				use bzipper::{Encode, OStream, SizedEncode};

				let buf_size = Unnamed::MAX_ENCODED_SIZE; // value

				let mut buf = vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice();
				let mut stream = OStream::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					Unnamed::from_char(random()).encode(&mut stream).unwrap();
				}
			}

			ciborium: {
				use std::io::Cursor;

				let buf_size =
					size_of::<u8>()         // header
					+ size_of::<Unnamed>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					ciborium::into_writer(&Unnamed::from_char(random()), &mut buf).unwrap();
				}
			}

			postcard: {
				use std::io::Cursor;

				let buf_size = size_of::<Unnamed>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&Unnamed::from_char(random()), &mut buf).unwrap();
				}
			}
		}

		encode_struct_named: {
			bincode: {
				use bincode::serialize_into;
				use std::io::Cursor;

				let buf_size = size_of::<Named>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT]);

				for _ in 0x0..VALUE_COUNT {
					serialize_into(&mut buf, &Named::from_u64(random())).unwrap();
				}
			}

			borsh: {
				use std::io::Cursor;

				let buf_size = size_of::<Named>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &Named::from_u64(random())).unwrap();
				}
			}

			bzipper: {
				use bzipper::{Encode, OStream, SizedEncode};

				let buf_size = Named::MAX_ENCODED_SIZE; // value

				let mut buf = vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice();
				let mut stream = OStream::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					Named::from_u64(random()).encode(&mut stream).unwrap();
				}
			}

			ciborium: {
				use std::io::Cursor;

				let buf_size =
					size_of::<u8>()       // header
					+ size_of::<u64>()    // tag
					+ size_of::<u8>()     // header
					+ size_of::<Named>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					ciborium::into_writer(&Named::from_u64(random()), &mut buf).unwrap();
				}
			}

			postcard: {
				use std::io::Cursor;

				let buf_size = size_of::<Named>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&Named::from_u64(random()), &mut buf).unwrap();
				}
			}
		}

		encode_enum_unit: {
			bincode: {
				use bincode::serialize_into;
				use std::io::Cursor;

				let buf_size =
					size_of::<u32>()     // discriminant
					+ size_of::<Unit>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT]);

				for _ in 0x0..VALUE_COUNT {
					serialize_into(&mut buf, &Enum::Unit(Unit)).unwrap();
				}
			}

			borsh: {
				use std::io::Cursor;

				let buf_size =
					size_of::<u8>()      // discriminant
					+ size_of::<Unit>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &Enum::Unit(Unit)).unwrap();
				}
			}

			bzipper: {
				use bzipper::{Encode, OStream, SizedEncode};

				let buf_size =
					isize::MAX_ENCODED_SIZE   // discriminant
					+ Unit::MAX_ENCODED_SIZE; // value

				let mut buf = vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice();
				let mut stream = OStream::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					Enum::Unit(Unit).encode(&mut stream).unwrap();
				}
			}

			ciborium: {
				use std::io::Cursor;

				let buf_size =
					size_of::<u8>()      // header
					+ size_of::<u64>()   // tag (discriminant)
					+ size_of::<u8>()    // header
					+ size_of::<Unit>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					ciborium::into_writer(&Enum::Unit(Unit), &mut buf).unwrap();
				}
			}

			postcard: {
				use std::io::Cursor;

				let buf_size =
					size_of::<u32>()     // discriminant
					+ size_of::<Unit>(); // value

				let mut buf = Cursor::new(vec![0x00; buf_size * VALUE_COUNT].into_boxed_slice());

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&Enum::Unit(Unit), &mut buf).unwrap();
				}
			}
		}
	};
}