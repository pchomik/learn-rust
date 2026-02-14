fn main() {
    let cat: (&str, f64) = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let (name, age): (&str, f64) = cat;

    println!("{name} is {age} years old");
}
