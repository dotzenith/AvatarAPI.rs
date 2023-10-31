mod database;

fn main() {
    let db = database::Database::new("quotes.db").unwrap();
    let res = db.random(2).unwrap();
    println!("{:?}", res);
}
