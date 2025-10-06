use bc_utils_lg::types::structures::{
    ARGS, 
    BF_VEC, 
    SLICE_ARG,
    SLICE1_ARG
};
use bc_utils_lg::types::maps::MAP;
use bc_utils_lg::enums::indicators::T_HASHMAP;
use num_traits::Float;


pub fn bf_window<T>(
    src: &SLICE_ARG<T>,
    window: &usize,
    exc_last: &bool,
) -> MAP<&'static str, Vec<T>>
where 
    T: Float
{
    let minus = if *exc_last {0} else {1};
    MAP::from_iter([
        ("window", src[src.len() - minus - *window..src.len() - minus].to_vec())
    ])
}

pub fn bf_window_abstr<T>(
    src: &SLICE1_ARG<T>,
    a: &ARGS<T>,
    exc_last: &bool,
) -> BF_VEC<T>
where 
    T: Float
{
    vec![
        T_HASHMAP::VecF(bf_window(src[0], a[0].unwrap_usize(), exc_last))
    ]
}
