use bc_indicators::map::args::map_args_ind;
use bc_indicators::map::ind::map_ind_coll;
use bc_utils_lg::statics::prices::{OPEN, OPEN_LAST, SRC};
use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_USED_SRC};
use bc_utils_lg::types::maps::{MAP, MAP_LINK};

use bc_indicators::ind::osc::math::mm_scaler::{
    mm_scaler_bf,
    mm_scaler_coll,
    mm_scaler,
};
use bc_indicators::bf::window::bf_window;
use bc_indicators::gw::ind::gw_ind_coll;


#[test]
fn mm_scaler_bf_res_1()
{
    let mut bf = bf_window(OPEN.as_slice(), &49, &false);
    let res = mm_scaler_bf(&OPEN_LAST, &mut bf);
    let open49 = &OPEN[1..];
    let maxx = open49.iter().max_by(|v, vv| v.partial_cmp(vv).unwrap()).unwrap();
    let minn = open49.iter().min_by(|v, vv| v.partial_cmp(vv).unwrap()).unwrap();
    assert_eq!(
        mm_scaler(OPEN_LAST, *minn, *maxx),
        res
    );
}

#[test]
fn mm_scaler_coll_res_1()
{
    let res = mm_scaler_coll::<_, Vec<_>>(OPEN.as_slice(), &10);
    let open10 = &OPEN[OPEN.len() - 10..];
    let maxx = open10.iter().max_by(|v, vv| v.partial_cmp(vv).unwrap()).unwrap();
    let minn = open10.iter().min_by(|v, vv| v.partial_cmp(vv).unwrap()).unwrap();
    assert_eq!(res.len(), OPEN.len());
    assert_eq!(res.last().unwrap(), &mm_scaler(OPEN_LAST, *minn, *maxx));
}

#[test]
fn mm_scaler_gw_res_1()
{
    let s = MAP_LINK::from_iter([("mm_scaler_1".to_string(), SETTINGS_IND{
        key: "mm_scaler".to_string(),
        kwargs_f64: MAP::default(),
        kwargs_usize: MAP::from_iter([("window".to_string(), 10)]),
        kwargs_string: MAP::default(),
        used_src: vec![SETTINGS_USED_SRC{
            key: "open".to_string(),
            sub_from_last_i: 0
        }],
        used_ind: vec![]
    })]);
    let mapind = map_ind_coll::<Vec<f64>, f64>();
    let argss = map_args_ind(&s);
    let res = gw_ind_coll(
        &SRC, 
        &s, 
        &mapind, 
        &mapind, 
        &argss,
    );
    assert_eq!(res["mm_scaler_1"].iter().filter(|v| !v.is_nan()).collect::<Vec<_>>(), mm_scaler_coll::<_, Vec<f64>>(OPEN.as_slice(), &10).iter().filter(|v| !v.is_nan()).collect::<Vec<_>>());
}
