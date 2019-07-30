
#![allow(unused_variables)]
fn main() {
    pub trait Summary {
        fn summarize(&self) -> String;

        fn summarize_author(&self) -> String{
            String::from("(Read more...)")
        }

    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    //simple way of expression
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    //trait bounds way of expression
    pub fn notify2<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    // if having two traits in function
    pub fn notify3(item1: impl Summary, item2: impl Summary) {
        println!("Breaking news! {}", item1.summarize());
    }
    // using two traits on same function.
    //pub fn notify(item: impl Summary + Display) {}
    //or
    //pub fn notify<T: Summary + Display>(item: T){}


    /*
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
    //using where class for easy readability
    fn some_function<T,U>(t:T, u:U) ->i32
           where T: Display + Clone,
                 U: Clone + Debug
    {
    ....
    }
    */

    //Returning Types that Implement Traits
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    // cannot return two differnet types for the return traits
    //Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around
    //how the impl Trait syntax is implemented in the compiler.
    /*
    fn returns_summarizable(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from("The Pittsburgh Penguins once again are the best
                hockey team in the NHL."),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }
    */

}
