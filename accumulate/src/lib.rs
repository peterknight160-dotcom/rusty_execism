/// What should the type of function be?
pub fn map<T, F, U>(input: Vec<T>,  function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
   
    input.into_iter().map(function).collect()
}
