// Last updated: 6/6/2025, 3:32:02 PM
use std::{cell::RefCell, rc::Rc};

impl Solution 
{
    pub fn is_subsequence(s: String, t: String) -> bool 
    {
        let recursion = false;
        let recursion_closure = true;
        
        let (sb, tb) = (s.as_bytes(), t.as_bytes());
        let (mut S, mut T, mut l, mut r) = (sb.len(), tb.len(), 0, 0);

        if recursion
        {
            fn recursion(l: usize, r: usize, sb: &[u8], tb: &[u8], S: usize, T: usize) -> bool
            {
                if l == S { return true; }
                if r == T { return false; }

                if (sb[l] == tb[r]) { return recursion(l+1, r+1, sb, tb, S, T) }
                return recursion(l, r+1, sb, tb, S, T)
            }

            return recursion(0, 0, sb, tb, S, T)
        }
        else if recursion_closure
        {
            // Recursive Closure (Y-Combinator) with recursive self-referential dynamically defined function

            // Type: 
            //    - Rc<>: Allows shared ownership of closure (closure_holder captured self-referentially by captured_holder)
            //    - dyn Fn(): a read-only (immutable) trait for a closure : (usize, usize) --> bool
            //    - + 'a: inferred lifetime closure such that captured references live as long as the lifetime 'a
            type RecursiveFn<'a> = Rc<dyn Fn(usize, usize) -> bool + 'a>;

            // Closure Holder
            //    - Rc<RefCell<Option<RecursiveFn<'a>>>>>:
            //        - Rc: Shared ownership
            //        - RefCell : "interior mutability" to modify the `Option` to defer evaluation while `closure_holder` might be shared
            //        - Option<RecursiveFn<'a>>: Base Case that breaks circular dependencies with `None` initially, and later `Some(our_actual_closure)`
            let master_sword: Rc<RefCell<Option<RecursiveFn>>> = Rc::new(RefCell::new(None));

            // Clone the holder so the Rc clone will be captured by the closure
            let fighter_sword = master_sword.clone();                                                   // Create new Rc ptr: Rc<RefCell<Option<RecursiveFn<'a>>>> where RecursiveFn<'a> is defined above

            // Define the closure's logic.
            //    - move: takes ownership of all variables it captures from the environment, or copies them if they are `Copy` types (`S` and `T` : usize, `sb` and `tb` : &[u8])
            //        - `captured_holder` (`Rc<...`) is moved (ownership of this `Rc` instance transferred).
            let ocarina = move |sidx: usize, tidx: usize| -> bool                                       // RecursiveFn<'a> = Rc<dyn Fn(usize, usize) -> bool + 'a>
            {
                if sidx == S { return true; }
                if tidx == T { return false; }

                // Take ownership of the captured_holder to get the Recursive Fn address
                let borrow_fighters_sword = fighter_sword.borrow();                                     // Borrow Ref: Rc<RefCell<Option<RecursiveFn<'a>>>> --> Ref<Option<RecursiveFn<'a>>>
                let ocarina_song: &RecursiveFn = borrow_fighters_sword                                  // returns &RecursiveFn<'a> ( &Rc<dyn Fn(...) + 'a> )

                    .as_ref()                                                                           // converts &Option<T> --> Option<&T> ( &Option<RecursiveFn<'a>> --> Option<&RecursiveFn<'a>> )
                    .expect("Recursive closure was not set in its holder.");                            // None / Some(value) --> unwrap() --> value

                
                if sb[sidx] == tb[tidx] { (*ocarina_song)(sidx + 1, tidx + 1) }                         // Call the dyn Fn. Rc dereferences to the dyn Trait.
                else { (*ocarina_song)(sidx, tidx + 1) }
            };
            
            
            let ocarina_of_time: RecursiveFn = Rc::new(ocarina);                                        // Box (Rc) closure on heap to make it a trait object

            *master_sword.borrow_mut() = Some(ocarina_of_time);                                         // Mutate shared reference on heap to point to RecursiveFn logic

            let final_boss;
            {
                let borrow_master_sword = master_sword.borrow();
                if let Some(result) = borrow_master_sword.as_ref() { final_boss = (*result)(0, 0); }
                else { panic!("Recursive closure was not properly initialized for the first call."); }
            }
            return final_boss;
        }
        else
        {
            while l < S && r < T
            {
                if (sb[l] == tb[r]) { l += 1; }
                r += 1;
            }

            S == l
        }
    }
}


