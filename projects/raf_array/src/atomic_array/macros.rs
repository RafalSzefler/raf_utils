macro_rules! readonly {
    (
        $visible:vis struct $name:ident {
            $($fname:ident : $ftype:ty),* $(,)?
        }
    ) => {
        $visible struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            #[inline(always)]
            $visible const fn new($($fname : $ftype),*) -> $name {
                $name { $($fname),* }
            }

            #[inline(always)]
            $($visible const fn $fname(&self) -> &$ftype {
                &self.$fname
            })*
        }
    }
}

pub(super) use readonly;

macro_rules! readonly_by_value {
    (
        $visible:vis struct $name:ident {
            $($fname:ident : $ftype:ty),* $(,)?
        }
    ) => {
        $visible struct $name {
            $($fname : $ftype),*
        }

        impl $name {

            #[inline(always)]
            $visible const fn new($($fname : $ftype),*) -> $name {
                $name { $($fname),* }
            }

            #[inline(always)]
            $($visible const fn $fname(&self) -> $ftype {
                self.$fname
            })*
        }
    }
}

pub(super) use readonly_by_value;
