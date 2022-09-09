use crate::block::Block;

pub struct Level {
    pub blocks: Vec<Block>
}

pub mod loader {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, ErrorKind};
    use crate::block::create;
    use crate::level::Level;

    pub fn load(path: &str) -> Result<Level, Error> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut level = Level {blocks: vec![]};
        for line in buf_reader.lines().filter(|l| l.is_ok()).map(|l| l.unwrap()) {
            let data = line.split(';').collect::<Vec<&str>>();/*.filter_map(|e| e.parse::<u32>().ok())*/
            if data.len() < 4 {
                return Err(Error::new(ErrorKind::InvalidInput, "A line need to have at least four arguments separated with a ';'"))
            }
            let operation = *data.get(0).unwrap();
            let block_data = data.iter().filter_map(|d| d.parse::<u32>().ok()).collect::<Vec<u32>>();
            match operation {
                "b" => level.blocks.push(create(block_data)),
                "r" => {
                    if block_data.len() < 5 {
                        return Err(Error::new(ErrorKind::InvalidInput, "'r' operation need 5 numbers"))
                    }
                    let block_id = *block_data.get(0).unwrap();
                    let min_x = *block_data.get(1).unwrap();
                    let min_y = *block_data.get(2).unwrap();
                    let max_x = *block_data.get(3).unwrap();
                    let max_y = *block_data.get(4).unwrap();
                    for x in min_x..=max_x {
                        for y in max_y..=min_y {
                            level.blocks.push(create(vec![block_id, x, y]));
                        }
                    }
                },
                _ => return Err(Error::new(ErrorKind::InvalidInput,  format!("no operation '{operation}' known")))
            }
        }
        Ok(level)
    }

}




