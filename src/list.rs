//A Rust trait works similarly to an abstract class in C++
//except that we can't hold data inside of them,
//they are only for behavior
pub trait List {
    //This will be whatever type we want to contain in the list
    //The technical reason we need this here is a little complex
    //Rool with it
    type Val;
    
    //add a node as the last item of the list
    fn append(elem: &Self::Val);

    //remove all of the nodes of the list
    fn clear();

    //return an item T at a specific position in the list
    fn get_element(position: usize) -> Option<Self::Val>;

    //return the length of the list
    fn get_length() -> i32;

    //inserts the elem at the position specified in the list
    fn insert(position: usize, elem: &Self::Val) -> Result<(), String>;

    //checks if the length is 0 or not
    fn is_empty() -> bool;

    //removes the element at the position specified in the list
    fn remove(position: usize) -> Result<(), String>;

    //replaces the elem in the list at the position specified
    fn replace(position: usize, elem: &Self::Val) -> Result<(), String>;
}