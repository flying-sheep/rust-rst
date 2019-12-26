macro_rules! cartesian_impl {
	($out:tt [] $b:tt $init_b:tt $submacro:tt) => {
		$submacro!{$out}
	};
	($out:tt [$a:tt, $($at:tt)*] [] $init_b:tt $submacro:tt) => {
		cartesian_impl!{$out [$($at)*] $init_b $init_b $submacro}
	};
	([$($out:tt)*] [$a:tt, $($at:tt)*] [$b:tt, $($bt:tt)*] $init_b:tt $submacro:tt) => {
		cartesian_impl!{[$($out)* ($a, $b),] [$a, $($at)*] [$($bt)*] $init_b $submacro}
	};
}

macro_rules! cartesian {
	( $submacro:tt, [$($a:tt)*], [$($b:tt)*]) => {
		cartesian_impl!{[] [$($a)*,] [$($b)*,] [$($b)*,] $submacro}
	};
}


#[cfg(test)]
mod tests {
	macro_rules! print_cartesian {
		( [ $(($a1:tt, $a2:tt)),* , ] ) => {
			fn test_f(x:i64, y:i64) -> Result<(i64, i64), ()> {
				match (x, y) {
				$(
					($a1, $a2) => { Ok(($a1, $a2)) }
				)*
				_ => { Err(()) }
				}
			}
		};
	}

	#[test]
	fn print_cartesian() {
		cartesian!(print_cartesian, [1, 2, 3], [4, 5, 6]);
		assert_eq!(test_f(1, 4), Ok((1, 4)));
		assert_eq!(test_f(1, 3), Err(()));
		assert_eq!(test_f(3, 5), Ok((3, 5)));
	}
}
