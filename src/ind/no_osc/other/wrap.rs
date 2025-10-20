use bc_utils_lg::types::structures::{
    ARGS, BF_VEC, SLICE1_ARG, SLICE_ARG
};
use num_traits::Float;


pub fn wrap_bf_abstr<T>(
    src: &SLICE_ARG<T>, 
    _: &ARGS<T>, 
    _: &mut BF_VEC<T>
) -> T
where 
    T: Float
{
    src[0]
}

pub fn wrap_coll_abstr<T, C>(
    src: &SLICE1_ARG<T>,
    _: &ARGS<T>
) -> C
where 
    T: Float,
    C: FromIterator<T>
{
    src[0].iter().copied().collect()
}
