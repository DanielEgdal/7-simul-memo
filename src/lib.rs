type Clock = [i8; 14];

fn actual_dial_postion(dial:i8)-> i8{
    ((dial%12)+12) %12
}

fn make_move(clockwheel: i8, amount: i8, direction: i8) -> i8 {
    let tmp_clock =  clockwheel + (amount * direction);
    actual_dial_postion(tmp_clock)
    // clockwheel + (amount * direction)
}

fn move_num_to_clock_idx(move_num:usize) -> (Vec<usize>,i8){
    let idxs = match move_num {
        0 => (vec![1,4,5,6],1), // UR
        1 => (vec![3,4,6,8],1), // DR
        2 => (vec![2,4,7,8],1), // DL
        3 => (vec![0,4,5,7],1), // UL
        4 => (vec![0,1,4,5,6,7],1), // U
        5 => (vec![1,3,4,5,6,8],1), // R
        6 => (vec![2,3,4,6,7,8],1), // D
        7 => (vec![0,2,4,5,7,8],1), // L
        8 => (vec![0,1,2,3,4,5,6,7,8],1), // ALL
        9 => (vec![],1), // y2
        10 => (vec![0,1,9,10,11,12],-1), // U
        11 => (vec![0,2,9,10,11,13],-1), // R
        12 => (vec![2,3,9,11,12,13],-1), // D
        13 => (vec![1,3,9,10,12,13],-1), // L
        14 => (vec![0,1,2,3,9,10,11,12,13],-1), // ALL
        15 => (vec![],-1), // pins
        16 => (vec![],-1), // pins
        17 => (vec![],-1), // pins
        18 => (vec![],-1), // pins
        _ => unreachable!(),
    };
    idxs
}

fn get_direction(movee: char) -> i8{
    let direction = match movee {
        '+' => 1,
        '-' => -1,
        '2' => 0, // y2, will anyway be ignored
        _ => unreachable!(),
    };
    direction
}

fn apply_scramble(scr:String) -> Clock{
    let mut clock: Clock = [0 as i8;14];
    let split_scr = scr.split_whitespace().enumerate();

    for (idx,movee) in split_scr{
        if idx == 9 || idx >= 15 {
            continue;
        }
        let amount_str = &movee[movee.len()-2..movee.len()-1];
        let amount = amount_str.parse::<i8>().expect("failed to parse amount");

        let direction_temp = movee.chars().last().expect("string should not be empty");
        let direction = get_direction(direction_temp);

        let (clock_idxs, backside)= move_num_to_clock_idx(idx);
        
        for clock_idx in clock_idxs{
            let mut tmp_direction = direction;
            if clock_idx < 4{
                tmp_direction *= backside;
            }
            clock[clock_idx] = make_move(clock[clock_idx],amount,tmp_direction);
        }
    }

    clock
}

fn corner_back_front_map(dial: i8) -> i8{
    let res = dial - 6;
    (6 - res) % 12
}

fn clock_modulo(x: i8) -> i8 {
    let rem = ((x % 12) + 12) % 12;
    if rem > 6 {
        rem - 12
    } else if rem < -5 {
        rem + 12
    } else {
        rem
    }
}

fn move1(clock:Clock) -> i8{
    clock_modulo(clock[9]-clock[13])
}

fn move2(clock:Clock) -> i8{
    let p1 = clock_modulo(corner_back_front_map(clock[2])-clock[11]);
    let p2 = clock_modulo(clock[8]-clock[6]);
    clock_modulo(p2+p1)
}

fn move3(clock:Clock) -> i8{
    clock_modulo(clock[13]-clock[11])
}

fn move4(clock:Clock) -> i8{
    clock_modulo(clock[6]-clock[8])
}

fn move5(clock:Clock) -> i8{
    let p1 = actual_dial_postion(clock_modulo(clock[8]-clock[4])+clock[5]);
    let p2 = clock_modulo(corner_back_front_map(clock[1])-clock[12]);
    let p3 = clock_modulo(corner_back_front_map(clock[2])-clock[11]);
    clock_modulo(clock_modulo(p1+p2+p3)*-1)
}

fn move6(clock:Clock) -> i8{
    let p1 = actual_dial_postion(clock_modulo(clock[10]-clock[9])+clock[13]);
    let p2 = clock_modulo(clock[3]-clock[6]);
    let p3 = clock_modulo(clock[0]-clock[7]);
    clock_modulo(p1+p2+p3)
}

pub fn run(scr:String) -> Vec<i8>{
    let clock = apply_scramble(scr);
    // println!("{:?}",clock);
    // println!("{} {}",move1(clock),move2(clock));
    // println!("{} {}",move3(clock),move4(clock));
    // println!("{} {}",move5(clock),move6(clock));
    vec!(move1(clock),move2(clock),move3(clock),move4(clock),move5(clock),move6(clock))

}
