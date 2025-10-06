use bc_utils_lg::types::structures::{
    ARGS, 
    BF_VEC, 
    SLICE1_ARG, 
    SLICE_ARG,
};
use bc_utils_lg::types::maps::MAP;
use bc_utils::other::coll1_roll_replace_el;
use num_traits::Float;

use crate::bf::window::bf_window;


pub fn mm_scaler<T>(
    src: T, 
    minn: T, 
    maxx: T,
) -> T
where 
    T: Float
{
    (src - minn) / (maxx - minn)
}

pub fn mm_scaler_bf<T>(
    src: &T,
    bf: &mut MAP<&'static str, Vec<T>>,
) -> T
where 
    T: Float,
{
    let newvec = coll1_roll_replace_el::<Vec<T>, T, T>(
        bf
            .remove("window")
            .unwrap()
            .as_mut_slice(),
        &-1, 
        *src
    );
    bf.insert("window", newvec);
    let maxx = *bf["window"].iter().max_by(|v, vv| v.partial_cmp(vv).unwrap()).unwrap();
    let minn = *bf["window"].iter().min_by(|v, vv| v.partial_cmp(vv).unwrap()).unwrap();
    mm_scaler(*src, minn, maxx)
}

pub fn mm_scaler_bf_abstr<T>(
    src: &SLICE_ARG<T>,
    _: &ARGS<T>,
    bf: &mut BF_VEC<T>,
) -> T
where 
    T: Float,
{
    mm_scaler_bf(&src[0], bf[0].unwrap_vec_f())
}

pub fn mm_scaler_coll<T, C>(
    src: &SLICE_ARG<T>,
    window: &usize,
) -> C
where
    T: Float,
    C: FromIterator<T>,
{
    let mut bf = bf_window(&src[..*window], window, &true);
    src
        .iter()
        .enumerate()
        .map(|(i,v)| if i < *window {T::nan()} else {mm_scaler_bf(v, &mut bf)})
        .collect()
}

pub fn mm_scaler_coll_abstr<T, C>(
    src: &SLICE1_ARG<T>,
    args: &ARGS<T>,
) -> C
where 
    T: Float,
    C: FromIterator<T>,
{
    mm_scaler_coll::<T, C>(&src[0], args[0].unwrap_usize())
}
