use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

pub struct Compiler {
    pub map_melt: u8,
    pub map_ice: u8,
    pub map_freeze: u8,
}

impl Compiler {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u8;

        Compiler {
            map_melt: seed.wrapping_add(13),
            map_ice: seed.wrapping_mul(7),
            map_freeze: seed.wrapping_xor(0x3C),
        }
    }

    pub fn compile(&self, source_path: &str, output_path: &str) {
        let mut output = Vec::new();

        output.push(self.map_melt);
        output.push(self.map_ice);
        output.push(self.map_freeze);

        let mut file = File::create(output_path).expect("FAULT_CREATE_GLAZE");
        file.write_all(&output).expect("FAULT_WRITE_GLAZE");
    }
}
