struct Contact {
    name: String,
    email: String,
    phone: String,
}

impl Contact {
    // Method to print contact details
    fn print_contact(&self) {
        println!("Name: {}", self.name);
        println!("Email: {}", self.email);
        println!("Phone: {}", self.phone);
    }

    // Method to update email
    fn update_email(&mut self, email: String) {
        self.email = email; // Direct assignment
    }

    // Method to update phone number
    fn update_phone(&mut self, phone: String) {
        self.phone = phone; // Direct assignment
    }

    // Method to check if another contact has the same name
    fn has_same_name(&self, other: &Contact) -> bool {
        self.name == other.name
    }
}

fn main() {
    let mut cont1 = Contact {
        name: String::from("John Doe"),
        email: String::from("MyCool@email.com"),
        phone: String::from("123456789"),
    };

    let cont2 = Contact {
        name: String::from("John Doe"),
        email: String::from("AnotherCool@email.com"),
        phone: String::from("8654787556"),
    };

    cont1.print_contact();
    cont1.update_email(String::from("UpdatingMy@email.com"));
    cont1.update_phone(String::from("98765485"));
    cont1.print_contact();

    // Check if two contacts have the same name
    if cont1.has_same_name(&cont2) {
        println!("Uh oh! Contact with the name '{}' already exists.", cont1.name);
    } else {
        println!("No contact with the same name found.");
    }
}
