extern crate rpack;

use rpack::{Bin, Item, Packing};


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