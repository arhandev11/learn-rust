use std::collections::HashSet;


fn longest_palindrome(s: String) -> i32 {
    let mut count = 0;
    let mut set: HashSet<char> = HashSet::new();
    for c in s.chars() {
        if set.contains(&c){
            count += 2;
            set.remove(&c);
        }else{
            set.insert(c);
        }
    }

    if set.len() > 0 {
        count+=1;
    }
    println!("{}", count);
    count
}

fn hello_world(){
    println!("Hallo Dunia");
}

fn main() {
    longest_palindrome("abccccdd".to_string());
    hello_world();
}
