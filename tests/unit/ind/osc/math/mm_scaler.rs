use bc_utils_lg::statics::prices::{OPEN, OPEN_LAST};

use bc_indicators::ind::osc::math::mm_scaler::{
    mm_scaler_bf,
    mm_scaller_coll,
    mm_scaler,
};
use bc_indicators::bf::window::bf_window;


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
    let res = mm_scaller_coll::<_, Vec<_>>(OPEN.as_slice(), &10);
    let open10 = &OPEN[OPEN.len() - 10..];
    let maxx = open10.iter().max_by(|v, vv| v.partial_cmp(vv).unwrap()).unwrap();
    let minn = open10.iter().min_by(|v, vv| v.partial_cmp(vv).unwrap()).unwrap();
    assert_eq!(res.len(), OPEN.len());
    assert_eq!(res.last().unwrap(), &mm_scaler(OPEN_LAST, *minn, *maxx));
}
