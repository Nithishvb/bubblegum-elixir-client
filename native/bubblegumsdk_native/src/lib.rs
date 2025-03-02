#[rustler::nif]
fn add() -> String {
    let val: String = String::from("Hello world from elixir");
    return val;
}

rustler::init!("Elixir.BubblegumElixirClient");
