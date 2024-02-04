// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// fn trim_me(input: &str) -> String {
//     // TODO: Remove whitespace from both ends of a string!

//     let mut beginIdx=0;
//     let mut lastIdx=0;
//     let mut triggerOnce = false;
//     for (index,c) in input.char_indices(){
//         if c!=' '{
//             if triggerOnce==false{
//             beginIdx=index;
//             triggerOnce=true;
//             }
//             lastIdx=index;
//         }
//     }
//     input[beginIdx..lastIdx+1].to_string()
// }
fn trim_me(input: &str) -> String {
    let mut begin_idx = 0;
    let mut last_idx = input.len();

    for (index, c) in input.char_indices() {
        if !c.is_whitespace() {
            last_idx = index;
            break;
        }
    }

    for (index, c) in input.char_indices().rev() {
        if !c.is_whitespace() {
            begin_idx = index;
            break;
        }
    }

    input[begin_idx..=last_idx].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string()+" world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    
    for (index,c) in input.char_indices(){
        if c!=' '{
            if triggerOnce==false{
            beginIdx=index;
            triggerOnce=true;
            }
            lastIdx=index;
        }
    }
    vec.into_iter().collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
