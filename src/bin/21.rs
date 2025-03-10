use petgraph::Direction;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
struct Step {
    pos: (usize, usize),
    direction: ,
}
fn shortest_routes(start_pos: usize, end_pos: usize, grid: Vec<Vec<Step>>) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
