# Genders

[![Build Status](https://travis-ci.org/DoubleCouponDay/genders.svg)](https://travis-ci.org/DoubleCouponDay/genders)
[![Crates.io](https://img.shields.io/crates/v/genders.svg)](https://crates.io/crates/genders)
[![dependency status](https://deps.rs/repo/github/DoubleCouponDay/genders/status.svg)](https://deps.rs/repo/github/DoubleCouponDay/genders)

A forward thinking rust genders implementation

### Quick Example

```Rust
use genders::Gender;

assert_eq!(Gender::Male, Gender::Male);   /* Will succeed */
assert_eq!(Gender::Male, Gender::Female); /* Will fail */
assert!(Gender::Male>Gender::Female);	  /* Will succeed */

for gender in Gender::iter() {
	match gender {
		Gender::Male => println!("{}: The male gender", gender),
		Gender::Female => println!("{}: The female gender", gender),
	}
}
```