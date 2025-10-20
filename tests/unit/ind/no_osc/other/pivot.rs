use bc_indicators::bf::window::bf_window;
use bc_indicators::ind::no_osc::other::pivot::{
    pivot_bf,
    pivot_coll
};

use bc_utils_lg::statics::prices::{ 
    OPEN, 
    OPEN_LAST, 
    SRC_TRANSPOSE,
};

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
fn pivot_coll_res_1()
{
    let res = pivot_coll::<_, Vec<_>>(OPEN.as_slice(), &10usize, "s");
    let mut bf = bf_window(&OPEN, &10usize, &false);
    assert_eq!(res.last().unwrap(), &pivot_bf(&OPEN_LAST, "s", &mut bf));
    let resnan = pivot_coll::<_, Vec<_>>(&[f64::NAN,], &1usize, "s");
    assert!(resnan[0].is_nan());
}

#[test]
fn gw_pivot_res_1()
{
    let s = MAP_LINK::from_iter([("pivot".to_string(), SETTINGS_IND{
        key: "pivot".to_string(),
        used_src: vec![
            SETTINGS_USED_SRC{
                key: "open".to_string(),
                sub_from_last_i: 0
            },
        ],
        used_ind: vec![],
        kwargs_f64: MAP::default(),
        kwargs_usize: MAP::from_iter([("window".to_string(), 10)]),
        kwargs_string: MAP::from_iter([("type".to_string(), "s".to_string())]),
    })]);
    let mapindc = map_ind_coll::<Vec<_>, f64>();
    let mapind = map_ind_t_bf();
    let maparg = map_args_ind(&s);
    let mapbf = map_func_bf_ind();
    let mut bf = gw_func_bf_ind(&SRC_TRANSPOSE, &s, &mapbf, &mapindc, &maparg, &false);
    let mut bfind = bf_window(&OPEN, &10, &false);
    assert_eq!(
        gw_ind_bf(&SRC_TRANSPOSE, &s, &mapind, &maparg, &mut bf)["pivot"],
        pivot_bf(&OPEN_LAST, "s", &mut bfind)     
    );
}
