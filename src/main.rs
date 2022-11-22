
fn transition(command: char, x: i32, y:i32, direction: char) -> (i32, i32, char) {
    match command {
        'f' => match direction {
            'n' => (x, y+1, direction),
            's' => (x, y-1, direction),
            'e' => (x+1, y, direction),
            'w' => (x-1, y, direction),
            _ => panic!("unknown direction!")

        },
        'l' => match direction {
            'n' => (x, y, 'w'),
            's' => (x, y, 'e'),
            'e' => (x, y, 'n'),
            'w' => (x, y, 's'),
            _ => panic!("unknown direction!")
        },

        _ => unimplemented!()
    }
}

fn sequence(commands: &str, x: i32, y:i32, direction: char) -> (i32, i32, char) {
    let (mut cx, mut cy, mut cd) = (x, y, direction);

    for c in commands.chars() {
        (cx, cy, cd) = transition(c, cx, cy, cd);
    }
    (cx, cy, cd)
}

fn main() {
    println!("Hello, world!");
}

// create test
#[cfg(test)]
mod tests {
    use crate::transition;
    use crate::sequence;

    #[test]
    fn test_transition() {
        let test_x = 1;
        let test_y = 2;
        let test_command = 'f';
        let test_direction = 'e';

        let (r_x, r_y, r_dir) = transition(test_command, test_x, test_y, test_direction);

        assert_eq!(r_x, 2);
        assert_eq!(r_y, 2);
        assert_eq!(r_dir, 'e');
    }

    #[test]
    fn test_turn_left() {
        let test_x = 1;
        let test_y = 2;
        let test_command = 'l';
        let test_direction = 'e';

        let (r_x, r_y, r_dir) = transition(test_command, test_x, test_y, test_direction);

        assert_eq!(r_x, 1);
        assert_eq!(r_y, 2);
        assert_eq!(r_dir, 'n');


        let test_x = 1;
        let test_y = 2;
        let test_command = 'l';
        let test_direction = 'n';

        let (r_x, r_y, r_dir) = transition(test_command, test_x, test_y, test_direction);

        assert_eq!(r_x, 1);
        assert_eq!(r_y, 2);
        assert_eq!(r_dir, 'w');
    }

    #[test]
    fn test_sequence() {
        let commands = "flflf";

        let test_x = 1;
        let test_y = 2;
        let test_direction = 'n';

        let result = sequence(commands, test_x, test_y, test_direction);

        assert_eq!(result.0, 0);
        assert_eq!(result.1, 2);
        assert_eq!(result.2, 's');


    }
}
