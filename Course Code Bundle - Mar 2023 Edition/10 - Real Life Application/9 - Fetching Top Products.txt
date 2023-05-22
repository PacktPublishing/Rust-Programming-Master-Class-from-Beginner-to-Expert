// -------------------------------------------
// 			Fetching Top Products
//           	- Description
//           	    -
//           	- Tools
//           	    - Linklist + Iterators
// -------------------------------------------

#[derive(Debug)]

struct Linklist<T: std::fmt::Debug> {
    head: pointer<T>,
}

#[derive(Debug)]

struct Node<T> {
    element: T,
    next: pointer<T>,
}

type pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug> Linklist<T> {
    fn create_empty_list() -> Linklist<T> {
        Linklist { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element: element,
            next: previous_head,
        });
        self.head = Some(new_head);
    }

    fn reverse(&mut self) {
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }

        let mut prev = None;
        let mut current_node = self.head.take();
        while current_node.is_some() {
            let next = current_node.as_mut().unwrap().next.take();
            current_node.as_mut().unwrap().next = prev.take();
            prev = current_node.take();
            current_node = next;
        }

        self.head = prev.take();
    }

    fn remove(&mut self) -> Option<T> {
        let previous_head = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None,
        }
    }
    fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(H) => Some(&H.element),
            None => None,
        }
    }
    fn printing(&self) {
        let mut list_traversal = &self.head;
        println!();

        while true {
            match list_traversal {
                Some(Node) => {
                    print!("{:?} ", Node.element);
                    list_traversal = &list_traversal.as_ref().unwrap().next; // It may think that you may try to change the underlying data
                }
                None => break,
            }
        }
    }
}

fn sorting_lists(vec_list: &mut Vec<Linklist<i32>>) -> Linklist<i32> {
    let mut sortted_list: Linklist<i32> = Linklist::create_empty_list();

    let mut values: Vec<i32> = Vec::new();

    while true {
        let values = vec_list
            .into_iter()
            .map(|x| x.head.as_ref().unwrap().element)
            .collect::<Vec<i32>>();

        let min_val = *values.iter().min().unwrap();
        let min_index = values.iter().position(|x| *x == min_val).unwrap();

        sortted_list.add(min_val);
        vec_list[min_index].remove();

        if vec_list[min_index].head.is_none() {
            vec_list.remove(min_index);
        }

        if vec_list.len() == 0 {
            break;
        }
    }
    sortted_list
}

fn main() {
    // an empty list
    let mut list1 = Linklist::create_empty_list();
    list1.add(45);
    list1.add(40);
    list1.add(35);
    list1.add(23);
    list1.add(11);

    let mut list2 = Linklist::create_empty_list();
    list2.add(60);
    list2.add(44);

    let mut list3 = Linklist::create_empty_list();
    list3.add(85);
    list3.add(20);
    list3.add(15);

    let mut result = sorting_lists(&mut vec![list1, list2, list3]);
    result.printing();

    result.reverse();
    result.printing();
}
