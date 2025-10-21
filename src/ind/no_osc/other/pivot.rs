use std::ops::AddAssign;

use bc_utils::{nums::avg, other::roll_slice1};
use bc_utils_lg::types::{maps::MAP, structures::{ARGS, BF_VEC, SLICE1_ARG, SLICE_ARG}};
use num_traits::Float;

use crate::bf::window::bf_window;


pub fn pivot_bf<T>(
    src: &T,
    type_: &str,
    bf: &mut MAP<&str, Vec<T>>,
) -> T
where 
    T: Float,
    T: AddAssign
{
    roll_slice1(bf.get_mut("window").unwrap(), &-1);
    let lenbf = bf["window"].len();
    bf.get_mut("window").unwrap()[lenbf - 1] = *src;
    let mut slc = bf["window"].clone();
    slc.sort_by(|v, vv| v.partial_cmp(vv).unwrap());
    let divv = slc
        .iter()
        .enumerate()
        .zip(slc.iter().enumerate().skip(1))
        .map(|(v, vv)|(v.0,  *v.1 - *vv.1));
    let i = divv.min_by(
        |v, vv| v.1.partial_cmp(&vv.1).unwrap()
    ).unwrap().0;
    avg(match type_ {
        "s" => &slc[..i],
        _ => &slc[i..],
    })
}

pub fn pivot_bf_abstr<T>(
    src: &SLICE_ARG<T>,
    args: &ARGS<T>,
    bf: &mut BF_VEC<T>,
) -> T
where 
    T: Float,
    T: AddAssign,
{
    pivot_bf(&src[0], args[1].unwrap_str(), &mut bf[0].unwrap_vec_f())
}

pub fn pivot_coll<T, C>(
    src: &SLICE_ARG<T>,
    window: &usize,
    type_: &str,
) -> C
where 
    T: Float,
    T: AddAssign,
    C: FromIterator<T>
{
    let mut bf = bf_window(&src[..*window], window, &true);
    src
        .iter()
        .enumerate()
        .map(|(i, v)| if i < *window {
            T::nan()
        } else {pivot_bf(v, type_, &mut bf)})
        .collect()
}

pub fn pivot_coll_abstr<T, C>(
    src: &SLICE1_ARG<T>,
    args: &ARGS<T>
) -> C
where 
    T: Float,
    T: AddAssign,
    C: FromIterator<T>
{
    pivot_coll::<T, C>(&src[0], args[0].unwrap_usize(), args[1].unwrap_str())
}
