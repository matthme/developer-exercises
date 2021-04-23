use hdk::prelude::*;
use holo_hash::HeaderHashB64;

entry_defs![SnackingLog::entry_def(), WorkoutLog::entry_def()];

#[hdk_entry(id = "SnackingLog")]
pub struct SnackingLog(String);

#[hdk_entry(id = "WorkoutLog")]
pub struct WorkoutLog(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderHashPair{
    current: HeaderHashB64,
    previous: HeaderHashB64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Answer(String);


#[hdk_extern]
pub fn register_snacking(input: SnackingLog) -> ExternResult<HeaderHashB64> {
    Ok(HeaderHashB64::from(create_entry(&input)?))
}

#[hdk_extern]
pub fn is_previous_header_hash(pair: HeaderHashPair) -> ExternResult<Answer> {
    let current_header_hash_from_input:HeaderHash = HeaderHash::from(pair.current);
    let prev_header_hash_from_input:HeaderHash = HeaderHash::from(pair.previous);

    let element: Element = get(current_header_hash_from_input, GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find current element")))?;
    let header: Header = element.header().clone();
    let prev_header_option:Option<&HeaderHash> = header.prev_header();
    let prev_header:HeaderHash;

    match Some(prev_header_option) {
        Some(a) => prev_header = a
                                .ok_or(WasmError::Guest(String::from("Prev header is empty")))?
                                .clone(),
        None => return Ok(Answer(String::from("previous header NOT FOUND"))),
    }

    if prev_header != prev_header_hash_from_input {
        return Ok(Answer(String::from("is NOT previous header")))
    }

    Ok(Answer(String::from("IS previous header")))
}

#[hdk_extern]
pub fn happened_before(pair: HeaderHashPair) -> ExternResult<Answer> {
    let starting_point:HeaderHash = HeaderHash::from(pair.current);
    let potential_before:HeaderHash = HeaderHash::from(pair.previous);

    let mut current:Option<HeaderHash> = Some(starting_point);
    while current != None {
        let found:Option<HeaderHash> = get_previous_header(
            current
            .ok_or(WasmError::Guest(String::from("Prev header is empty")))?
            .clone());
        match found {
            Some(a) => {
                if a == potential_before {
                    let answer:Answer = Answer(String::from("happened before"));
                    return Ok(answer) 
                } else {
                    current = Some(a.clone());
                }
            },
            None => current = None,
        }
    }
    Ok(Answer(String::from("did NOT happen before")))
}

fn get_previous_header(a:HeaderHash) -> Option<HeaderHash> {

    let ext_result:ExternResult<Option<Element>> = get(a, GetOptions::default());
    let el_option:Option<Element> = ext_result.unwrap_or(None);
    
    match Some(el_option) {
        Some(element) => return Some(element.unwrap().header().clone().prev_header().unwrap().clone()),
        None => return None,
    }
}

#[hdk_extern]
pub fn get_header_sequence_number(a: HeaderHashB64) -> ExternResult<Answer> {
    let element: Element = get(HeaderHash::from(a), GetOptions::default())?
    .ok_or(WasmError::Guest(String::from("Could not find current element")))?;
    let header: Header = element.header().clone();
    let sequence:u32 =  header.header_seq();
    Ok(Answer(format!("header sequence is {}", sequence)))
}
