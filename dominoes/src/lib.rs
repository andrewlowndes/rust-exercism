use std::collections::HashMap;

type Domino = (u8, u8);

fn find_complete_chain(
    domino_index: &HashMap<u8, Vec<&Domino>>,
    total_dominos: usize,
    domino: &Domino,
    start_num: u8,
) -> Option<Vec<Domino>> {
    let mut result = vec![];

    let arranged_domino = if domino.0 != start_num {
        (domino.1, domino.0)
    } else {
        domino.clone()
    };

    result.push(arranged_domino);

    let mut domino_index_clone = domino_index.clone();

    let first_links = domino_index_clone.get_mut(&arranged_domino.0)?;
    first_links
        .iter()
        .position(|i| i == &domino)
        .map(|i| first_links.remove(i));

    let next_set = domino_index_clone.get_mut(&arranged_domino.1)?;
    next_set
        .iter()
        .position(|i| i == &domino)
        .map(|i| next_set.remove(i));

    next_set
        .to_vec()
        .iter()
        .map(|item| {
            find_complete_chain(
                &domino_index_clone,
                total_dominos - 1,
                item,
                arranged_domino.1,
            )
        })
        .filter(|item| item.is_some())
        .next()
        .map(|remaining_chain| result.append(&mut remaining_chain.unwrap()));

    if result.len() != total_dominos {
        return None;
    }

    Some(result)
}

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    let mut domino_index: HashMap<u8, Vec<&Domino>> = HashMap::new();

    input.iter().for_each(|domino| {
        domino_index
            .entry(domino.0)
            .and_modify(|v| {
                v.push(domino);
            })
            .or_insert_with(|| vec![domino]);
        domino_index
            .entry(domino.1)
            .and_modify(|v| {
                v.push(domino);
            })
            .or_insert_with(|| vec![domino]);
    });

    if domino_index.len() == 0 {
        return Some(vec![]);
    }

    let result = find_complete_chain(&domino_index, input.len(), &input[0], input[0].0)?;

    if result.first()?.0 != result.last()?.1 {
        return None;
    }

    Some(result)
}
