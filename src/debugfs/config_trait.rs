use std::fmt::Debug;

pub trait Config: TryFrom<u64> + Debug + Copy  {
    fn to_config(&self) -> String;
}

macro_rules! impl_config {
    ($($t:ty),*) => {
        $(
            impl Config for $t {
                fn to_config(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_config!(u64);
