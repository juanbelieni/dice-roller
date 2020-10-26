mod dice;

use dice::Dice;
use std::io::stdin;

fn parse_dice(dice_string: &String) -> Result<Vec<Dice>, String> {
    let mut pos = 0;
    let mut dice: Vec<Dice> = vec![];

    while pos < dice_string.len() {
        let op = match dice_string.get(pos..pos + 1).unwrap() {
            "+" | "-" => {
                let op = dice_string.get(pos..pos + 1).unwrap().to_string();
                pos += 1;
                op
            }
            _ => "+".to_string(),
        };

        let next_op_pos = match dice_string
            .get(pos..dice_string.len())
            .unwrap()
            .chars()
            .position(|c| c == '+' || c == '-')
        {
            Some(next_op_pos) => pos + next_op_pos,
            None => dice_string.len(),
        };

        let next_dice_string = dice_string.get(pos..next_op_pos).unwrap();

        if !next_dice_string.contains('d') {
            return Err(format!("`d` not found in `{}`", next_dice_string));
        }

        let next_dice_data = next_dice_string
            .trim()
            .split('d')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<u64>().unwrap_or(0))
            .collect::<Vec<u64>>();

        if next_dice_data[0] == 0 && next_dice_data[0] == 1 {
            return Err(format!(
                "non nullable integers not found before and after `d`"
            ));
        } else if next_dice_data[1] == 0 {
            return Err(format!(
                "non nullable integer not found after `{}d`",
                next_dice_data[0]
            ));
        }

        let mult: u64 = match next_dice_data[0] {
            0 => 1,
            mult => mult,
        };
        let sides: u64 = next_dice_data[1];

        dice.push(Dice { op, mult, sides });

        pos = next_op_pos;
    }

    Ok(dice)
}

fn main() {
    let mut input = "".to_string();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");

    input = input.trim().to_string();

    let all_dice = match parse_dice(&input) {
        Ok(all_dice) => all_dice,
        Err(err) => panic!("{}", err),
    };

    let roll = all_dice.iter().fold(0, |mut sum: i128, dice| {
        sum += dice.roll();
        sum
    });

    println!("Roll: {}", roll);
}
