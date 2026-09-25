mod b_tree_map;
mod b_tree_set;
mod binary_heap;
mod boxed;
mod cell;
mod cin;
mod cout;
mod execute;
mod hash_map;
mod hash_set;
mod linked_list;
mod mutex;
mod path;
mod rc;
mod ref_cell;
mod rw_lock;
mod string;
mod vector;
mod vector_deque;

use std_macro_extensions::*;

use std::{
    boxed::Box,
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    rc::Rc,
    sync::{Mutex, MutexGuard, RwLock},
};
