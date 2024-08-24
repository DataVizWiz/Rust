struct Contact {
    name: String,
    email: Option<String>,
    phone: Option<String>
}

impl Contact {
    fn compare_name(&self, contact: &Contact) -> Result<(), String> {
        if self.name == contact.name {
            Err(format!("Warning: {} already exists", self.name))
        } else {
            Ok(())
        }
    }

    fn print_contact(&self) {
        println!("{}", self.name);
        
        match &self.email {
            None => println!("Email: Not provided"),
            Some(x) => println!("{}", x),
        }

        match &self.phone {
            None => println!("Phone: Not provided"),
            Some(x) => println!("{}", x),
        }
    }

    // Setter method naming convention
    fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    fn set_phone(&mut self, phone: Option<String>) {
        self.phone = phone;
    }

    fn get_summary(&self) {
        let email_summary = self.email
                                        .as_ref()
                                        .map_or("Email: Not provided", String::as_str);
        
        let phone_summary = self.phone
                                        .as_ref()
                                        .map_or("Phone: Not provided", String::as_str);
    
        println!("{}, {}, {}", self.name, email_summary, phone_summary);
    }
}

fn main() {
    let mut contact1 = Contact {
        name: String::from("Mrs Blogs"),
        email: None,
        phone: None
    };

    let mut contact2 = Contact {
        name: String::from("Joe Blogs"),
        email: Some(String::from("JoeBlogs@mail.com")),
        phone: Some(String::from("123456789"))
    };

    match contact1.compare_name(&contact2) {
        Ok(()) => println!("No name conflict."),
        Err(e) => println!("{}", e),
    }

    contact1.print_contact();
    contact2.print_contact();

    contact1.set_email(Some(String::from("MrsBlogs@mail.com")));
    contact2.set_email(None);

    contact1.print_contact();
    contact2.print_contact();

    contact1.set_phone(Some(String::from("97567124323")));
    contact2.set_phone(None);

    contact1.print_contact();
    contact2.print_contact();

    contact1.get_summary();
    contact2.get_summary();
}
