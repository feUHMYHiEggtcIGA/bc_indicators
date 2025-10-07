use bc_utils_lg::statics::prices::{
    CLOSE, 
    CLOSE_LAST, 
    OPEN, 
    OPEN_LAST, 
    SRC, SRC_TRANSPOSE,
};

use bc_indicators::ind::no_osc::math::mult::*;
use bc_indicators::gw::ind::*;
use bc_indicators::gw::bf::gw_func_bf_ind;
use bc_indicators::map::ind::{
    map_ind_t_bf, 
    map_ind_coll
};
use bc_indicators::map::args::map_args_ind;
use bc_indicators::map::bf::map_func_bf_ind;
use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_USED_SRC};
use bc_utils_lg::types::maps::{MAP, MAP_LINK};


#[test]
fn mult_coll_res_1()
{
    let res = mult_coll::<_, Vec<_>>(OPEN.as_slice(), CLOSE.as_slice());
    assert_eq!(res.last().unwrap(), &(OPEN_LAST * CLOSE_LAST));
    let resnan = mult_coll::<_, Vec<_>>(&[f64::NAN,], &[f64::NAN,]);
    assert!(resnan[0].is_nan());
}

#[test]
fn gw_div_res_1()
{
    let s = MAP_LINK::from_iter([("mult".to_string(), SETTINGS_IND{
        key: "mult".to_string(),
        used_src: vec![
            SETTINGS_USED_SRC{
                key: "open".to_string(),
                sub_from_last_i: 0
            },
            SETTINGS_USED_SRC{
                key: "close".to_string(),
                sub_from_last_i: 0
            }
        ],
        used_ind: vec![],
        kwargs_f64: MAP::default(),
        kwargs_usize: MAP::default(),
        kwargs_string: MAP::default(),
    })]);
    let mapindcoll = map_ind_coll::<Vec<_>, f64>();
    let mapindbf = map_ind_t_bf::<f64>();
    let arg = map_args_ind::<f64>(&s);
    let mapfnbfind = map_func_bf_ind();
    let rescoll = gw_ind_coll::<f64, Vec<_>>(&SRC, &s, &mapindcoll, &mapindcoll, &arg);
    let mut bf = gw_func_bf_ind(&SRC_TRANSPOSE, &s, &mapfnbfind, &mapindcoll, &arg, &false);
    let res = gw_ind_bf(&SRC_TRANSPOSE, &s, &mapindbf, &arg, &mut bf);
    assert_eq!(rescoll["mult"].last().unwrap(), &res["mult"]);
}
