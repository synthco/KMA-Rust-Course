pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}")
    if list.is_empty(){
        return String::new();
    }

    let mut lines = Vec::new();

    for pair in list.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        lines.push(format!("For want of a {a} the {b} was lost."));
    }
     lines.push(format!("And all for the want of a {}.", list[0]));
    lines.join("\n")

}

