macro_rules! DeclareCollectionEnum {(
    $T: ident; $($F: ident)*
) => {
    #[derive(Debug)]
    pub enum $T { $($F($F),)* }
	$(
		impl Into<$T> for $F {
			fn into(self) -> $T { $T::$F(self) }
		}
	)*
}}
