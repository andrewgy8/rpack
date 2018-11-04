use std::ops::Add;

/// Bin type represents what something is packed into.  
///
/// Bin.contents contains a `Vec<Item>`, representing the items of that packed bin.
#[derive(Debug, PartialEq)]
pub struct Bin {
    pub contents: Vec<Item>
}

impl Bin {

    /// Constructs a new `Bin` instance 
    ///
    /// # Examples
    ///
    /// ```
    /// use rpack::Bin;
    ///
    /// let mut bin = Bin::new();
    /// ```
    ///
    pub fn new() -> Bin {
        Bin{contents: vec![]}
    }
}

/// Item type for the packing  
///
/// Size represents the one-dimensional area that the Item consumes.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub struct Item {
    pub size: u32
}

impl Add for Item {
    type Output = u32;

    fn add(self, other: Item) -> u32 {
        self.size + other.size
    }
}

pub struct Packing {
    pub bin_size: u32
}

impl Packing {

    /// Constructs a new `Packing` instance 
    ///
    /// Requires a `bin_size<u32>` which will be the max size that any bin 
    /// can fit.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpack::Packing;
    ///
    /// let packing = Packing::new(5);
    /// ```
    ///
    /// `packing` will have a bin size of 5.
    pub fn new(bin_size: u32) -> Packing {
        Packing{ bin_size }
    }

    /// Main entrypoint for packing bins. 
    ///
    /// Requires a `Vec<Item>` representing the sizes of all the bins that will 
    /// be packed in the bins.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpack::{Bin, Packing, Item};
    /// let items = vec![
    ///     Item{size: 2}, 
    ///     Item{size: 4}, 
    ///     Item{size: 4}, 
    ///     Item{size: 6}, 
    ///     Item{size: 1}
    /// ];
    /// let expected = vec![
    ///     Bin{contents:vec![Item{size: 6}]}, 
    ///     Bin{contents:vec![Item{size: 4}, Item{size: 2}]}, 
    ///     Bin{contents:vec![Item{size: 4}, Item{size: 1}]},
    /// ];
    /// let mut packing = Packing{bin_size: 6};
    /// let packed_bins = packing.pack_items(items);
    ///    
    /// assert_eq!(packed_bins, expected);
    /// ```
    pub fn pack_items(&mut self, mut packages: Vec<Item>) -> Vec<Bin>{
        let mut bins: Vec<Bin> = vec![];

        packages.sort_by(|a, b| b.cmp(&a));

        if self.bin_size < packages.iter().max().unwrap().size {
            panic!("Bin size must be larger than max value.");
        }
        
        while !packages.is_empty() {

            let mut initial_bin = Bin::new();

            for package in packages.clone() {

                let res: u32 = initial_bin.contents.iter().fold(0,|a, &b| a + b.size);
                if (res + package.size) <= self.bin_size {
                    initial_bin.contents.push(package);
                    
                    let pos = packages.iter().position(|&x| x == package).unwrap();
                    
                    packages.remove(pos);
                }

                let after_sum: u32 = initial_bin.contents.iter().fold(0,|a, &b| a + b.size);
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
    
    use super::*;

    #[test]
    fn returns_bins_packed_with_items() {
        let items = vec![
            Item{size: 2}, 
            Item{size: 4}, 
            Item{size: 4}, 
            Item{size: 6}, 
            Item{size: 1}
        ];
        let expected = vec![
            Bin{contents:vec![Item{size: 6}]}, 
            Bin{contents:vec![Item{size: 4}, Item{size: 2}]}, 
            Bin{contents:vec![Item{size: 4}, Item{size: 1}]},
        ];
        let mut packing = Packing{bin_size: 6};
        let packed_bins = packing.pack_items(items);
        
        assert_eq!(packed_bins, expected);
    }

    #[test]
    fn returns_bins_packed_with_best_size() {
        let packages = vec![
            Item{size: 4}, 
            Item{size: 2}, 
            Item{size: 2}, 
            Item{size: 2}, 
            Item{size: 1}, 
            Item{size: 1}, 
            Item{size: 4}, 
            Item{size: 6}, 
            Item{size: 1}, 
            Item{size: 1}
        ];
        let expected = vec![
        	Bin{contents:vec![Item{size: 6}]},  
        	Bin{contents:vec![Item{size: 4}, Item{size: 2}]}, 
        	Bin{contents:vec![Item{size: 4}, Item{size: 2}]}, 
        	Bin{contents:vec![Item{size: 2}, Item{size: 1}, Item{size: 1}, Item{size: 1}, Item{size: 1}]}
        ];
        let mut packing = Packing{bin_size: 6};
        let packed_bins = packing.pack_items(packages);
        
        assert_eq!(packed_bins, expected);
    }

    #[test]
    fn returns_bins_when_last_bin_is_not_full() {
        let packages = vec![Item{size: 6}, Item{size: 4}, Item{size: 2}, Item{size: 2}, Item{size: 1}, Item{size: 1}, Item{size: 1}, Item{size: 1}, Item{size: 1}];
        let expected = vec![
        	Bin{contents:vec![Item{size: 6}]}, 
        	Bin{contents:vec![Item{size: 4}, Item{size: 2}]}, 
        	Bin{contents:vec![Item{size: 2}, Item{size: 1}, Item{size: 1}, Item{size: 1}, Item{size: 1}]},
        	Bin{contents:vec![Item{size: 1}]}
        ];

        let mut packing = Packing{bin_size: 6};
        let packed_bins = packing.pack_items(packages);
        
        assert_eq!(packed_bins, expected);
    }

    #[test]
    fn returns_bin_size_when_last_bin_not_perfect_size() {
        let packages = vec![
            Item{size: 6}, 
            Item{size: 4}, 
            Item{size: 2}, 
            Item{size: 2}, 
            Item{size: 1}, 
            Item{size: 1}, 
            Item{size: 1}, 
            Item{size: 1}, 
            Item{size: 1}
        ];
        let expected = vec![
        	Bin{contents:vec![Item{size: 6}, Item{size: 2}]}, 
        	Bin{contents:vec![Item{size: 4}, Item{size: 2}, Item{size: 1}, Item{size: 1}]}, 
        	Bin{contents:vec![Item{size: 1}, Item{size: 1}, Item{size: 1}]}
        ];

        let mut packing = Packing{bin_size: 8};
        let packed_bins = packing.pack_items(packages);
        
        assert_eq!(packed_bins, expected);
    }

    #[test]
    #[should_panic(expected="Bin size must be larger than max value.")]
    fn panics_when_bin_size_is_smaller_than_largest_given_size() {
        let packages = vec![Item{size: 6}, Item{size: 1}];
        let mut packing = Packing{bin_size: 2};

        packing.pack_items(packages);
    }
}
