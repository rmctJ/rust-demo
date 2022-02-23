use chrono::Local;

fn main() {
    let local = Local::now();

    println!("Hello! time:{}", local.to_rfc3339());
}
