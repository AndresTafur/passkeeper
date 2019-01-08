use entity::Credential;

#[derive(Clone)]
pub struct PassKeeperService {
    credentials: Vec<Credential>,
    master_password: Option<String>,
}

impl PassKeeperService {
    pub fn new() -> Self {
        Self {
            credentials: Vec::new(),
            master_password: None,
        }
    }

    pub fn get_unlocked_credentials(&self, id: &String) -> Result<Option<Credential>, String> {
        return match self.master_password.clone() {
            Some(pass) => Ok(self
                .credentials
                .iter()
                .find(|x| x.get_id() == id)
                .map(|x| x.unlock(&pass))),
            None => Err("Master password must not be empty".to_string()),
        };
    }

    pub fn get_locked_credentials(&self, id: &String) -> Option<&Credential> {
        self.credentials.iter().find(|x| x.get_id() == id)
    }

    pub fn add(&mut self, credential: &Credential) {
        let cypher_cred = credential.lock(&self.master_password.clone().unwrap());
        self.credentials.push(cypher_cred);
    }

    pub fn set_password(&mut self, password: String) {
        self.master_password = Some(password);
    }
}

#[cfg(test)]
mod tests {

    use entity::Credential;
    use service::PassKeeperService;

    #[test]
    fn lock_test() {
        let mut pass_keeper = PassKeeperService::new();
        let credential = Credential::new(
            "test_id".to_string(),
            "test_user".to_string(),
            "test_password".to_string(),
        );

        pass_keeper.set_password("test_p4ssw0rd".to_string());
        pass_keeper.add(&credential);

        let cred = pass_keeper
            .get_unlocked_credentials(&"test_id".to_string())
            .unwrap_or_else(|_| panic!("Error"))
            .unwrap_or_else(|| panic!("Error"));

        assert_eq!(cred.get_id(), "test_id");
        assert_eq!(cred.get_password(), "test_password");
        assert_eq!(cred.get_user(), "test_user");
    }

    #[test]
    fn unlock_test() {
        let mut pass_keeper = PassKeeperService::new();
        let credential = Credential::new(
            "test_id".to_string(),
            "test_user".to_string(),
            "test_password".to_string(),
        );

        pass_keeper.set_password("test_p4ssw0rd".to_string());
        pass_keeper.add(&credential);

        let result = pass_keeper.get_locked_credentials(&"test_id".to_string());
        let cred = result.expect("Error");

        assert_eq!(cred.get_id(), "test_id");
        assert_eq!(cred.get_password(), "c29tZXNhbHTpdePC+3xTv9JiIjQNueUG");
        assert_eq!(cred.get_user(), "test_user");
    }

}
