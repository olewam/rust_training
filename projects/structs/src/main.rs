fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{0}", user2.username);
    
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* Unit type structs (No fields)
     - Useful when you need to implement a trait on 
        some type but don't have any data that you 
        want to store in the type itself 
*/
struct AlwaysEqual;


