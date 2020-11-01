#![feature(split_inclusive)]
#![feature(option_result_contains)]

use bitvec::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|num| {
            if *num == 0 {
                return vec![0];
            }

            let bits = num.view_bits::<Msb0>();
            let last_chunk_index = ((bits.len() as f32) / 7.0).ceil() as usize - 1;

            let new_bits = bits
                .rchunks(7)
                .rev()
                .enumerate()
                .skip_while(|(_, chunk)| !chunk.any())
                .map(|(index, chunk)| {
                    let last_bit = {
                        if index == last_chunk_index {
                            0
                        } else {
                            1
                        }
                    };

                    let mut new_block = bitvec![Msb0, u8; last_bit];

                    new_block.extend(vec![false; 7 - chunk.len()]);
                    new_block.extend(chunk);

                    new_block
                });

            new_bits
                .map(|bitset| bitset.load::<u8>())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut results = vec![];

    if bytes.len() == 0 {
        return Ok(results);
    }

    if bytes
        .last()
        .map(|last_block| last_block.view_bits::<Msb0>()[0])
        .contains(&true)
    {
        return Err(Error::IncompleteNumber);
    }

    for num_blocks in bytes.split_inclusive(|byte| !byte.view_bits::<Msb0>()[0]) {
        let mut result_bits = BitVec::<Msb0, u32>::new();

        num_blocks.iter().for_each(|block| {
            result_bits.extend(block.view_bits::<Msb0>()[1..].to_bitvec());
        });

        let unpadded_bits = result_bits
            .iter()
            .skip_while(|bit| !**bit)
            .collect::<BitVec<Msb0, u32>>();

        if unpadded_bits.is_empty() {
            results.push(0);
            continue;
        }

        if unpadded_bits.len() > 32 {
            return Err(Error::Overflow);
        }

        results.push(unpadded_bits.load::<u32>());
    }

    Ok(results)
}
