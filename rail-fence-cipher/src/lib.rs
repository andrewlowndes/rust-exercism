enum IterateDirection {
    Up,
    Down,
}

pub fn chain_iterator(num_rails: usize) -> impl Iterator<Item = usize> {
    let mut chain_index = 0;
    let mut chain_dir = IterateDirection::Down;
    let last_chain_index = num_rails - 1;

    (0..).map(move |_| {
        let result = chain_index;

        match chain_dir {
            IterateDirection::Down => {
                chain_index += 1;

                if chain_index == last_chain_index {
                    chain_dir = IterateDirection::Up;
                }
            }
            IterateDirection::Up => {
                chain_index -= 1;

                if chain_index == 0 {
                    chain_dir = IterateDirection::Down;
                }
            }
        }

        result
    })
}

pub struct RailFence {
    num_rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            num_rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        if self.num_rails < 2 {
            return text.to_string();
        }

        let mut chains: Vec<Vec<char>> = vec![vec![]; self.num_rails];

        text.chars()
            .zip(chain_iterator(self.num_rails))
            .for_each(|(char, chain_index)| {
                chains[chain_index].push(char);
            });

        chains
            .into_iter()
            .map(|chain| chain.iter().collect::<String>())
            .collect::<String>()
    }

    pub fn decode(&self, cipher: &str) -> String {
        //create placeholder chains
        let mut chains: Vec<Vec<Option<char>>> = vec![vec![]; self.num_rails];
        cipher
            .chars()
            .zip(chain_iterator(self.num_rails))
            .for_each(|(_, chain_index)| {
                chains[chain_index].push(None);
            });

        //fill the chains
        let mut load_str = cipher.chars();
        chains.iter_mut().for_each(|chain| {
            chain.iter_mut().for_each(|item| {
                item.replace(load_str.next().unwrap());
            });
        });

        //now just iterate the length and read back the chain values
        let mut chain_iterators = chains.iter().map(|chain| chain.iter()).collect::<Vec<_>>();

        cipher
            .chars()
            .zip(chain_iterator(self.num_rails))
            .map(|(_, chain_index)| chain_iterators[chain_index].next().unwrap().unwrap())
            .collect::<String>()
    }
}
