



fn check_input(input: String, radix: u32) -> bool {
    // can't do this comfortably
    let digits_binary = vec![0, 1];
    let digits_octal =  [digits_binary.clone(), vec![2, 3, 4, 5, 6, 7]].concat();
    let digits_hex = [digits_octal.clone(), vec![8, 9, 10, 11, 12, 13, 14, 15]].concat();

    // have to choose a set of digits
    let digits_chosen;
    
    match radix{
        2 => digits_chosen = digits_binary,
        8 => digits_chosen = digits_octal,
        16 => digits_chosen = digits_hex,
        _ => panic!("Attempted to use an improper radix"),
    }

    //println!("{:?}", digits_chosen);

    // iterating over the input, matching each character with the given radix
    for character in input.as_str().chars(){
        match character.to_digit(radix){
            // '.', can't use a nicer way to deal with this
            None => {
                if character == '.'{
                    continue;
                } else {
                    return false;
                }
            },
            thing if !digits_chosen.contains(&thing.unwrap()) => return false,
            Some(_) => (),
            
        }
    }
    // for completeness sake
    return true;
}

// function that converts from any system to decimal
fn to_decimal(input: String, radix: u8) -> String {
    // can use len() due to just handling simple text
    let input_divided: Vec<&str> = input.split_terminator('.').collect();
    let mut counter: isize = input_divided[0].to_string().len() as isize;
    //let mut counter = 0;
    let mut output = 0.0;
    for character in input.as_str().chars(){
        
        if character == '.' {
            continue;
        } else {
            counter -= 1;
            output += match character {
                'A' => 10.0 * (radix as f64).powi(counter as i32),
                'B' => 11.0 * (radix as f64).powi(counter as i32),
                'C' => 12.0 * (radix as f64).powi(counter as i32),
                'D' => 13.0 * (radix as f64).powi(counter as i32),
                'E' => 14.0 * (radix as f64).powi(counter as i32),
                'F' => 15.0 * (radix as f64).powi(counter as i32),
                _ => character.to_string().parse::<f64>().unwrap() * (radix as f64).powi(counter as i32)
            }
        }
    }
    return output.to_string();
}

// converts given input to a num system
// still doesn't support floating
fn to_num_system(input: String, radix: u8) -> String {
    // can use len() due to just handling simple text
    let input_divided: Vec<&str> = input.split_terminator('.').collect();
    //let mut counter = 0;
    let mut output: String = "".to_string();
    let mut quotient = input_divided[0].parse::<i64>().unwrap();
    // integer part
    while quotient > 0 {
        // add handling for radix 16
        output.insert_str(0, (match quotient % (radix as i64) {
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            x=> char::from_digit(x as u32, 10).unwrap()
        }).to_string().as_str() );
        quotient /= radix as i64;
    }
    
    // floating part
    if input_divided.len() > 1 {
        let mut floating_divided: Vec<&str>;
        let mut floating_part = "0.".to_string() + input_divided[1];
        let mut output_floating = "".to_string();
        // convert floating_part to number
        loop {
            floating_part = (floating_part.parse::<f64>().unwrap() * (radix as f64)).to_string();
            floating_divided = floating_part.split_terminator('.').collect();
            output_floating.push_str(floating_divided[0]);
            
            floating_part = {
                if floating_divided[1].parse::<i64>().unwrap() == 0 || output_floating.len() >= 23 {
                    break;
                } else {
                    "0.".to_string() + floating_divided[1]
                }
            };
        }
        output.push_str(".");
        output.push_str(&output_floating.as_str());
    }

    return output;
}

fn system_to_system(input: String, input_radix: u32, output_radix: u32) -> String{
    let mid = to_decimal(input, input_radix as u8);
    return to_num_system(mid, output_radix as u8);
}

fn main() {
    let  yo = "153";

    println!("{}", '.' as u32);
    println!("{}", yo);
    println!("{}", system_to_system(yo.to_string(), 8, 2));
    //println!("{}", to_num_system(yo.to_string(), 8));

    //println!("{}", to_decimal(yo.to_string(), 16));
    // have to check over here if the given input is proper
    if check_input(yo.to_string(),16) {
        println!("wowee");
    } else {
        println!("fuck");
    }
    
}