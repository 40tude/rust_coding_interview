type Link = Option<Box<ListNode>>;

struct ListNode {
    val: i32,
    next: Link,
}

impl ListNode {
    fn new(val: i32, next: Link) -> ListNode {
        ListNode { val, next }
    }
}

fn main() {
    let vals = vec![1, 2, 3, 4, 5];
    let mut head = None;

    for v in vals.into_iter().rev() {
        head = Some(Box::new(ListNode::new(v, head)));
    }

    head = remove_kth_last_node(head, 2);

    let mut current = head.as_ref();
    while let Some(node) = current {
        println!("{}", node.val);
        current = node.next.as_ref();
    }
}

fn remove_kth_last_node(head: Link, k: usize) -> Link {
    // Move the leader k times
    let mut leader = head.as_ref();
    for _ in 0..k {
        if let Some(node) = leader {
            leader = node.next.as_ref();
        } else {
            return head; // k trop grand
        }
    }

    let mut trailer = head.as_ref();

    // Move leader and trailer forward until leader reach the end
    while let Some(node) = leader {
        leader = node.next.as_ref();
        trailer = trailer.unwrap().next.as_ref();
    }

    head
}

// fn remove_kth_last_node(mut head: Link, k: usize) -> Link {
//     let mut leader = &head;
//     for _ in 0..k {
//         if let Some(leader_node) = leader {
//             leader = &leader_node.next;
//         } else {
//             return head; // k is too big
//         }
//     }

//     let mut trailer = &head;
//     while let Some(leader_node) = leader {
//         leader = &leader_node.next;
//         if let Some(trailer_node) = trailer {
//             trailer = &trailer_node.next;
//         }
//     }

//     // let mut trailer_ptr: *mut Link = &mut head; // raw mutable pointer

//     // let mut leader = leader;
//     // while let Some(node) = leader {
//     //     leader = &node.next;

//     //     // SAFETY: trailer_ptr always points to a valid Link in the list
//     //     unsafe {
//     //         trailer_ptr = &mut (*trailer_ptr).as_mut().unwrap().next;
//     //     }
//     // }

//     // // SAFETY: trailer_ptr points to the node before the one we want to remove
//     // unsafe {
//     //     if let Some(node) = trailer_ptr.as_mut().unwrap() {
//     //         *trailer_ptr = node.next.take();
//     //     }
//     // }

//     head
// }

// fn remove_kth_last_node(mut head: Link, k: usize) -> Link {
//     let mut leader = &head;
//     for _ in 0..k {
//         if let Some(node) = leader {
//             leader = &node.next;
//         } else {
//             return head; // k is too big
//         }
//     }

//     let mut trailer_ptr: *mut Link = &mut head; // raw mutable pointer

//     let mut leader = leader;
//     while let Some(node) = leader {
//         leader = &node.next;

//         // SAFETY: trailer_ptr always points to a valid Link in the list
//         unsafe {
//             trailer_ptr = &mut (*trailer_ptr).as_mut().unwrap().next;
//         }
//     }

//     // SAFETY: trailer_ptr points to the node before the one we want to remove
//     unsafe {
//         if let Some(node) = trailer_ptr.as_mut().unwrap() {
//             *trailer_ptr = node.next.take();
//         }
//     }

//     head
// }
