#[macro_export] macro_rules! define_interface {
    ($(#[ $stra:meta ])* $strv:vis struct $name:ident { $($inner:tt)* }) => {
        define_interface! {
            $(#[$stra])* $strv $name, | $($inner)*
        }
    };
    (
        $(#[ $stra:meta ])* $strv:vis $name:ident,
        $($fieldv:vis $field:ident : $fieldt:ty,)* |
        $v:vis fn $n:ident (*const self $(, $argtype:ty)*) $(-> $rettype:ty)?,
        $($rest:tt)*
    ) => {
        define_interface!(
            $(#[$stra])* $strv $name,
            $($fieldv $field: $fieldt,)*
            $v $n: unsafe extern "win64" fn(*const $name $(, $argtype)*) $(-> $rettype)?,
            | $($rest)*
        );
    };
    (
        $(#[ $stra:meta ])* $strv:vis $name:ident,
        $($fieldv:vis $field:ident : $fieldt:ty,)* |
        $v:vis $n:ident : $t:ty,
        $($rest:tt)*
    ) => {
        define_interface!(
            $(#[$stra])* $strv $name,
            $($fieldv $field: $fieldt,)*
            $v $n: $t,
            | $($rest)*
        );
    };
    ($(#[ $stra:meta ])* $strv:vis $name:ident, $($fieldv:vis $field:ident : $fieldt:ty,)* |) => {
        $(#[$stra])* $strv struct $name {
            $($fieldv $field: $fieldt,)*
        }
    };
}
