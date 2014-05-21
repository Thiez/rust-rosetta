// Implements http://rosettacode.org/wiki/Flatten_a_list

enum List {
  Items(Vec<List>),
  Item(int),
}

impl List {
  fn flatten(&self) -> Vec<int> {
    fn append_all(v: &mut Vec<int>, li: &List) {
      match *li {
        Item(i) => v.push(i),
        Items(ref subLi) => {
          for s in subLi.iter() {
            append_all(v,s);
          }
        }
      };
    }
    let mut res = Vec::new();
    append_all(&mut res,self);
    res
  }
}

#[cfg(not(test))]
fn main() {
  let list = Items(vec![
    Items(vec![Item(1)]),
    Item(2),
    Items(vec![Items(vec![Item(3),Item(4)]),Item(5)]),
    Items(vec![Items(vec![Items(vec![])])]),
    Item(7),
    Item(8),
    Items(vec![])]);
  println!("{}",list.flatten());
}

#[test]
fn test_flatten() {
  let list = Items(vec![
    Items(vec![Item(1)]),
    Item(2),
    Items(vec![Items(vec![Item(3),Item(4)]),Item(5)]),
    Items(vec![Items(vec![Items(vec![])])]),
    Item(7),
    Item(8),
    Items(vec![])]);
  assert_eq!(list.flatten(), vec![1,2,3,4,5,7,8]);
}
