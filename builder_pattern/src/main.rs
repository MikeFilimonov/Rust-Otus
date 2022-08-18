#[derive(Debug, PartialEq)]
struct Potus {
    name: String,
    surname: String,
    phone: u32,
    email: Option<String>,
    instagram: Option<String>,
}

#[derive(Debug, PartialEq)]
struct PotusBuilder {
    name: String,
    surname: String,
    phone: u32,
    email: Option<String>,
    instagram: Option<String>,
}

impl PotusBuilder {
    fn new(name: impl Into<String>, surname: impl Into<String>, phone: u32) -> Self {
        Self {
            name: name.into(),
            surname: surname.into(),
            phone,
            email: None,
            instagram: None,
        }
    }

    fn with_instagram(mut self, instagram: impl Into<String>) -> Self {
        self.instagram = Some(instagram.into());
        self
    }

    fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

}

impl Potus {
        fn builder(name: impl Into<String>, surname: impl Into<String>, phone: u32) -> PotusBuilder {
            PotusBuilder::new(name, surname, phone)
        }
    }

fn main() {
    let _fourtyfifth = PotusBuilder::new("Dolan", "Tumpr", 18007895);
    let _yaoldie = Potus::builder("Joseph", "Biden", 18002022)
        .with_email("potus@potus.com")
        .with_instagram("@potus");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn build_a_potus() {
        let original = Potus::builder("Elon", "Musk", 1991005030);
        let clone: PotusBuilder = PotusBuilder::new("Elon", "Musk", 1991005030);
        assert_eq!(original, clone);
    }
}
