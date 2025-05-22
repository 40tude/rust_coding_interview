type Link = Option<Box<ListNode>>; // type alias 

struct ListNode {
    val: i32,
    next: Link,
}

impl ListNode {
    fn new(val: i32, next: Link) -> Self {
        Self { val, next }
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

    let mut trailer2 = pointeur sur trailer
    unsafe {
        
        // Remove the kth-last node
        if let Some(node) = &mut *trailer2 {
            *trailer2 = node.next.take();
        }
    }

    // Suppression du nœud
    // if let Some(node) = trailer {
    //     node.next = node.next.take();
    //     node.next = node.next.take();
    // }

    // dummy.unwrap().next
    head
}

// fn remove_kth_last_node(head: Link, k: usize) -> Link {
//     let mut dummy = Box::new(ListNode::new(-1, head));

//     // Phase 1: advance a temp pointer `leader` k steps using a scoped block
//     {
//         let mut leader = &dummy.next;
//         for _ in 0..k {
//             match leader {
//                 Some(node) => leader = &node.next,
//                 None => return dummy.next, // k > length
//             }
//         }

//         // Phase 2: now that leader's borrow is dropped, we can create a mutable trailer
//         let mut trailer = &mut dummy.next;

//         while let Some(node) = leader {
//             leader = &node.next;

//             if let Some(t_node) = trailer {
//                 trailer = &mut t_node.next;
//             }
//         }

//         // Now trailer is before the node to remove
//         if let Some(node) = trailer {
//             *trailer = node.next.take(); // Remove node
//         }
//     }

//     dummy.next
// }

// fn remove_kth_last_node(head: Link, k: usize) -> Link {
//     let mut dummy = Box::new(ListNode::new(-1, head));

//     // Phase 1: avancer k fois sans toucher à dummy
//     let mut temp = dummy.next.as_ref();
//     for _ in 0..k {
//         if let Some(node) = temp {
//             temp = node.next.as_ref();
//         } else {
//             // k trop grand
//             return dummy.next;
//         }
//     }

//     // Phase 2: maintenant que temp n'est plus utilisée, on peut modifier
//     let mut leader = &mut dummy.next;
//     let mut trailer = &mut dummy.next;

//     for _ in 0..k {
//         leader = &mut leader.as_mut().unwrap().next;
//     }

//     while leader.as_ref().is_some() {
//         leader = &mut leader.as_mut().unwrap().next;
//         trailer = &mut trailer.as_mut().unwrap().next;
//     }

//     if let Some(node) = trailer {
//         *trailer = node.next.take();
//     }

//     dummy.next
// }

// fn remove_kth_last_node(head: Link, k: usize) -> Link {
//     let mut dummy = Box::new(ListNode::new(-1, head));
//     let mut trailer = &mut dummy.next;

//     // On utilise un pointeur temporaire NON mutable pour avancer k fois
//     {
//         let mut temp = trailer.as_ref();
//         for _ in 0..k {
//             if let Some(node) = temp {
//                 temp = node.next.as_ref();
//             } else {
//                 // k est trop grand
//                 return dummy.next;
//             }
//         }

//         // Maintenant on avance trailer jusqu'à ce que temp atteigne la fin
//         while let Some(node) = temp {
//             temp = node.next.as_ref();
//             if let Some(t_node) = trailer {
//                 trailer = &mut t_node.next;
//             }
//         }
//     }

//     // trailer est juste avant le nœud à supprimer
//     if let Some(node) = trailer {
//         *trailer = node.next.take();
//     }

//     dummy.next
// }

// fn remove_kth_last_node(head: Link, k: usize) -> Link {
//     let mut dummy = Box::new(ListNode::new(-1, head));

//     // Phase 1: avancer une référence immuable
//     let mut lead = dummy.next.as_ref();
//     for _ in 0..k {
//         match lead {
//             Some(node) => lead = node.next.as_ref(),
//             None => return dummy.next, // k trop grand
//         }
//     }

//     // Phase 2: maintenant, emprunt mutable uniquement
//     let mut trailer = &mut dummy.next;
//     let mut lead = &dummy.next; // redémarre un nouveau `lead`, en lecture

//     for _ in 0..k {
//         lead = &lead.as_ref().unwrap().next;
//     }

//     while lead.as_ref().is_some() {
//         lead = &lead.as_ref().unwrap().next;
//         trailer = &mut trailer.as_mut().unwrap().next;
//     }

//     if let Some(node) = trailer {
//         *trailer = node.next.take(); // suppression du nœud
//     }

//     dummy.next
// }

// // Phase 1: check if we can advance k steps
// fn can_advance_k(mut current: &Link, k: usize) -> bool {
//     for _ in 0..k {
//         match current {
//             Some(node) => current = &node.next,
//             None => return false,
//         }
//     }
//     true
// }

// // Phase 2: actual mutable manipulation
// fn remove_kth_last_node_core(mut dummy: Box<ListNode>, k: usize) -> Link {
//     let mut trailer = &mut dummy.next;

//     // Advance leader k steps
//     for _ in 0..k {
//         trailer = &mut trailer.as_mut().unwrap().next;
//     }

//     let mut leader = &mut dummy.next;
//     while trailer.as_ref().is_some() {
//         trailer = &mut trailer.as_mut().unwrap().next;
//         leader = &mut leader.as_mut().unwrap().next;
//     }

//     if let Some(node) = leader {
//         *leader = node.next.take();
//     }

//     dummy.next
// }

// // Public function
// fn remove_kth_last_node(head: Link, k: usize) -> Link {
//     if can_advance_k(&head, k) {
//         remove_kth_last_node_core(Box::new(ListNode::new(-1, head)), k)
//     } else {
//         head // k > len → on ne modifie rien
//     }
// }

// fn remove_kth_last_node(head: Link, k: usize) -> Link {
//     let mut dummy = Box::new(ListNode::new(-1, head));
//     let mut trailer = &mut dummy;

//     // Phase 1: avancer trailer de k étapes AVANT de créer un second pointeur
//     for _ in 0..k {
//         match trailer.next {
//             Some(ref mut node) => {
//                 trailer = node;
//             }
//             None => return dummy.next, // liste trop courte
//         }
//     }

//     // Phase 2: maintenant on avance `leader` jusqu’à la fin,
//     // et `prev` suivra juste derrière
//     let mut leader = &mut trailer.next;
//     let mut prev = &mut dummy.next;

//     while let Some(ref mut node) = leader {
//         leader = &mut node.next;
//         if let Some(ref mut p) = prev {
//             prev = &mut p.next;
//         }
//     }

//     // Maintenant prev pointe juste avant le nœud à supprimer
//     if let Some(ref mut node) = prev {
//         *prev = node.next.take();
//     }

//     dummy.next
// }

// fn remove_kth_last_node(head: Link, k: usize) -> Link {
//     // Phase 1 : mesure de la taille (sans emprunt mutable)
//     let mut length = 0;
//     let mut curr = head.as_ref();
//     while let Some(node) = curr {
//         length += 1;
//         curr = node.next.as_ref();
//     }

//     if k == 0 || k > length {
//         return head;
//     }

//     // Phase 2 : suppression à l'indice (length - k)
//     let mut dummy = Box::new(ListNode::new(-1, head));
//     let mut current = &mut dummy.next;
//     for _ in 0..(length - k) {
//         current = &mut current.as_mut().unwrap().next;
//     }

//     if let Some(node) = current {
//         *current = node.next.take();
//     }

//     dummy.next
// }

fn main() {
    let vals = vec![1, 2, 3, 4, 5];
    let mut head = None; // Initialize previous_node as None

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
