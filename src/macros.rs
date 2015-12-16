#[macro_export]
macro_rules! map {
	() => ( Map::new() );
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = Map::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[macro_export]
macro_rules! set {
	() => ( Set::new() );
    ($( $val: expr ),*) => {{
         let mut set = Set::new();
         $( set.insert($val); )*
         set
    }}
}

#[macro_export]
macro_rules! seq {
    () => ( Seq::new() );
    ($( $val: expr ),*) => {{
         let mut seq = Seq::new();
         $( seq.push($val); )*
         seq
    }}
}

#[macro_export]
macro_rules! strseq {
    ($str: expr) => {{
         let mut seq = Seq::new();
         seq.extend($str.chars());
         seq
    }}
}

/// Implements `From<EnumT> for T` and `From<T> for EnumT` for variants of an enum.
/// Implicitly implements corresponding `Into` traits. Also implements Debug trait.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate codegen_runtime; fn main() {
/// enum A {I(Vec<i8>), U(Vec<u8>)}
/// impl_union!{ A: Vec<i8> as A::I, Vec<u8> as A::U }
/// 
/// // packing into enum type.
/// let ai = A::from(vec![-12i8]);
///
/// println!("{:?}", ai);
///
/// // unpacking - destination type must be annotated.
/// let i: Vec<i8> = ai.into();
/// # }
/// ```
#[macro_export]
macro_rules! impl_union {
    ($enum_t:ty: $( $t:ty as $v:path),*) => {
        $(
            impl From<$enum_t> for $t {
                fn from(val: $enum_t) -> $t {
                    match val {
                        $v(val) => val,
                        _ => panic!("Wrong enum variant: {:?}", val),
                    }
                }
            }
            
            impl From<$t> for $enum_t {
                fn from(val: $t) -> $enum_t {
                    $v(val)
                }
            }
        )*
        
        impl ::std::fmt::Debug for $enum_t {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				match *self {
					$(
						$v(ref val) => write!(f, "{:?}", val),
					)*		
				}
    		}
		}
    };
}

/// Declares a unit-like `pub struct`, and implements
/// `std::fmt::Display` and `std::fmt::Debug` traits.
/// Also derives `PartialEq, Eq, Hash, Clone and Copy` 
///  automatically.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate codegen_runtime; 
/// impl_quote! { Bar }
/// 
/// fn main() {
/// 	let b1 = Bar;
/// 	let b2 = Bar;
///    
/// 	println!("b1: {}", b1); // prints: "b1: <Bar>"
/// 	assert!(b1 == b2);
/// }
/// ```
#[macro_export]
macro_rules! impl_quote {
    ($qt:ident) => {
    	#[derive(PartialEq, Eq, Hash, Clone, Copy)]
    	pub struct $qt;
    	
    	impl ::std::fmt::Display for $qt {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		        write!(f, "<{}>", stringify!($qt))
	    	}
		}
    	
    	impl ::std::fmt::Debug for $qt {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		        write!(f, "<{}>", stringify!($qt))
	    	}
		}   	
    };
}

/// Implements a trivial new() function
/// for a struct.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate codegen_runtime; 
/// #[derive(PartialEq, Debug)]
///pub struct Point {
///    pub x: i32,
///    pub y: i32,
///}
///
///impl_record! { Point: x as i32, y as i32 }
///
///pub fn main() {
///	let p: Point = Point::new(1,2);
///    assert_eq!(Point{x: 1, y: 2}, p);
///}
/// ```
#[macro_export]
macro_rules! impl_record {
    ($rec:ident: $( $n:ident as $t:ty),* ) => {
    	impl $rec {
			pub fn new($( $n : $t),*) -> $rec {
			    $rec {
			        $(
			            $n: $n,
			        )*
			    }
	    	}
		}
    };
}


/// Creates a set of tuples, with the cartesian product of 
/// the input sets.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate codegen_runtime; 
/// # #[macro_use] extern crate itertools;
/// use codegen_runtime::*;
/// 
/// fn main() {
///		let s1 = set!{1,2};
///		let s2 = set!{3,4};
///		let cart = cartesian_set!(s1, s2);
/// 	let result = set!{(1,3), (1,4), (2,3), (2,4)};
/// 	assert_eq!(result, cart);
/// }
/// ```
#[macro_export]
macro_rules! cartesian_set {
    ( $($S:expr),+ ) => {
    	iproduct!($($S.iter().cloned()),*).collect::<Set<_>>()    	
    };
}
