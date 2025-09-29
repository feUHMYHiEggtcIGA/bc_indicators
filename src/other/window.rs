use bc_utils_lg::{structs::settings::SETTINGS_IND, types::maps::MAP_ARGS};
use bc_utils_lg::types::maps::MAP_LINK;

use crate::map::other::{map_func_window_func_bf, map_window_ind};

pub fn window_max_settings(
    args_for_ind: &MAP_ARGS<f64>,
    settings_ind: &MAP_LINK<String, SETTINGS_IND>
) -> Result<usize, String>
{
    map_window_ind(
        &map_func_window_func_bf(), 
        settings_ind, 
        args_for_ind
    ).into_iter().map(|v| v.1).max().ok_or("yo".to_string())
}
