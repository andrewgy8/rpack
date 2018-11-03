#[derive(Debug)]
#[derive(PartialEq)]
struct Bin {
    contents: Vec<u32>
}

fn pack_bins(mut packages: Vec<u32>) -> Vec<Bin>{
	let mut bins: Vec<Bin> = vec![];
	packages.sort_by(|a, b| b.cmp(a));

	while packages.len() > 0{

		let mut initial_bin = Bin{contents: vec![packages.remove(0)]};

		for package in packages.clone() {

			let res: u32 = initial_bin.contents.iter().sum();
			if (res + package) <= 6 {

				initial_bin.contents.push(package);
				

				let pos = packages.iter().position(|&x| x == package).unwrap();
				packages.remove(pos);	

			}

			let after_sum: u32 = initial_bin.contents.iter().sum();
			if after_sum == 6{
				bins.push(initial_bin);

				break;
			}
		}
	}

	bins
}

#[cfg(test)]
mod tests {
    
    use pack_bins;
    use Bin; 


    #[test]
    fn returns_bins_packed_with_best_size() {
        let rectangles = vec![6, 6, 6, 4, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1];
        let expected = vec![
        	Bin{contents:vec![6]}, 
        	Bin{contents:vec![6]}, 
        	Bin{contents:vec![6]}, 
        	Bin{contents:vec![4, 2]}, 
        	Bin{contents:vec![4, 2]}, 
        	Bin{contents:vec![4, 2]}, 
        	Bin{contents:vec![2, 1, 1, 1, 1]}
        ];

        let packed_bins = pack_bins(rectangles);
        
        assert_eq!(packed_bins, expected);
    }
}
