/*
 * Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
 * taking advantage of the repetition in the song.
 */

use std::collections::HashMap;
fn main() {
    
    let number_positions: HashMap<i32, &str> = [
        (1, "first"), 
        (2, "second"), 
        (3, "third"),
        (4, "fourth"),
        (5, "fifth"),
        (6, "sixth"),
        (7, "seventh"),
        (8, "eighth"),
        (9, "ninth"),
        (10, "tenth"),
        (11, "eleventh"),
        (12, "twelfth")
    ].iter().cloned().collect();
    
    let number_song_line: HashMap<i32, &str> = [
        (1, "And a partridge in a pear tree."), 
        (2, "Two turtle doves,"), 
        (3, "Three French hens,"),
        (4, "Four calling birds"),
        (5, "Five gold rings"),
        (6, "Six geese a-laying"),
        (7, "Seven swans a-swimming"),
        (8, "Eight maids a-milking"),
        (9, "Nine ladies dancing"),
        (10, "Ten lords a-leaping"),
        (11, "Eleven pipers piping"),
        (12, "Twelve drummers drumming")
    ].iter().cloned().collect();

    for number in 1..13 {

        println!("");
        
        println!("On the {} day of Christmas\nMy true love sent to me:",
                 number_positions[&number]);

        if number == 1 {
            println!("A partridge in a pear tree.");
            continue;
        } 
        
        let song_line_limit = number + 1;
        
        for song_line_number in (1..song_line_limit).rev() {
            println!("{}", number_song_line[&song_line_number]);
        }

        println!("");
    }
}
