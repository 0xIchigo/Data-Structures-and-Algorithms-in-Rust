// 99 Bottles of Beer is a traditional reverse counting song in Canada and the United States. It's
// usually sung on road trips, family outings, school trips, or Scout / Girl Guide
// outings, due to its repetitive, easy to memorize format

// The song is also popular in programming circles as Tim Robinson maintained a website with
// the song coded in various programming languages. The idea was initiated by a post to a humor mailing
// list, where someone sent the full lyrics in 1994. https://www.99-bottles-of-beer.net/ is a continuation 
// of Tim's early site featuring over 1500 different programming languages and variations

trait Bottles {
    fn bottles_of_beer(&self) -> Self;
    fn on_the_wall(&self);
}

impl Bottles for u32 {
    fn bottles_of_beer(&self) -> u32 {
        match *self {
            0 => print!("No bottles of beer"),
            1 => print!("{} bottle of beer", self),
            _ => print!("{} bottles of beer", self)
        }
        *self
    }

    fn on_the_wall(&self) {
        println!(" on the wall!");
    }
}

fn main () {
    for i in (1..100).rev() {
        i.bottles_of_beer().on_the_wall();
        i.bottles_of_beer();
        println!("\nTake one down, pass it around...");
		(i - 1).bottles_of_beer().on_the_wall();
		println!("-----------------------------------");
    }
}