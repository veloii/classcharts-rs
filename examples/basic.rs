use classcharts::Client;
use std::io::{stdin, stdout, Write};

// doesn't really matter to this library, just generic code
fn grab_input(text: &'static str) -> String {
    let mut s = String::new();

    print!("{} ", text);

    let _ = stdout().flush();

    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return s;
}

#[tokio::main]
async fn main() {
    let code = grab_input("What is your classcharts code?");
    let dob = grab_input("What is your date of birth (DD/MM/YYYY)?");

    // logging in and creating the client
    let client = Client::create(code, dob, None).await;

    match client {
        Ok(mut client) => {
            // grabbing the student info
            let student = client.get_student_info().await.unwrap();
            
            println!("Hello, {}!", student.data.user.first_name);

            // checking if user has the homework feature
            if student.data.user.display_homework {
                // grabbing the user's homework, dates are default, hense why None
                let homework = client.get_homeworks(None).await.unwrap();

                // this is from the classcharts meta response, (not generated by the library)
                let completed = homework.meta.this_week_completed_count;

                println!(
                    "You have {} completed homework{} this week.",
                    completed,
                    if completed == 1 { "" } else { "s" }
                );
            } else {
                println!("Sadly you do not have access to the homework feature but here's a cat 🐱");
            }
        }
        Err(err) => {
            // this will probably occur if the user supplies an incorrect access code or cc is down
            println!("{:?}: {}", err, err);
        }
    }
}
