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
        Ok(_) => (),
    }

    for i in 0..500 {
    	for j in 0..500 {
            //let r = j%255;
            let g = (((i*i+j*j)/4)*(255/4))%255;
            let r = (g*2)%255;
            let b = (r*2)%255;
    		match file.write_all(format!("{} {} {}\n",r,g,b).as_bytes()) {
    			Err(why) => {
    				panic!("Error writing pixels to {} because {}", display,
    					why.description())
    			},
    			Ok(_) => (),
    		}
    	}
    }
}
