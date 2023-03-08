
fn main(){
    test1();

    let newArticle = NewArticle {
        headline: String::from("today top news"),
        location: String::from("shanghai"),
        author: String::from("Spark"),
        content: String::from("happy new year"),
    };

    let tweet = Tweet{
        username: String::from("Free"),
        content: String::from("come back to old home"),
        reply: true,
        retweet: true,
    };

    println!("newArticle summarize: {}", newArticle.summarize());
    println!("tweet summarize: {}", tweet.summarize());
}