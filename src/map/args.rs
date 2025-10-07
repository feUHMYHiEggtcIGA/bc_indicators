use num_traits::Float;
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::types::maps::*;
use bc_utils_lg::structs::settings::SETTINGS_IND;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn map_args_ind<'a, T>(
    settings: &'a MAP_LINK<String, SETTINGS_IND>
) -> MAP_ARGS<'a, T>
where  
    T: Float,
{
    settings
        .iter()
        .map(
            |setting| 
            (
                setting.0.as_str(),
                match setting.1.key.as_str() {
                    "sma" | "rma" | "ema" | "rsi" | "mm_scaler" => vec![T_ARGS::<T>::Usize(setting.1.kwargs_usize["window"])],
                    "nohesi" => vec![
                        T_ARGS::<T>::Float(T::from(setting.1.kwargs_f64["hesi"]).unwrap()),
                    ],
                    "avg" | "trend_ma" | "time_frsrc" | "div" | "minus" | "plus" | "mult"  => vec![],
                    _ => panic!("key indication unknown"),
                }
            )
        )
        .collect()
}
