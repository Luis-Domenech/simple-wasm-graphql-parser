use apollo_parser::ast::SelectionSet;
use crate::prelude::{Selection, REGEX, Config};

// TODO: Make this not recursive by modifying last added element to vector or hashmap
// Did it this way cause I had problems dealing with variable borrows, moves, etc
pub fn parse_selection_set(selection_set: SelectionSet, _config: &Config) -> Vec<Selection> {
    let mut selections: Vec<Selection> = vec![];
    
    let mut stopping_point = 0;
    let mut recurse_on_next: bool = false;

    for (index, line) in selection_set.to_string().lines().enumerate() {
        // Handle case where line starts with { and not a name field, which I believe is every case
        if index == 0 {
            let has_name = REGEX.match_first_word.captures(line).unwrap().is_none();
            if has_name {
                continue;
            }
        }

        if index > stopping_point || index == 0 {
            let name: Option<String> = match REGEX.match_first_word.captures(line).unwrap() {
                Some(cap) => Some(cap.get(0).unwrap().as_str().to_string()),
                None => None
            };
    
            if line.contains("{") {
                if name.is_some() {
                    // Here measn we have fieldName {
                    // After recursion, stopping point reflects where we should continue
                    stopping_point = index;
                    let selection = Selection {
                        name: name.unwrap(),
                        selections: Some(recurse_selection(&selection_set.to_string(), &mut stopping_point))
                    };

                    selections.push(selection);
                }
                else {
                    recurse_on_next = true;
                }
            }
            else if line.contains("}") {
                if name.is_some() {                
                    let selection = Selection {
                        name: name.unwrap(),
                        selections: None
                    };
    
                    selections.push(selection);
                }
                else {
                    continue;
                }
            }
            else {
                if name.is_some() {
                    if recurse_on_next {
                        stopping_point = index;
    
                        let selection = Selection {
                            name: name.unwrap(),
                            selections: Some(recurse_selection(&selection_set.to_string(), &mut stopping_point))
                        };
        
                        selections.push(selection);
    
                        recurse_on_next = false; // Reset it
                    }
                    else {
                        if is_parent(&selection_set.to_string(), index) {
                            stopping_point = index;
    
                            let selection = Selection {
                                name: name.unwrap(),
                                selections: Some(recurse_selection(&selection_set.to_string(), &mut stopping_point))
                            };
    
                            selections.push(selection);
                        }
                        else {
                            let selection = Selection {
                                name: name.unwrap(),
                                selections: None
                            };
    
                            selections.push(selection);
                        }
                    }
                }
                else {
                    continue;
                }
            }
        }
    }

    selections
}

fn recurse_selection(raw_string: &str, stopping_point: &mut usize) -> Vec<Selection> {
    let mut selections: Vec<Selection> = vec![];
    let mut recurse_on_next: bool = false;

    for (index, line) in raw_string.lines().enumerate() {
        if index > *stopping_point {

            let name: Option<String> = match REGEX.match_first_word.captures(line).unwrap() {
                Some(cap) => Some(cap.get(0).unwrap().as_str().to_string()),
                None => None
            };
        
            if line.contains("{") {
                if name.is_some() {
                    *stopping_point = index;
                    // Recursion updates stopping point, so on return, we know where to continue, so we won't rescan lines
                    let selection = Selection {
                        name: name.unwrap(),
                        selections: Some(recurse_selection(&raw_string, stopping_point))
                    };

                    selections.push(selection);
                }
                else {
                    // Here means we know that next name is going to be a parent, so no need to call the is_parent function on next name
                    recurse_on_next = true;
                }
            }
            else if line.contains("}") {
                if name.is_some() {                
                    let selection = Selection {
                        name: name.unwrap(),
                        selections: None
                    };

                    selections.push(selection);
                    
                    *stopping_point = index;
                    return selections
                }
                else {
                    // Here means we recursed all selections from parent
                    *stopping_point = index;
                    return selections
                }
            }
            else {
                if name.is_some() {
                    // Here we encountered a field. 
                    if recurse_on_next {
                        // We know this is a parent, so recurse
                        *stopping_point = index;

                        let selection = Selection {
                            name: name.unwrap(),
                            selections: Some(recurse_selection(&raw_string, stopping_point))
                        };
        
                        selections.push(selection);

                        recurse_on_next = false; // Reset it
                    }
                    else {
                        // We don't know if this is just a regular build or not
                        if is_parent(&raw_string, index) {
                            *stopping_point = index;

                            let selection = Selection {
                                name: name.unwrap(),
                                selections: Some(recurse_selection(&raw_string, stopping_point))
                            };

                            selections.push(selection);
                        }
                        else {
                            let selection = Selection {
                                name: name.unwrap(),
                                selections: None
                            };

                            selections.push(selection);
                        }
                    }
                }
                else {
                    // Here means we probably stumbled on some comment or whatever, so do nithing
                    continue;
                }
            }
        }
    }

    selections
}

fn is_parent(raw_string: &str, current_index: usize) -> bool {
    for (index, line) in raw_string.lines().enumerate() {
        if index > current_index {
            let name: Option<String> = match REGEX.match_first_word.captures(line).unwrap() {
                Some(cap) => Some(cap.get(0).unwrap().as_str().to_string()),
                None => None
            };

            if line.contains("{") {
                if name.is_some() {
                    // Here means fieldName }, but since we are in a further index than our stopping point,
                    // then this means that fieldName in this case is not related to the field name we called this for, hence false
                    return false
                }
                else {
                    return true
                }
            }
            else if line.contains("}") {
                if name.is_some() {
                    return false
                }
                else {
                    return false
                }
            }
            else {
                if name.is_some() {
                    return false
                }
                else {
                    // Here means we probably stumbled on some comment or whatever, so do continue
                    continue;
                }
            } 
        }
    }

    false
}