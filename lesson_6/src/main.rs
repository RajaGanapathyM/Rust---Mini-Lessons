use std::collections::{HashMap,HashSet};
use std::fs::{File};
use std::io::{self,BufReader,BufWriter,Write};
use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct contact{
    name:String,
    tag:HashSet<String>,
    phone:String,
}


#[derive(Serialize, Deserialize, Debug)]
struct contactManager{
    contacts:HashMap<String,contact>,
}


impl contactManager {
    fn new() -> contactManager {
        contactManager{
            contacts:HashMap::new(),
        }
    }

    fn add_contact(&mut self,name: &str,phone: &str){
        self.contacts.entry(name.to_string()).or_insert(contact{
            name:name.to_string(),
            phone:phone.to_string(),
            tag:HashSet::new(),
        });
    }

    fn add_tag(&mut self,name: &str,tag: &str){
        if let Some(contact)=self.contacts.get_mut(name){
            contact.tag.insert(tag.to_string());
        } else {
            println!("Contact not found");
        }
    }

    fn remove_tag(&mut self,name: &str,tag: &str){
        if let Some(contact)=self.contacts.get_mut(name){
            contact.tag.remove(tag);
        } else {
            println!("Contact not found");
        }
    }

    fn remove_contact(&mut self,name: &str){
        if self.contacts.contains_key(name){
            self.contacts.remove(name);
        } else {
            println!("Contact not found");
        }
    }

    fn list_contacts(&self){
        for (name,contact) in &self.contacts{
            println!("Name: {}, Phone: {}, Tags: {:?}", name, contact.phone, contact.tag);
        }
    }


    fn save_to_file(&self,filename: &str)-> io::Result<()> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<contactManager> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let manager: contactManager = serde_json::from_reader(reader)?;
        Ok(manager)
    }
}

fn main() {
    let mut manager = match contactManager::load_from_file("./contacts.json") {
        Ok(m) => m,
        Err(_) => contactManager::new(),
    };
    
    println!("\n📞 Welcome to the Rust Contact Manager CLI");
    println!("Available commands: add, remove, tag, untag, list, save, load, exit");

    let stdin=std::io::stdin();

    loop{
        let mut input=String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let tokens : Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.as_slice() {
            ["add", name, phone] => {
                manager.add_contact(name, phone);
                println!("Added contact: {} with phone: {}", name, phone);
            },
            ["remove", name] => {
                manager.remove_contact(name);
                println!("Removed contact: {}", name);
            },
            ["tag", name, tag] => {
                manager.add_tag(name, tag);
                println!("Added tag: {} to contact: {}", tag, name);
            },
            ["untag", name, tag] => {
                manager.remove_tag(name, tag);
                println!("Removed tag: {} from contact: {}", tag, name);
            },
            ["list"] => {
                manager.list_contacts();
            },
            ["save", filename] => {
                if let Err(e) = manager.save_to_file(filename) {
                    eprintln!("Error saving contacts: {}", e);
                } else {
                    println!("Contacts saved to {}", filename);
                }
            },
            ["load", filename] => {
                match contactManager::load_from_file(filename) {
                    Ok(m) => {
                        manager = m;
                        println!("Contacts loaded from {}", filename);
                    },
                    Err(e) => eprintln!("Error loading contacts: {}", e),
                }
            },
            ["exit"] => break,
            _ => println!("Unknown command. Please try again."),
        }
    }

}

