use std::fmt::{Display, Formatter};

pub trait NumeralSystem: Display {
    fn to_dec(&self) -> i32;
    fn from_dec(dec: i32) -> Self where Self: Sized;
}


pub struct Decimal(pub i32);

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NumeralSystem for Decimal {
    fn to_dec(&self) -> i32 {self.0}
    fn from_dec(dec: i32) -> Decimal { Decimal(dec)}
}


#[derive(Debug)]
// используем числа от 0 до 8, но реально значение d-4
pub struct Symmetric9 {
    pub value: String,
}

impl Display for Symmetric9 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NumeralSystem for Symmetric9 {
    fn to_dec(&self) -> i32 {
        let mut res = 0;
        let mut weight = 1;
        for c in self.value.chars().rev() {
            let n = c.to_digit(9).expect("how") as i32 - 4;
            res += n * weight;
            weight *= 9;
        }
        res
    }

    fn from_dec(dec: i32) -> Self {
        let mut dec = dec;
        let mut res = String::new();
        if dec == 0 {
            res += "4";
        }
        while dec != 0 {
            let mut r = dec % 9;
            dec /= 9;
            if r > 4 {
                r -= 9;
                dec += 1;
            } else if r < -4 {
                r += 9;
                dec -= 1;
            }
            res += &*(r + 4).to_string();
        }
        Self { value: res.chars().rev().collect::<String>() }
    }
}

#[derive(Debug)]
pub struct Fibonacci {
    pub value: String
}
impl Fibonacci {
    fn fibs(limit: usize, is_len: bool) -> Vec<u32> {
        let mut res: Vec<u32> = vec![1, 2];
        let mut next = res[res.len()-1] + res[res.len()-2];
        while (is_len && res.len() < limit) || (!is_len && next < limit as u32) {
            res.push(next);
            next = res[res.len()-1] + res[res.len()-2];
        }
        res
    }
}

impl Display for Fibonacci {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NumeralSystem for Fibonacci {
    fn to_dec(&self) -> i32 {
        let fibs = Self::fibs(self.value.len(), true);
        let mut res = 0;
        for (i, c) in self.value.chars().rev().enumerate() {
            res += fibs[i] * c.to_digit(2).expect("how");
        }
        res as i32
    }

    fn from_dec(dec: i32) -> Self {
        if dec == 0 {
            return Self { value: "0".to_string() };
        }
        let fibs = Self::fibs(dec as usize, false);
        let mut dec = dec as u32;
        let mut res = String::new();
        let mut last_was_1 = false;
        let mut i = fibs.len() - 1;
        while dec > 0 {
            if dec >= fibs[i] && !last_was_1 {
                res += "1";
                last_was_1 = true;
                dec -= fibs[i];
                i -= 1;
            } else {
                res += "0";
                last_was_1 = false;
                i -= 1;
            }
        }
        while i > 0 {
            res += "0";
            i -= 1;
        }
        res += "0";
        Self { value: res.chars().collect::<String>()}
    }
}

#[derive(Debug)]
pub struct Factorial {
    pub value: String,
}

impl Display for Factorial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NumeralSystem for Factorial {
    fn to_dec(&self) -> i32 {
        let mut res = 0;
        let mut weight = 1;
        for (i, c) in self.value.chars().rev().enumerate() {
            weight *= i as u32 + 1;
            res += c.to_digit(i as u32 + 2).expect("how") * weight;
        }
        res as i32
    }

    fn from_dec(dec: i32) -> Self {
        if dec == 0 {
            return Self { value: "0".to_string() };
        }
        let mut res = String::new();
        let mut i = 2;
        let mut dec = dec;
        while dec > 0 {
            res += &*(dec % i).to_string();
            dec /= i;
            i += 1;
        }
        Self { value: res.chars().rev().collect::<String>() }
    }
}

#[derive(Debug)]
pub struct Neg10 {
    pub value: String,
}

impl Display for Neg10 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NumeralSystem for Neg10 {
    fn to_dec(&self) -> i32 {
        let mut res = 0;
        let mut weight = 1i32;
        for c in self.value.chars().rev() {
            let n = c.to_digit(10).expect("how") as i32;
            res += n * weight;
            weight *= -10;
        }
        res
    }

    fn from_dec(dec: i32) -> Self {
        let mut dec = dec;
        let mut res = String::new();
        if dec == 0 {
            res += "0";
        }
        while dec != 0 {
            let mut r = dec % -10;
            dec /= -10;
            if r < 0 {
                r += 10;
                dec += 1;
            }
            res += &*r.to_string();
        }
        Self { value: res.chars().rev().collect::<String>() }
    }
}