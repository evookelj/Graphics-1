fn main() {
	use std::error::Error;
	use std::fs::File;
	use std::io::prelude::*;
	use std::path::Path;

	static HEADER: &'static str = 
		"P3\n500 500 255\n";

    let path = Path::new("image.ppm");
    let display = path.display();

    // Open file for writing
    let mut file = match File::create(&path) {
        Err(why) => panic!("Error creating {} because {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write HEADER string to file
    match file.write_all(HEADER.as_bytes()) {
        Err(why) => {
            panic!("Error writing to {} because {}", display,
                                               why.description())
        },
        Ok(_) => println!("No error writing to {}", display),
    }

    for i in 0..500 {
    	for j in 0..500 {
    		let r = j % 255;
    		let g = j % 255;
    		let b = j % 255;
    		match file.write_all(format!("{} {} {}\n",r,g,b).as_bytes()) {
    			Err(why) => {
    				panic!("Error writing pixels to {} because {}", display,
    					why.description())
    			},
    			Ok(_) => println!("No error writing pixels"),
    		}
    	}
    }
}
