pub struct IntCode {
    pub input: Vec<u32>,
    pointer: usize,
}

impl IntCode {
    pub fn new(input: &Vec<u32>) -> IntCode {
        IntCode {
            input: input.clone(),
            pointer: 0,
        }
    }

    pub fn patch(&mut self, index: usize, value: u32) {
        self.input[index] = value
    }

    pub fn run(&mut self) -> u32 {
        self.pointer = 0;

        loop {
            match self.input[self.pointer] {
                99 => break self.input[0],
                opcode => {
                    let noun = self.input[self.input[self.pointer + 1] as usize];
                    let verb = self.input[self.input[self.pointer + 2] as usize];
                    let output_position = self.input[self.pointer + 3] as usize;

                    self.input[output_position] = match opcode {
                        1 => noun + verb,
                        2 => noun * verb,
                        _ => panic!("Unexpected code {}", opcode),
                    };

                    self.pointer += 4;
                }
            }
        }
    }
}
