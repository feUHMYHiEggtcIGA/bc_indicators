use bc_utils_lg::types::structures::{ARGS, BF_VEC, SLICE1_ARG, SLICE_ARG};
use num_traits::Float;


pub fn time_frsrc<T>(
    src: &T,
) -> T
where
    T: Float,
{
    let dayms = T::from(86400000u32).unwrap();
    *src % dayms / dayms 
}

pub fn time_frsrc_abstr<T>(
    src: &SLICE_ARG<T>,
    _: &ARGS<T>
) -> T
where
    T: Float
{
    time_frsrc(&src[0])
}

pub fn time_frsrc_bf_abstr<T: Float>(
    src: &SLICE_ARG<T>,
    _: &ARGS<T>,
    _:  &mut BF_VEC<T>,
) -> T
{
    time_frsrc(&src[0])
}

pub fn time_frsrc_coll<T, C>(
    src: &SLICE_ARG<T>
) -> C
where
    T: Float,
    C: FromIterator<T>,
{
    src
        .iter()
        .map(|v| time_frsrc(v))
        .collect()
}

pub fn time_frsrc_coll_abstr<T, C>(
    src: &SLICE1_ARG<T>,
    _: &ARGS<T>
) -> C
where
    T: Float,
    C: FromIterator<T>
{
    time_frsrc_coll::<T, C>(&src[0])
}
