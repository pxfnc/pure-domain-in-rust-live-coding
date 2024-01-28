use std::collections::HashMap;

// #[derive(Debug)]
// struct User {
//     id: u64,
//     name: String
// }

type DB = HashMap<u64, HashMap<&'static str, String>>;

fn create_user_usecase_bad(db: &mut DB, name: String) -> Result<(), &'static str> {
    let new_id = db.keys().max().map(|n| n + 1).unwrap_or_default();
    let mut user = HashMap::new();

    if name.len() > 6 {
        return Err("5バイト以下");
    }

    user.insert("id", new_id.to_string());
    user.insert("name", name);
    db.insert(new_id, user);
    Ok(())
}

fn main() {
    let mut db: DB = HashMap::new();
    let _ = create_user_usecase_bad(&mut db, "Taro".to_string());
    let _ = create_user_usecase_bad(&mut db, "Jiro".to_string());
    println!("{:?}", db);
}
