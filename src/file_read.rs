pub mod read {
    use std::{ fs::File, io::{ BufRead, BufReader } };

    pub fn open_file(path: &str) -> Vec<String> {
        let mut haystack = Vec::new();
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(v) => {
                        haystack.push(v);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
        haystack
    }
}
