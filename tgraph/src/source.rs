// -------------------------------------------------------------------------------
// Here we declare
// -------------------------------------------------------------------------------
DeclareCollectionEnum!{Source; VecInput}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum VecInput {
    F32(Vec<f32>),
    F64(Vec<f64>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    Bool(Vec<bool>),
}

// -------------------------------------------------------------------------------
// These traits are used to extract data and shape from multi-dimensional arrays
// -------------------------------------------------------------------------------
pub trait FlatVecInput {
    fn flatvec(self) -> VecInput;
}

pub trait Shape<const N: usize> {
    fn shape(&self) -> [usize; N];
}

// -------------------------------------------------------------------------------
// These macros derive FlatVecInput trait for 1..10 dimensional arrays
// -------------------------------------------------------------------------------
macro_rules! mk_arr_type {
    ($t:ty; $n:ident $($m:ident)*) => {mk_arr_type!{[$t; $n]; $($m)*}};
    ($t:ty; ) => { $t }
}
macro_rules! prod {
    ($n: ident, $($m: ident, )+) => {$n * prod!($($m, )+)};
    ($n: ident, ) => {$n}
}
macro_rules! DeriveFlatVecInput {(
    $T:ident ($t:ident); $N:literal $($M:ident)*
) => {
    impl<$(const $M: usize, )*> FlatVecInput for mk_arr_type!{$t; $($M)*}
        where [(); {prod!($($M, )*)}] : Sized
    {
        fn flatvec(self) -> VecInput {
            let flattened = unsafe {
                let cast = &self;
                let cast = cast as *const mk_arr_type!{$t; $($M)*};
                // I thought of 1 $(*$M), but it invoked weird compiler bugs
                &*(cast as *const [$t; {prod!($($M, )*)}])
            };
            VecInput::$T(flattened.to_vec())
        }
    }
    impl<$(const $M: usize, )*> Shape<$N> for mk_arr_type!{$t; $($M)*} {
        fn shape(&self) -> [usize; $N] { [$($M, )*] }
    }
}}
macro_rules! BatchDeriveFlatVecInput {(
    $T:ident ($t:ident)
) => {
    DeriveFlatVecInput!{$T($t); 1  N0}
    DeriveFlatVecInput!{$T($t); 2  N0 N1}
    DeriveFlatVecInput!{$T($t); 3  N0 N1 N2}
    DeriveFlatVecInput!{$T($t); 4  N0 N1 N2 N3}
    DeriveFlatVecInput!{$T($t); 5  N0 N1 N2 N3 N4}
    DeriveFlatVecInput!{$T($t); 6  N0 N1 N2 N3 N4 N5}
    DeriveFlatVecInput!{$T($t); 7  N0 N1 N2 N3 N4 N5 N6}
    DeriveFlatVecInput!{$T($t); 8  N0 N1 N2 N3 N4 N5 N6 N7}
    DeriveFlatVecInput!{$T($t); 9  N0 N1 N2 N3 N4 N5 N6 N7 N8}
    DeriveFlatVecInput!{$T($t); 10 N0 N1 N2 N3 N4 N5 N6 N7 N8 N9}
}}
BatchDeriveFlatVecInput!{F32(f32)}
BatchDeriveFlatVecInput!{F64(f64)}
BatchDeriveFlatVecInput!{I32(i32)}
BatchDeriveFlatVecInput!{I64(i64)}
BatchDeriveFlatVecInput!{U32(u32)}
BatchDeriveFlatVecInput!{U64(u64)}
BatchDeriveFlatVecInput!{Bool(bool)}
// -------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_mk_arr_type() {
        const N0: usize = 1;
        const N1: usize = 2;
        const N2: usize = 3;
        const N3: usize = 4;
        let _x : mk_arr_type!{f32; N0 N1 N2 N3} = [[[[1f32; N0]; N1]; N2]; N3];
        assert!(_x == [[[[1f32; 1]; 2]; 3]; 4]);
    }

    #[test]
    fn check_from_const_array() {
        let x = [[1.0,2.0,3.0,4.0],[4.0,3.0,2.0,1.0],[5.0,6.0,7.0,8.0]];
        let x = (x.shape(), x.flatvec());
        let y = ([4usize, 3usize],
            VecInput::F64(vec![1.0,2.0,3.0,4.0,4.0,3.0,2.0,1.0,5.0,6.0,7.0,8.0])
        );
        assert!(x == y, "{x:?}\n{y:?}");
        let x = [[1, 2, 3, 4],[4, 3, 2, 1],[5, 6, 7, 8]];
        let x = (x.shape(), x.flatvec());
        let y = ([4usize, 3usize], 
            VecInput::I32(vec![1,2,3,4,4,3,2,1,5,6,7,8]));
        assert!(x == y, "{x:?}\n{y:?}");
    }
}
