fn main() {
    println!("TWELVE DAYS OF CHRISTMAS:");
    println!();
    let gifts: [&str;12] = ["a partridge in a pear tree","two turtle doves","three French hens","four calling birds","five gold rings","six geese a-laying",
                            "seven swans a-swimming","eight maids a-milking","nine ladies dancing","ten lords a-leaping","eleven pipers piping","twelve drummers drumming"];
    let days: [&str;12] = ["first","second","third","fourth","fifth","sixh","seventh","eigth","nineth","tenth","eleventh","twelveth"];

    for day in 0..days.len() {
        println!("On the {} day of christmas\nmy true love gave to me", days[day]);

        if day == 0 {
            println!("{}",gifts[day]);
            println!();
            continue;
        }
        else {
            for gift in (0..(day+1)).rev() {

                if gift > 0 {
                    println!("{}",gifts[gift]);
                    continue;
                }
                println!("and {}",gifts[0]);
            }
        }
        println!();
    }
}
