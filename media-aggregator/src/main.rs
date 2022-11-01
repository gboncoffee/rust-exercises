use media_aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("gg"),
        content:  String::from("UwU"),
        reply:    false,
        retweet:  false,
    };

    println!("New tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Cyberpunk, weebs and femboys unite against the corporations"),
        location: String::from("Cyberspace"),
        author:   String::from("Gabriel G. de Brito"),
        content:  String::from("Cyberpunk, weebs and femboys made an open coalizion against the \
                               dominance of the corporations on the Cyberspace."),
    };

    println!("New article: {}", article.summarize());
}
