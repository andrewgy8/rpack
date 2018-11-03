# rpack
[![Build Status](https://travis-ci.org/andrewgy8/rpack.svg?branch=master)](https://travis-ci.org/andrewgy8/rpack)

1D packing algorithm implementation written in Rust.

## Usage

	let packages = vec![6, 4, 2, 2, 1, 1, 1, 1, 1];
    let expected = vec![
    	Bin{contents:vec![6]}, 
    	Bin{contents:vec![4, 2]}, 
    	Bin{contents:vec![2, 1, 1, 1, 1]},
    	Bin{contents:vec![1]}
    ];

    let mut packing = Packing{bin_size: 6};
    let packed_bins = packing.pack_bins(rectangles);
    
    assert_eq!(packed_bins, expected);

## Running the tests

	$ cargo test
	$ cargo clippy

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/andrewgy8/rpack/tags). 

## Authors

* **Andrew Graham-Yooll** 

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
