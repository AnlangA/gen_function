use std::collections::LinkedList;


/// 在链表main_list中查找是否包含链表sub_list，如果包含则返回最后一个匹配的节点，否则返回None
pub fn find_sublist_last_node<T: PartialEq + Clone>(
    main_list: &LinkedList<T>,
    sub_list: &LinkedList<T>,
 ) -> Option<T> {
    if sub_list.is_empty() {
        return None; // 空链表不返回任何节点
    }
    let mut main_iter = main_list.iter();
    let mut sub_iter = sub_list.iter();
    // 获取子链表的第一个元素
    let first_sub_elem = sub_iter.next().unwrap();
    // 遍历主链表，寻找与子链表第一个元素匹配的元素
    while let Some(main_elem) = main_iter.next() {
        if main_elem == first_sub_elem {
            // 找到匹配的元素，开始比较后续元素
            let mut main_iter_clone = main_iter.clone();
            let mut sub_iter_clone = sub_iter.clone();
            let mut last_matched = main_elem.clone();
            let mut is_sublist = true;
            while let Some(sub_elem) = sub_iter_clone.next() {
                if let Some(main_elem) = main_iter_clone.next() {
                    if main_elem != sub_elem {
                        is_sublist = false;
                        break;
                    }
                    last_matched = main_elem.clone();
                } else {
                    return None; // 主链表遍历完，但子链表还有元素
                }
            }
            if is_sublist {
                return Some(last_matched); // 返回最后一个匹配的节点
            }
        }
    }
    None // 未找到匹配的子链表
 }