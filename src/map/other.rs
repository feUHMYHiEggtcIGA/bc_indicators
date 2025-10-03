use bc_utils_lg::structs::settings::SETTINGS_IND;
use num_traits::Float;
use bc_utils_lg::types::funcs::FUNC_USIZE;
use bc_utils_lg::types::maps::*;
use bc_utils_lg::types::structures::ARGS;
use rustc_hash::FxHashMap;

pub fn map_func_window_func_bf<'a, T>() -> MAP_FUNC_USIZE<'a, T> 
where 
    T: Float,
{
    MAP_FUNC_USIZE::from_iter([
        ("sma", |a: &ARGS<T>| -> usize {*a.first().unwrap().unwrap_usize() + 1} as FUNC_USIZE<T>),
        ("ema", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 1} as FUNC_USIZE<T>),
        ("rma", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 1} as FUNC_USIZE<T>),
        ("rsi", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 2} as FUNC_USIZE<T>),
        ("nohesi", |_: &ARGS<T>| -> usize {3} as FUNC_USIZE<T>),
        ("trend_ma", |_: &ARGS<T>| -> usize {2} as FUNC_USIZE<T>),
        ("tims_frsrc", |_: &ARGS<T>| -> usize {0} as FUNC_USIZE<T>),
    ])
}

pub fn map_window_ind<'a, T>(
    map_func_window: &MAP_FUNC_USIZE<T>, 
    settings: &'a MAP_LINK<String, SETTINGS_IND>,
    map_args: &MAP_ARGS<T>
) -> FxHashMap<&'a str, usize>
where 
    T: Float
{
    settings
        .iter()
        .map(
            |setting| {
                let key_uniq = setting.0.as_str();
                (
                    key_uniq,
                    map_func_window[setting.1.key.as_str()](
                        &map_args[key_uniq]
                    )
                )
            }
        )
        .collect()
}
