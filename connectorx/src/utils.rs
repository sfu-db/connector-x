use anyhow::Result;
use rust_decimal::Decimal;
use std::ops::{Deref, DerefMut};

pub struct DummyBox<T>(pub T);

impl<T> Deref for DummyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DummyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn decimal_to_i128(mut v: Decimal, scale: u32) -> Result<i128> {
    v.rescale(scale);

    let v_scale = v.scale();
    if v_scale != scale as u32 {
        return Err(anyhow::anyhow!(
            "decimal scale is not equal to expected scale, got: {} expected: {}",
            v_scale,
            scale
        ));
    }

    Ok(v.mantissa())
}
