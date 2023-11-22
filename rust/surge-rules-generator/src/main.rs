fn main() {
    let gfw = "https://raw.githubusercontent.com/gfwlist/gfwlist/master/gfwlist.txt";
    let resp = reqwest::get(gfw)
        .await?
        .text()
        .await?;
    println!("body = {:?}", resp);
}
