use std::ffi::CStr;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct StringView {
	data: VecDeque<char>,
}

// TODOOO: Finish writing tests for everything here, and fix the broken one.
impl StringView {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn from_str(other: &str) -> Self {
        let mut data: VecDeque<char> = VecDeque::new();
        for c in other.chars() {
            data.push_back(c);
        }
        Self {
            data: data,
        }
    }

    pub fn to_string(&self) -> String {
        self.data.iter().collect()
    }

	pub unsafe fn from_raw_parts(data: *const char, len: usize) -> Self {
		unsafe {
			let mut out: VecDeque<char> = VecDeque::new();
			for i in 0..len {
				out.push_back(*(data.add(i)));
			}
			Self {
				data: out,
			}
		}
	}

	pub unsafe fn from_cstr(data: &CStr) -> Self {
		let tmp = data.to_string_lossy();
		let data: VecDeque<char> = tmp.to_owned().chars().collect();
		Self {
			data: data,
		}
	}

	pub fn trim_left(&mut self) -> Self {
		let mut out: Self = self.clone();
		while out.data.len() > 0 && 
			(out.data[0] == ' '  || 
			 out.data[0] == '\t' || 
			 out.data[0] == '\r' || 
			 out.data[0] == '\n') {
				let _ = out.data.pop_front();
		}
		out
	}

	pub fn trim_right(&mut self) -> Self {
		let mut out: Self = self.clone();
		while out.data.len() > 0 && 
			(out.data[out.data.len() - 1] == ' ' || 
			 out.data[out.data.len() - 1] == '\t' || 
			 out.data[out.data.len() - 1] == '\r' || 
			 out.data[out.data.len() - 1] == '\n') {
				let _ = out.data.pop_back();
			}
		out
	}

	pub fn trim(&mut self) -> Self {
		self.trim_left().trim_right()
	}

	pub fn take_left_while(&mut self, _predicate: fn (data: char) -> bool) 
		-> Self {
		let mut out: Self = Self::new();
		let mut i: usize = 0;
		let mut t: bool = false;
		while i < self.data.len() {
			let c: char = self.data[i];
			let t2 = _predicate(c);
			if !t && t2 {
				t = true;
				out.data.push_back(c);
			}
			if !t2 && t {
				break;
			}
			i += 1;
		}
		out
	}

	pub fn chop_by_delim(&mut self, delim: char) -> Self {
		let mut i: usize = 0;
		while i < self.data.len() && self.data[i] != delim {
			i += 1;
		}

		let mut out: Self = Self::new();
		assert!(i <= self.data.len());
		while i > 0 {
			out.data.push_back(self.data.pop_front()
				.expect("Not enough data"));
			i -= 1;
		}
		let _discarded_delim = self.data.pop_front();
		out
	}

	pub fn chop_by_sv(&mut self, other: &Self) -> Self {
		assert!("chop_by_sv is broken, and needs fixed. Right now, it only spits out empty `StringView`s" == "");
		assert!(other.data.len() <= self.data.len());
		let mut out: Self = Self::new();
		let mut i: usize = 0;
		let mut j: usize = 0;
		while i < self.data.len() {
			if self.data[i + j] == other.data[j] {
				j += 1;
			} else {
				j = 0;
			}
			if j >= other.data.len() {
				// string matches. pop everything before match and place it in
				// `out`, pop the match, then return out
				for _ in 0..i {
					out.data.push_back(self.data.pop_front()
						.expect("Not enough data"));
				}
				for _ in 0..other.data.len() {
					let _ = self.data.pop_front();
				}
				return out;
			}
			i += 1;
		}
		out
	}

	pub fn try_chop_by_delim(&mut self, delim: char) -> Option<Self> {
		let mut i = 0;
		let mut out: Self = Self::new();
		while i < self.data.len() {
			if self.data[i] == delim {
				// we found a match. Pop everything before `i` into `out`
				// then pop `i`, and return true.
				for _ in 0..i {
					out.data.push_back(self.data.pop_front()
						.expect("Not enough data"));
				}
				let _ = self.data.pop_front();
				return Some(out);
			}
			i += 1;
		}
		None
	}

	pub fn chop_left(&mut self, n: usize) -> Self {
		let mut out: Self = Self::new();
		assert!(n <= self.data.len());
		for _ in 0..n {
			out.data.push_back(self.data.pop_front()
				.expect("Not enough data"));
		}
		out
	}

	pub fn chop_right(&mut self, n: usize) -> Self {
		let mut out: Self = Self::new();
		assert!(n <= self.data.len());
		for _ in 0..n {
			out.data.push_front(self.data.pop_back()
				.expect("Not enough data"));
		}
		out
	}

	pub fn sv_chop_left_while(&mut self, _fun: fn (x: char) -> bool) -> Self {
		let mut i: usize = 0;
		assert!(self.data.len() > 0);
		let mut x: char = self.data[0];
		while i < self.data.len() && _fun(x) {
			i += 1;
			x = self.data[i];
		}
		self.chop_left(i)
	}

	pub fn index_of(&self, c: char) -> Option<usize> {
		let mut i = 0;
		while i < self.data.len() {
			if self.data[i] == c {
				return Some(i);
			}
			i += 1;
		}
		None
	}

	pub fn eq(&self, other: &Self) -> bool {
		if self.data.len() != other.data.len() {
			return false;
		} else {
			for (i, c) in self.data.iter().enumerate() {
				if other.data[i] != *c {
					return false;
				}
			}
			return true;
		}
	}

	pub fn eq_ignorecase(&self, other: &Self) -> bool {
		if self.data.len() != other.data.len() {
			return false;
		} else {
			for (i, c) in self.data.iter().enumerate() {
				let c = c.to_ascii_lowercase();
				if other.data[i].to_ascii_lowercase() != c {
					return false;
				}
			}
			return true;
		}
	}

	pub fn starts_with(&self, prefix: Self) -> bool {
		if self.data.len() < prefix.data.len() {
			return false;
		}
		for i in 0..prefix.data.len() {
			if self.data[i] != prefix.data[i] {
				return false;
			}
		}
		return true;
	}

	pub fn ends_with(&self, suffix: Self) -> bool {
		if self.data.len() < suffix.data.len() {
			return false;
		}
		for i in 0..suffix.data.len() {
			let self_offset = self.data.len() - suffix.data.len() + i;
			if self.data[self_offset] != suffix.data[i] {
				return false;
			}
		}
		return true;
	}

	pub fn to_u64(&self) -> u64 {
		let tmp_str = self.to_string();
		u64::from_str_radix(&tmp_str, 10)
			.expect("Attempted to convert a string to a number")
	}

	pub fn chop_u64(&mut self) -> Option<u64> {
		let mut out_str: Self = Self::new();
		for i in 0..self.data.len() {
			if !self.data[i].is_ascii_digit() {
				break;
			}
			out_str.data.push_back(self.data[i]);
		}
		if out_str.data.len() == 0 {
			return None;
		}
		let tmp = out_str.to_string();
		return Some(u64::from_str_radix(&tmp, 10)
			.expect("Attempted to convert a string to a number."));
	}
}
#[cfg(test)]
mod tests {
	use super::StringView;

	#[test]
	fn working_test() {
		assert!(true);
	}

	#[test]
	fn test_string_stuff() {
		let s = "You have lost The Game".to_owned();
		let sv = StringView::from_str(&s);
		assert_eq!(s, sv.to_string());
	}
	
	#[test]
	fn test_trim() {
		let mut sv = StringView::from_str("\r\n\t You lost the game \t\n\r");
		let sv2 = sv.trim();
		let sv3 = StringView::from_str("You lost the game");
		assert_eq!(sv2, sv3);
	}

	#[test]
	fn test_chop_by_delim() {
		let mut sv = StringView::from_str("\r\n\t You lost the game \t\n\r");
		let chop = sv.chop_by_delim(' ');
		let chop2 = sv.chop_by_delim(' ');
		let cmp = StringView::from_str("You");
		assert_eq!(chop2, cmp);
	}

	// FIXME StringView::chop_by_sv currently does not work. it?
    /*
	#[test]
	fn test_chop_by_sv() {
		let mut sv = StringView::from_str(",.,.you,.lost,.the,.\n,game");
		let delim = StringView::from_str(",.");
		let _first = sv.chop_by_sv(&delim);
		let _second = sv.chop_by_sv(&delim);
		let _third = sv.chop_by_sv(&delim); 
		let cmp = StringView::from_str("you");
		dbg!(&_first, &_second, &_third, &cmp);
		assert_eq!(_third, cmp);
	}
    */
}
