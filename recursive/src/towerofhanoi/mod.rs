

pub fn tower_of_hanoi(n: u32) {

    fn tower_of_hanoi_recursive(n: u32, from: char, to: char, aux: char) {
        if n == 1 {
            println!("Move disk 1 from {} to {}", from, to);
            return;
        }
        tower_of_hanoi_recursive(n - 1, from, aux, to);
        println!("Move disk {} from {} to {}", n, from, to);
        tower_of_hanoi_recursive(n - 1, aux, to, from);
    }


    tower_of_hanoi_recursive(n, 'A', 'C', 'B');
}