use std::collections::HashMap;

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

impl User {
    pub fn new(id: u64, name: String) -> Result<Self, &'static str> {
        if name.len() > 6 {
            Err("5バイト以下")
        } else {
            Ok(Self { id, name })
        }
    }
}

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

fn create_user_usecase_chotto_god(db: &mut DB, name: String) -> Result<(), &'static str> {
    let new_id = db.keys().max().map(|n| n + 1).unwrap_or_default();

    let user = User::new(new_id, name)?;
    let mut record = HashMap::new();
    record.insert("id", user.id.to_string());
    record.insert("name", user.name);
    db.insert(new_id, record);
    Ok(())
}

fn main() {
    let mut db: DB = HashMap::new();
    let _ = create_user_usecase_chotto_god(&mut db, "Taro".to_string());
    let _ = create_user_usecase_chotto_god(&mut db, "Jiro".to_string());
    println!("{:?}", db);
}
