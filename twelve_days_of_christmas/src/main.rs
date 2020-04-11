fn main() {
    let verses = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a layin",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve Lords a leaping",
    ];
    for i in 1..12 {
        println!("On the first day of Christmas my true love sent to me");

        for j in (0..i).rev() {
            if j > 0 {
                print!("{}, ", verses[j]);
            } else {
                println!("{}", verses[j]);
            }
        }
        println!("A partridge in a pear tree\n");
    }
}
