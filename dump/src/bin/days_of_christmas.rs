
fn main() {
    const GIFTS: [&str; 12] = [
        "And a partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    
    const NUMBERS: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love send to me", NUMBERS[day]);
        if day == 0 {
            println!("A Partridge in a pear tree");
        }
        else {
            for a in (0..=day).rev() {
                println!("{}", GIFTS[a]);
            }    
        }
        
        println!("");
    }
    
    println!("{}", GIFTS[0]);
}
