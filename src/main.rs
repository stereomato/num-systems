



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

fn binary_to_decimal(input: String) -> f64 {
    // can use len() due to just handling simple text
    let input_divided: Vec<&str> = input.split_terminator('.').collect();
    let mut counter: isize = input_divided[0].to_string().len() as isize;
    //let mut counter = 0;
    let mut binary_number = 0.0;
    for character in input.as_str().chars(){
        
        if character == '.' {
            continue;
        } else {
            counter -= 1;
            binary_number += character.to_string().parse::<f64>().unwrap() * 2_f64.powi(counter as i32);
        }
    }
    return binary_number;
}



fn main() {
    let  yo = "1010.10";

    println!("{}", '.' as u32);
    println!("{}", yo);
    println!("{}", binary_to_decimal(yo.to_string()));
    // have to check over here if the given input is proper
    if check_input(yo.to_string(),16) {
        println!("wowee");
    } else {
        println!("fuck");
    }
    
}