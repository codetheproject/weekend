use thestack::{new, string};
use thestack_validator::{Validate as Valid, ValidatorContext};
use validator::Validate;

#[derive(Debug, Validate, new)]
struct User {
    #[validate(length(min = 10, message = "name is too short!"))]
    name: String,

    #[validate(nested)]
    old_user: OldUser,
}

#[derive(Debug, Validate, new)]
struct OldUser {
    #[validate(length(min = 10, message = "name is too short!"))]
    name: String,

    #[validate(range(min = 10, message = "You are too young!"))]
    age: i32,
}

fn main() {
    let user = User::new(string!("West"), OldUser::new(string!("East"), 2));
    let mut context = ValidatorContext::<User, _>::new(Valid);
    context.before_handler(|payload| {
        println!("{:?}", payload);

        Ok(())
    });

    if let Err(err) = context.validate(&user) {
        println!("{:?}", err);
    }
}
