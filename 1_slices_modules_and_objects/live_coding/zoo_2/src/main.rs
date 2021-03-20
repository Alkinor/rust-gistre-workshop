mod animal;
mod wrangler;

// No specific structure here as long as they use every function and have ZERO warning
fn main() {
    println!("Remaining Ape population: {}", animal::Ape::population());

    let mut arthur = animal::Ape::with_bananas(15);
    let an_ape = animal::Ape::new();

    an_ape.scream();

    arthur.eat_banana();
    arthur.scream();

    while arthur.is_happy() {
        arthur.eat_banana();
        arthur.scream();
    }

    let skunky = animal::Skunk;
    skunky.stink();

    let mut gistred = animal::Gistre::sad();

    gistred.attend_class(animal::gistre::Class::RGW);
    gistred.attend_class(animal::gistre::Class::DRIL);
    gistred.attend_class(animal::gistre::Class::BSP);
    gistred.attend_class(animal::gistre::Class::MAUTO);
}
