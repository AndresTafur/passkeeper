use domain::KeeperCipher;

#[derive(Clone)]
pub struct Credential {
    id: String,
    user: String,
    pass: String,
    salt: Option<String>,
    locked: bool,
}

impl Credential {
    pub fn new(id: String, user: String, pass: String) -> Self {
        Self {
            id: id,
            pass,
            user,
            salt: Option::None,
            locked: true,
        }
    }

    pub fn unlock(&self, master_password: &String) -> Self {
        let cipher = KeeperCipher::new(master_password.to_string());

        Self {
            id: self.id.clone(),
            pass: cipher.decrypt(&self.pass).unwrap(),
            user: self.user.clone(),
            salt: Option::None,
            locked: false,
        }
    }

    pub fn lock(&self, master_password: &String) -> Self {
        let cipher = KeeperCipher::new(master_password.to_string());
        let pass = cipher.encrypt(&self.pass).unwrap();

        Self {
            id: self.id.clone(),
            pass,
            user: self.user.clone(),
            salt: self.salt.clone(),
            locked: true,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_user(&self) -> &String {
        &self.user
    }

    pub fn get_password(&self) -> &String {
        &self.pass
    }
}
