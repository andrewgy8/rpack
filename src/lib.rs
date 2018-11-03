#[derive(Debug, PartialEq)]
pub struct Bin {
    contents: Vec<u32>
}

pub struct Packing {
    pub bin_size: u32
}

impl Packing {

    pub fn pack_bins(&mut self, mut packages: Vec<u32>) -> Vec<Bin>{
    	let mut bins: Vec<Bin> = vec![];

    	packages.sort_by(|a, b| b.cmp(a));

    	if self.bin_size < *packages.iter().max().unwrap() {
    		panic!("Bin size must be larger than max value.");
    	}
    	
    	while !packages.is_empty() {

    		let mut initial_bin = Bin{contents: vec![]};

    		for package in packages.clone() {

    			let res: u32 = initial_bin.contents.iter().sum();
    			if (res + package) <= self.bin_size {
    				initial_bin.contents.push(package);
    				
    				let pos = packages.iter().position(|&x| x == package).unwrap();
    				
    				packages.remove(pos);
    			}

    			let after_sum: u32 = initial_bin.contents.iter().sum();
    			if (after_sum == self.bin_size) | (packages.is_empty()) {
    				bins.push(initial_bin);

    				break;
    			}
    		}
    	}
    	bins
    }
}

#[cfg(test)]
mod tests {
    
    use Packing;
    use Bin;

    #[test]
    fn returns_bins_packed_with_best_size() {
        let packages = vec![2, 4, 4, 2, 2, 2, 6, 1, 1, 6, 4, 6, 1, 1];
        let expected = vec![
        	Bin{contents:vec![6]}, 
        	Bin{contents:vec![6]}, 
        	Bin{contents:vec![6]}, 
        	Bin{contents:vec![4, 2]}, 
        	Bin{contents:vec![4, 2]}, 
        	Bin{contents:vec![4, 2]}, 
        	Bin{contents:vec![2, 1, 1, 1, 1]}
        ];
        let mut packing = Packing{bin_size: 6};
        let packed_bins = packing.pack_bins(packages);
        
        assert_eq!(packed_bins, expected);
    }

    #[test]
    fn returns_bins_when_last_bin_is_not_full() {
        let packages = vec![6, 4, 2, 2, 1, 1, 1, 1, 1];
        let expected = vec![
        	Bin{contents:vec![6]}, 
        	Bin{contents:vec![4, 2]}, 
        	Bin{contents:vec![2, 1, 1, 1, 1]},
        	Bin{contents:vec![1]}
        ];

        let mut packing = Packing{bin_size: 6};
        let packed_bins = packing.pack_bins(packages);
        
        assert_eq!(packed_bins, expected);
    }

    #[test]
    fn returns_bin_size_when_last_bin_not_perfect_size() {
        let packages = vec![6, 4, 2, 2, 1, 1, 1, 1, 1];
        let expected = vec![
        	Bin{contents:vec![6, 2]}, 
        	Bin{contents:vec![4, 2, 1, 1]}, 
        	Bin{contents:vec![1, 1, 1]}
        ];

        let mut packing = Packing{bin_size: 8};
        let packed_bins = packing.pack_bins(packages);
        
        assert_eq!(packed_bins, expected);
    }

    #[test]
    #[should_panic]
    fn panics_when_bin_size_is_smaller_than_largest_given_size() {
        let packages = vec![6, 4, 2, 2, 1, 1, 1, 1, 1];

        let mut packing = Packing{bin_size: 2};
        packing.pack_bins(packages);
    }
}
