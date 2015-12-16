use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Default)]
pub struct Token {
	value: String	
}

impl Token {
	pub fn new(expr: &ToString) -> Token {
		Token { value : expr.to_string() }
	}
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mk_token({})", self.value)
    }
}

impl fmt::Debug for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mk_token({})", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
    	let red1 = Token::new(&"RED".to_string());
    	let red2 = Token::new(&"RED".to_string());   	    	
        assert_eq!(red1, red2);
    }
    
    #[test]
    fn inequality() {
    	let red = Token::new(&"RED".to_string());
    	let blue = Token::new(&"BLUE".to_string());   	    	
        assert!(red != blue);
    }
    
    #[test]
    fn display_formatting() {
    	let red = Token::new(&"RED".to_string());
    	let quote_string = red.to_string();  	    	
        assert_eq!("mk_token(RED)",quote_string);
    }
}