use std::fs::File;
use std::io::Read;

pub struct LogicController {
    pub raw_logic: Vec<u8>,
}

impl LogicController {
    pub fn init(path: &str) {
        let mut file = File::open(path).expect("FAULT_OPEN_FILE");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("FAULT_READ_FILE");

        let controller = LogicController {
            raw_logic: buffer,
        };

        controller.process();
    }

    fn process(&self) {
        let content = String::from_utf8_lossy(&self.raw_logic);
        for line in content.lines() {
            let chunk = line.trim();
            if chunk.is_empty() {
                continue;
            }
            self.route_logic(chunk);
        }
    }

    fn route_logic(&self, data: &str) {
        println!("LGC_PROC: {}", data);
    }
}
