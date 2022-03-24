// modules defined in ./src/<module_name>.rs
mod prime;
mod pagination;

// exposed functions
pub use prime::is_prime;
pub use pagination::amount_of_pages;
