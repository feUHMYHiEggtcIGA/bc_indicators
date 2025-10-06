use std::iter::Sum;
use std::ops::{
    AddAssign,
    DivAssign
};
use std::vec;

use bc_utils_lg::funcs::src::transpose;
use num_traits::Float;
use bc_utils_lg::types::maps::{MAP, MAP_LINK};
use bc_utils_lg::types::maps::*;
use bc_utils_lg::types::structures::*;
use bc_utils_lg::structs::settings::SETTINGS_IND;

use crate::gw::src::src_from_settings;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
#[allow(clippy::ptr_arg)]
#[allow(clippy::too_many_arguments)]
pub fn gw_ind_bf<'a, T>(
    buff_src: &SRC_TRANSPOSE<T>,
    settings: &'a MAP_LINK<String, SETTINGS_IND>,
    map_ind_bf_: &MAP_IND_T_BF<T>,
    map_args_ind_bf_: &MAP_ARGS<T>,
    map_bf_ind: &mut MAP_BF_VEC<T>,
) -> MAP<&'a str, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    T: DivAssign,
    // T: std::fmt::Debug,
{ 
    settings
        .iter()
        .fold(
            MAP::default(),
            |mut map, setting, | {
                let key_uniq_str = setting.0.as_str();
                let mut src_arg = vec![];
                for us_el in &setting
                    .1
                    .used_src
                {
                    src_arg.push({
                        let sk = &buff_src[&us_el.key];
                        sk[sk.len() - 1 - us_el.sub_from_last_i]
                    });
                }
                for ui_el in &setting
                    .1
                    .used_ind
                {
                    src_arg.push(map[ui_el.as_str()]);
                }
                map.insert(
                    key_uniq_str,
                    map_ind_bf_[setting.1.key.as_str()](
                        src_arg.as_slice(),
                        &map_args_ind_bf_[key_uniq_str],
                        map_bf_ind.get_mut(key_uniq_str).unwrap(),
                    )
                );
                map
            }
        )
}

pub fn gw_ind_coll<'a, T, C>(
    src: &SRC<T>,
    settings: &'a MAP_LINK<String, SETTINGS_IND>,
    map_ind_coll: &MAP_IND_COLL<C, T>,
    map_ind_vec: &MAP_IND_COLL<Vec<T>, T>,
    map_args: &MAP_ARGS<T>,
) -> MAP<&'a str, C>
where
    T: Float,
    T: Sum,
    T: AddAssign,
    T: DivAssign,
    C: FromIterator<T>
{
    let src_tr = transpose(src.clone()).unwrap();
    settings
        .into_iter()
        .map(
            |(k, v)| 
            (
                k.as_str(),
                map_ind_coll[v.key.as_str()](
                    src_from_settings(
                        &v.used_ind, 
                        &v.used_src, 
                        settings, 
                        &src_tr,
                        map_ind_vec,
                        map_args,
                    )
                        .iter()
                        .map(|vecc| vecc.as_slice())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    &map_args[k.as_str()]
                )
            )
        )
        .collect()
}
