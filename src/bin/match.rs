fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter
    };
    fn value_in_cents (coin:Coin) ->i32 {
        match coin {
            Coin::Penny=>1,
            Coin::Nickel=>5,
            Coin::Dime=>10,
            Coin::Quarter=>25,
        }
    };
    let matchingVaue:i32=value_in_cents(Coin::Dime);
println!("matching Value for Dime is {matchingVaue}");
}