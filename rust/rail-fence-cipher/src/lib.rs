pub struct RailFence {
    rails: u32
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let cycle = (0..self.rails)
            .chain((1..(self.rails - 1)).rev())
            .cycle();
        let mut fence = vec![vec![]; self.rails as usize];

        text.chars()
            .zip(cycle)
            .for_each(|(c, rail)| {
                fence[rail as usize].push(c);
            });

        fence.into_iter()
            .flat_map(|x| x)
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let cycle_size: usize = ((self.rails - 1) * 2) as usize;
        let full_cycles = cipher.len() / cycle_size;
        let partial_cycles = (cipher.len() % cycle_size) as u32;
        let mut fence = Vec::new();
        let mut chars = cipher.chars();

        (0..self.rails).for_each(|rail| {
            let is_partial_cycle = match rail < partial_cycles {
                true => 1,
                false => 0,
            };
            let to_add = if rail == 0 {
                chars.by_ref()
                    .take(full_cycles + is_partial_cycle)
                    .collect::<Vec<char>>()
                    .into_iter()
            } else if rail == self.rails - 1 {
                chars.by_ref()
                    .take(full_cycles + is_partial_cycle)
                    .collect::<Vec<char>>()
                    .into_iter()
            } else {
                chars.by_ref()
                    .take(2 * full_cycles + is_partial_cycle)
                    .collect::<Vec<char>>()
                    .into_iter()
            };
            fence.push(to_add);
        });

        let cycle = (0..self.rails).chain((1..(self.rails - 1))
            .rev())
            .cycle();
        let mut decoded = String::new();
        let mut size = cipher.len();

        for rail in cycle {
            decoded.push(fence[rail as usize]
                .next()
                .unwrap());
            size -= 1;
            if size == 0 {
                break;
            }
        }
        decoded
    }
}