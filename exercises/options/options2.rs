// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // nested some - pop returns an option too 
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop
        while let Some(Some(element)) = optional_integers.pop() {
            assert_eq!(element, cursor);
            cursor -= 1;
        }
        // another solution that checks for none
        // while let Some(element) = optional_integers.pop() {
        //     if let Some(i) = element {
        //         assert_eq!(i, cursor);
        //         cursor -= 1;
        //     } else if let None = element {
        //         assert_eq!(None, element);
        //     }
        // }

        assert_eq!(cursor, 0);
    }
}
