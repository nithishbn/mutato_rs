use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
pub fn insert_mutation_in_sequence(sequence: &mut String, mutation: &String) -> Result<()> {
    let (wt, pos_1_index, mutant) = parse_mutation(mutation)?;
    let pos = pos_1_index - 1;
    if pos > sequence.chars().count() {
        return Err(anyhow!(format!(
            "Mutation position {pos} is greater than sequence length {sequence_len}",
            sequence_len = sequence.chars().count()
        )));
    }
    if let Some(char_vec) = sequence.chars().nth(pos) {
        if char_vec == wt {
            sequence.replace_range(pos..pos + 1, &mutant.to_string());
        } else {
            return Err(anyhow!(format!(
                "Position {pos} does not contain {wt} but instead contains {char_vec}"
            )));
        }
    }
    return Ok(());
}

pub fn parse_mutation(mutation: &String) -> Result<(char, usize, char)> {
    let re = Regex::new("(?<wt>[A-Z])(?<pos>[1-9].*)(?<mut>[A-Z])").unwrap();
    let Some(mutant) = re.captures(mutation) else {
        let var_name = Err(anyhow!(format!("No match found for {mutation}")));
        return var_name;
    };
    return Ok((
        mutant["wt"].chars().next().expect("string is empty"),
        mutant["pos"].parse::<usize>().expect("invalid integer"),
        mutant["mut"].chars().next().expect("string is empty"),
    ));
}
