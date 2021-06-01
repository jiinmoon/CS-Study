# Standard Collections

Rust standard library provides useful collections to work in.

`Vec<T>` is a vector/list structure that grows dynamically.

`VecDeque<T>` is a deque structure that optimizes on operating on either end of
the list (front or end).

`LinkedList<T>` is a doubly linkedlist.

`BinaryHeap<T> where T : Ord` is a max heap/priority queue (Python `heapq`).

`HashMap<K,V> where K : Eq+Hash` is a dictionary (unordered dictionary/map).

`BTreeMap<K,V> where K : Ord` is a dictionary where key is sorted/ordered.

`HashSet<T> where T : Eq+Hash` is a basic set structure.

`BTreeSet<T> where T : Ord` is an orderd/sorted set.

## `Vec`

Array is great when you know beforehand the size of data to store; but
sometimes, you require flexibility to grow dynamically. For example, if we are
reading an input from someone and we do not know how much memory it would
require to store them all.

```rust
fn vectors() {
    // initialize Vec<T>
    // has to be mutable in order to be useful
    let mut vec1 = Vec::new();
    vec1.push(1);
    // following is invalid
    //vec1.push(2);
    vec1.push(2);
    vec1.push(3);
    // Vec<T> implements Display
    println!("{:?}", vec1);
    // can be indexed like an array
    // note that index has to be usize (not isize)
    let idx:usize = 1;
    vec1[idx] = 99;
    // when index is out of bounds, this will raise a panic
    // this is not caught at compilation time
    // i.e. vec1[10000] will raise a panic
    // to avoid this, Vec<T> has get function
    // get is an Option<T> which we can use match to check for returned value
    // it would be either Some(T) or None
    match vec1.get(10000) {
        Some(val) => println!("vec1[10000] = {}", val),
        None => println!("Index out of bound!")
    }

    for i in 0..vec1.len() {
        println!("vec1[{}] = {}", i, vec1[i]);
    }
    // Vec<T> implements iterable
    for val in &vec1 {
        print!("{} ", val);
    }

    // remove last element
    // what if vec1 was empty?
    // notice how Vec<T>.pop() returns option to check
    while !vec1.is_empty() {
        let val = vec1.pop();
        match val {
            Some(x) => println!("{}", x),
            None => println!("")
        }
    }

    for i in 1..=5 {
        vec1.push(i);
    }

    // alternative way
    // this will run until Some() is returned
    while let Some(x) = vec1.pop() {
        println!("{}", x);
    }
}
```

## Side note on references in Rust

```rust
fn add_one(num:&mut i32) {
    println!("Before {:p} ==> {}", num, *num);
    *(num) += 1;
    println!("After {:p} ==> {}", num, *num);
}

fn references() {
    let mut num = 1;
    let num_ref = &num;
    let num_raw_ref = num_ref as *const i32;
    let num_addr = num_raw_ref as usize;
    // note when printing reference, it dereference automatically
    // more on https://stackoverflow.com/questions/27852613/why-does-printing-a-pointer-print-the-same-thing-as-printing-the-dereferenced-po
    println!("num {}, num_ref {}, num_raw_ref {:?}, num_addr {}",
             num, num_ref, num_raw_ref, num_addr);
    add_one(&mut num);
    println!("{}", num);
}
```

## HashMap

```rust
use std::collections::HashMap;

fn hash_maps() {
    let mut map = HashMap::new();
    map.insert(String::from("Apple"), 3);
    map.insert(String::from("Orange"), 11);
    println!("{:?}", map);

    // note reference to map in iterations
    // without &, iterations de-structures the map
    for (k, v) in &map {
        println!("{} == {} ", k, v);
    }

    map.insert(String::from("Apple"), 999);
    println!("{:?}", map);

    // note that program will panic if the key is not found in the map
    println!("There are {} apples", map["Apple".into()]);

    // to avoid panic, HashMap<K, V> has entry()
    map.entry("Banana".into()).or_insert(42);
    println!("{:?}", map);
    {
        // notice how this returns Entry<K, V> object
        // it references to actual value; so we can modify it
        let val = map.entry("Banana".into()).or_default();
        *val = 881;
    }
    println!("{:?}", map);
}
```

## HashSet

Same as mathmatical set - container of unique elements.

```rust
fn hash_sets() {
    let mut set = HashSet::new();
    set.insert("John");
    set.insert("Mary");

    println!("{:?}", set);

    // we can also check to see whether insertion was successful
    for element in ["Mike", "Duke", "John"].iter() {
        if set.insert(element) {
            println!("Inserted {}", element);
        } else {
            println!("Duplicate: Couldn't insert {}", element);
        }
    }

    if !set.contains("Nina") {
        println!("Nina is not in the set");
    }

    println!("{:?}", set);

    // remove() returns boolean value for us to check for
    if set.remove("Mike") {
        println!("Mike was removed successfully");
    }

    println!("{:?}", set);

    // typical set operations are provided
    let set_1to5: HashSet<_> = (1..=5).collect();
    let set_6to10: HashSet<_> = (6..11).collect();
    let set_1to10: HashSet<_> = (1..11).collect();
    let set_3to7: HashSet<_> = (3..=7).collect();

    println!("{:?}", set_1to5);
    if set_1to5.is_subset(&set_1to10) {
        println!("set 1 to 5 is subset of set 1 to 10");
    }
    println!("{:?}", set_1to5.intersection(&set_3to7));
    println!("{:?}", set_1to5.difference(&set_3to7));
    println!("{:?}", set_1to5.union(&set_3to7));
}
```

## Iterators

```rust
 fn iterators() {
    let vec1 = vec![1, 2, 3, 4, 5];

    // iterating without reference to vector will move the elements out
    // de-structuring the vec
    for element in vec1 {
        println!("{}", element);
    }

    // following will be invalid
    //println!("{:?}", vec1);

    // to fix this we either use & reference or Vec<T>.iter()
    let vec2 = vec![6, 7, 8, 9, 10];
    for element in vec2.iter() { // or &vec2
        println!("{}", element);
        // we cannot modify the elements within
        //element += 1; would be invalid
    }

    println!("{:?}", vec2);

    // to be able to work on the values, we create mutable Vec
    let mut vec3 = vec![11, 12, 13, 14, 15];
    // also, we need mutable iterator to bring out the references
    for val in vec3.iter_mut() {
        println!("val was {}", val);
        *val += 1;
        println!("val is now {}", val);
    }

    println!("{:?}", vec3);

    for val in vec3.iter().rev() {
        println!("{}", val);
    }

    // transform collections to iterator
    // (move elements out)
    let mut vec4 = vec![0, 0, 0];
    let mut vec5 = vec![1, 1, 1];
    // .extend() uses vec4.into_iter() method
    // vec4.into_iter() transforms vec4 into iterator
    // vec4 will then be unavailable to use
    vec5.extend(vec4);
    println!("{:?}", vec5);
}
```

