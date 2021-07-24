pub fn foo() {
    let some_value = bar();
    println!("some_value: {}", some_value);
}

fn bar() -> i128 {
    123
} 

pub fn foo2() {
    let some_value = bar2();
    println!("some_value: {}", some_value);
}

fn bar2() -> i32 {
    123
} 
#[cfg(test)]
mod tests {
    use rls_data::{DefKind,Analysis};
    #[test]
    fn forbid_i128() {
        let data = std::fs::read_to_string("saved_analysis.json").unwrap();
        let analysis : Analysis = serde_json::from_str(&data).unwrap();
        let mut errors = vec![];
        for def in analysis.defs.iter() {
            match def.kind {
                DefKind::Local => {
                    if def.value == "i128" {
                        
                        errors.push(def);
                    }
                }
                _ => {}
            }
        }
        if errors.len() > 0 {
            for err in errors.iter() {
                eprintln!("ERROR: found i128 variable named {} in file {:?} line {:?} ", &err.name, &err.span.file_name, &err.span.line_start);
            }
            panic!("i128 found in code")
        }
    }
}
