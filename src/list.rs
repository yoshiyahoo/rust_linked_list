//A Rust trait works similarly to an abstract class in C++
//except that we can't hold data inside of them,
//they are only for behavior
pub trait List {
    //This will be whatever type we want to contain in the list
    //The technical reason we need this here is a little complex
    //Rool with it
    type Val;
    
    //self helps us access a certain instance of something that implements the list trait
    //we need that instance so we can modify it, like in C++

    //add a node as the last item of the list
    fn append(&mut self, elem: Self::Val);

    //remove all of the nodes of the list
    fn clear(&self);

    //return an item T at a specific position in the list
    //This function should produce an error when our index is out of range
    //A result enum can wrap our error for us.
    fn get_element(&self, position: usize) -> Result<Self::Val, String>;

    //return the length of the list as an unsigned number
    fn get_length(&self) -> usize;

    //inserts the elem at the position specified in the list.
    //This function should produce an error when we our index is out of range
    //A result enum can wrap our error for us.
    fn insert(&mut self, position: usize, elem: Self::Val) -> Result<(), String>;

    //checks if the length is 0 or not
    fn is_empty(&self) -> bool;

    //removes the element at the position specified in the list
    //This function should produce an error when our index is out of range
    //A result enum can wrap our error for us.
    fn remove(&mut self, position: usize) -> Result<(), String>;

    //replaces the elem in the list at the position specified
    //This function should produce an error when we our index is out of range
    //A result enum can wrap our error for us.
    fn replace(&mut self, position: usize, elem: Self::Val) -> Result<(), String>;
}