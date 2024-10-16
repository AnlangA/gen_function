use std::collections::LinkedList;
use gen_db::data_analysis::*;

fn main() {
   let mut list1 = LinkedList::new();
   list1.push_back(1);
   list1.push_back(2);
   list1.push_back(3);
   list1.push_back(4);
   let mut list2 = LinkedList::new();
   list2.push_back(2);
   list2.push_back(3);
   match find_sublist_last_node(&list1, &list2) {
       Some(node) => println!("list1 contains list2, last node is {:?}", node),
       None => println!("list1 does not contain list2"),
   }
}
