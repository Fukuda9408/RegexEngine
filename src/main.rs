use regex::regex::Regex;

fn main() {
    // let accepts = vec![1].into_iter().collect();
    // let dfa = dfa::DeterministicFiniteAutomaton::new(1, accepts, transition_dfa);
    // let inputs = ["ab", "ba", "abab"];
    // for &input in inputs.into_iter() {
    //     let mut runtime = dfa.get_runtime();
    //     if runtime.doea_accept(input.as_bytes()) {
    //         println!("{} is accepted.", input);
    //     } else {
    //         println!("{} is not accepted.", input);
    //     }
    // }
    // let str = "(a|b)".to_string();
    // let mut lexer = lexer::Lexer::new(str);
    // println!("{:?}", lexer.scan());
    // println!("{:?}", lexer.scan());
    // println!("{:?}", lexer.scan());
    // println!("{:?}", lexer.scan());
    // println!("{:?}", lexer.scan());
    // println!("{:?}", lexer.scan());
    // println!("{:?}", lexer.scan());
    let regex = "(a|b)*".to_string();
    let regexp = Regex::new(regex).unwrap();
    if regexp.matches("aa".to_string()) {
        println!("match");
    } else {
        println!("not match");
    }
}

// fn transition_nfa(state: i32, character: Option<u8>) -> Result<HashSet<i32>, String> {
//     match state {
//         0 => {
//             match character.unwrap() {
//                 b'a' => return Ok(vec![1, 2].into_iter().collect()),
//                 _ => return Err("Not".to_string())
//             }
//         },
//         1 => {
//             match character.unwrap() {
//                 b'b' => return Ok(vec![1].into_iter().collect()),
//                 _ => return Err("Not".to_string())
//             }
//         },
//         2 => {
//             match character {
//                 Some(_) => return Err("Not".to_string()),
//                 None => return Ok(vec![0].into_iter().collect()),
//             }
//         },
//         _ => return Err("Not".to_string())
//     }
// }

fn transition_dfa(state: i32, character: u8) -> i32 {
    if state == 1 && character == b'a' {
        return 2
    } else if state == 2 && character == b'b' {
        return 1
    } else {
        return 0
    }
}
