fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "11th", "12th"];
    let day_words = ["Two candy canes", "Three boughs of holly", "Four colored lights", "A shining star", "Little silver bells", "Candles a-glowing", "Gold and silver tinsel", "A guardian angel", "Some mistletoe", "Gifts for one and all", "All their good wishes"];

    for i in 0..12 {
        println!("On the {} day of Christmas", days[i]);
        println!("My good friends brought me");
        for y in (0..i).rev() {
            println!("{}",day_words[y]);
        } 
        println!("And a song for the Christmas tree");
        println!("");
    }
}
