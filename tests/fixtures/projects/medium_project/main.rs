struct User {
    id: u32,
    username: String,
    email: String,
}

impl User {
    fn new(id: u32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
        }
    }

    fn display(&self) {
        println!("User {}: {} <{}>", self.id, self.username, self.email);
    }
}

struct Team {
    name: String,
    members: Vec<User>,
}

impl Team {
    fn new(name: String) -> Self {
        Team {
            name,
            members: Vec::new(),
        }
    }

    fn add_member(&mut self, user: User) {
        self.members.push(user);
    }

    fn list_members(&self) {
        println!("Team: {}", self.name);
        for member in &self.members {
            member.display();
        }
    }
}

fn main() {
    let mut team = Team::new("Engineering".to_string());

    let user1 = User::new(1, "alice".to_string(), "alice@example.com".to_string());
    let user2 = User::new(2, "bob".to_string(), "bob@example.com".to_string());
    let user3 = User::new(3, "charlie".to_string(), "charlie@example.com".to_string());

    team.add_member(user1);
    team.add_member(user2);
    team.add_member(user3);

    team.list_members();
}
