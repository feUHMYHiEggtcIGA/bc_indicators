use bc_indicators::ind::osc::other::time::{
    time_frsrc,
    time_frsrc_coll,
};


#[test]
fn time_frsrc_res_1()
{
    assert_eq!(time_frsrc(&1759518000000.), 0.7916666666666666);
    assert_eq!(time_frsrc(&1759521600000.), 0.8333333333333334);
}

#[test]
fn time_frsrc_coll_res_1()
{
    assert_eq!(
        time_frsrc_coll::<f64, Vec<f64>>(&[1759518000000., 1759521600000.,]),
        vec![0.7916666666666666, 0.8333333333333334], 
    );
}
