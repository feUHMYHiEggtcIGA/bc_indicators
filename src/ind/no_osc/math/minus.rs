use bc_utils_lg::types::structures::{
    ARGS, BF_VEC, SLICE1_ARG, SLICE_ARG
};
use num_traits::Float;


pub fn minus_bf_abstr<T>(
    src: &SLICE_ARG<T>,
    _: &ARGS<T>,
    _: &mut BF_VEC<T>
) -> T
where 
    T: Float
{
    src[0] - src[1]
}

pub fn minus_coll<T, C>(
    v: &SLICE_ARG<T>,
    v2: &SLICE_ARG<T>,
) -> C
where 
    T: Float,
    C: FromIterator<T>
{
    v
        .iter()
        .zip(v2.iter())
        .map(|(vv, vvv)| *vv - *vvv)
        .collect()
}

pub fn minus_coll_abstr<T, C>(
    src: &SLICE1_ARG<T>,
    _: &ARGS<T>
) -> C
where 
    T: Float,
    C: FromIterator<T>
{
   minus_coll::<T, C>(&src[0], &src[1])
}
