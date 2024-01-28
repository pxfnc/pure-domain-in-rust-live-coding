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

// -----------------------------------------------------------------------------

fn create_user_usecase_chotto_god(db: &mut DB, name: String) -> Result<(), &'static str> {
    let new_id = db.keys().max().map(|n| n + 1).unwrap_or_default();

    let user = User::new(new_id, name)?;
    let mut record = HashMap::new();
    record.insert("id", user.id.to_string());
    record.insert("name", user.name);
    db.insert(new_id, record);
    Ok(())
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct UserRepo {
    db: DB,
}

impl UserRepo {
    fn new(db: DB) -> Self {
        Self { db }
    }
    fn get_new_id(&self) -> u64 {
        self.db.keys().max().map(|n| n + 1).unwrap_or_default()
    }
    fn save(&mut self, user: User) -> Result<(), &'static str> {
        let new_id = self.db.keys().max().map(|n| n + 1).unwrap_or_default();
        let mut record = HashMap::new();
        record.insert("id", user.id.to_string());
        record.insert("name", user.name);
        self.db.insert(new_id, record);
        Ok(())
    }
}

fn create_user_usecase_sugoku_god(repo: &mut UserRepo, name: String) -> Result<(), &'static str> {
    let new_id = repo.get_new_id();
    let user = User::new(new_id, name)?;
    repo.save(user)
}

// -----------------------------------------------------------------------------

trait IUserRepo {
    fn get_new_id(&self) -> u64;

    fn save(&mut self, user: User) -> Result<(), &'static str>;
}

impl IUserRepo for UserRepo {
    fn get_new_id(&self) -> u64 {
        self.get_new_id()
    }

    fn save(&mut self, user: User) -> Result<(), &'static str> {
        self.save(user)
    }
}

fn create_user_usecase_awesome_saiko<R>(repo: &mut R, name: String) -> Result<(), &'static str>
where
    R: IUserRepo,
{
    let new_id = repo.get_new_id();
    let user = User::new(new_id, name)?;
    repo.save(user)
}

// -----------------------------------------------------------------------------

fn main() {
    let db: DB = HashMap::new();
    let mut repo: UserRepo = UserRepo::new(db);

    let _ = create_user_usecase_awesome_saiko(&mut repo, "Taro".to_string());
    let _ = create_user_usecase_awesome_saiko(&mut repo, "Jiro".to_string());
    println!("{:?}", repo);
}
