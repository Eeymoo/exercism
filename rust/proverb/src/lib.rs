pub fn build_proverb(list: &[&str]) -> String {
    let mut i =  0;
    let mut r = String::new();
    if list.len() == 0 {
        return r;
    }
    while i < list.len() {
        if i == list.len()-1 {
            break;
        }
        r.push_str(
            format!("For want of a {} the {} was lost.\n", list[i], list[i+1]).as_str()
        );
        i += 1;
    }
    r.push_str(format!("And all for the want of a {}.", list[0]).as_str());

    r
}
