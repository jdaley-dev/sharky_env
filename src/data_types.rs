use derive_more::From;
use derive_more::TryInto;

use parking_lot::RwLock;
use std::sync::Arc;

use crate::ffi_collections::CVec;

pub type SharkySynced<T> = Arc<RwLock<T>>;

pub trait SharkyValue {}

pub type SharkyHeapFrameIndex = usize;
pub type SharkyHeapCellIndex = usize;
pub type SharkyBytePoolIndex = usize;
pub type SharkyMax = usize;
pub type SharkyInt = i64;
pub type SharkyReal = f64;
pub type SharkyByte = u8;
pub type SharkyBool = bool;
pub type SharkyByteString = CVec<SharkyByte>;

impl SharkyValue for SharkyMax {}
impl SharkyValue for SharkyInt {}
impl SharkyValue for SharkyReal {}
impl SharkyValue for SharkyByte {}
impl SharkyValue for SharkyBool {}
impl SharkyValue for SharkyByteString {}

// TODO: PartialEq might not be... appropriate. The ByteString comparison might be wrong.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, From, TryInto)]
#[repr(C, u8)]
pub enum SharkyDataType {
    #[default]
    Nil,
    Max(SharkyMax),
    Int(SharkyInt),
    Real(SharkyReal),
    Byte(SharkyByte),
    Bool(SharkyBool),
    #[from(ignore)]
    #[try_into(ignore)]
    HeapReference(SharkyHeapFrameIndex),
    ByteString(SharkyByteString),
}

impl std::fmt::Display for SharkyDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SharkyDataType::Max(v) => write!(f, "Max({})", v),
            SharkyDataType::Int(v) => write!(f, "Int({})", v),
            SharkyDataType::Real(v) => write!(f, "Real({})", v),
            SharkyDataType::Byte(v) => write!(f, "Byte({})", v),
            SharkyDataType::Bool(v) => write!(f, "Bool({})", v),
            SharkyDataType::HeapReference(v) => write!(f, "Ref({})", v),
            SharkyDataType::ByteString(v) => {
                let mut formatted = String::new();
                let vec = v.get_operator();
                for val in vec.iter() {
                    formatted.push(*val as char);
                }
                write!(f, "")
            }
            SharkyDataType::Nil => write!(f, "nil"),
        }
    }
}

impl SharkyDataType {
    pub fn to_bool(&self) -> Option<SharkyDataType> {
        match *self {
            SharkyDataType::Nil => Some(SharkyDataType::Bool(false)),
            SharkyDataType::Max(v) => Some(SharkyDataType::Bool(v != 0)),
            SharkyDataType::Int(v) => Some(SharkyDataType::Bool(v != 0)),
            SharkyDataType::Byte(v) => Some(SharkyDataType::Bool(v != 0)),
            _ => None,
        }
    }

    pub fn to_max(&self) -> Option<SharkyDataType> {
        match *self {
            SharkyDataType::Nil => Some(SharkyDataType::Max(0)),
            SharkyDataType::Bool(v) => Some(SharkyDataType::Max(if v { 1 } else { 0 })),
            SharkyDataType::Int(v) => Some(SharkyDataType::Max(v as SharkyMax)),
            SharkyDataType::Byte(v) => Some(SharkyDataType::Max(v as SharkyMax)),
            SharkyDataType::Real(v) => Some(SharkyDataType::Max(v.trunc() as SharkyMax)),
            _ => None,
        }
    }

    pub fn to_int(&self) -> Option<SharkyDataType> {
        match *self {
            SharkyDataType::Nil => Some(SharkyDataType::Int(0)),
            SharkyDataType::Bool(v) => Some(SharkyDataType::Int(if v { 1 } else { 0 })),
            SharkyDataType::Max(v) => Some(SharkyDataType::Int(v as SharkyInt)),
            SharkyDataType::Byte(v) => Some(SharkyDataType::Int(v as SharkyInt)),
            SharkyDataType::Real(v) => Some(SharkyDataType::Int(v.trunc() as SharkyInt)),
            _ => None,
        }
    }

    pub fn to_byte(&self) -> Option<SharkyDataType> {
        match *self {
            SharkyDataType::Nil => Some(SharkyDataType::Byte(0)),
            SharkyDataType::Bool(v) => Some(SharkyDataType::Byte(if v { 1 } else { 0 })),
            SharkyDataType::Int(v) => Some(SharkyDataType::Byte(v as SharkyByte)),
            SharkyDataType::Max(v) => Some(SharkyDataType::Byte(v as SharkyByte)),
            SharkyDataType::Real(v) => Some(SharkyDataType::Byte(v.trunc() as SharkyByte)),
            _ => None,
        }
    }

    pub fn to_real(&self) -> Option<SharkyDataType> {
        match *self {
            SharkyDataType::Max(v) => Some(SharkyDataType::Real(v as SharkyReal)),
            SharkyDataType::Int(v) => Some(SharkyDataType::Real(v as SharkyReal)),
            SharkyDataType::Byte(v) => Some(SharkyDataType::Real(v as SharkyReal)),
            _ => None,
        }
    }

    pub fn to_heap_reference(&self) -> Option<SharkyDataType> {
        match *self {
            SharkyDataType::Max(v) => {
                Some(SharkyDataType::HeapReference(v as SharkyHeapFrameIndex))
            }
            SharkyDataType::Int(v) => {
                Some(SharkyDataType::HeapReference(v as SharkyHeapFrameIndex))
            }
            SharkyDataType::Byte(v) => {
                Some(SharkyDataType::HeapReference(v as SharkyHeapFrameIndex))
            }
            _ => None,
        }
    }

    pub fn is_numerical(&self) -> bool {
        match *self {
            Self::Max(_) => true,
            Self::Int(_) => true,
            Self::Byte(_) => true,
            Self::Real(_) => true,
            _ => false,
        }
    }

    pub fn is_zero(&self) -> Option<bool> {
        match *self {
            Self::Max(v) => Some(v == 0),
            Self::Int(v) => Some(v == 0),
            Self::Byte(v) => Some(v == 0),
            Self::Real(v) => Some(v == 0.0),
            _ => None,
        }
    }

    pub fn try_add(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l + r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l + r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l + r)),
            (Self::Real(l), Self::Real(r)) => Some(Self::Real(l + r)),
            _ => None,
        }
    }

    pub fn try_subtract(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l - r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l - r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l - r)),
            (Self::Real(l), Self::Real(r)) => Some(Self::Real(l - r)),
            _ => None,
        }
    }

    pub fn try_multiply(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l * r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l * r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l * r)),
            (Self::Real(l), Self::Real(r)) => Some(Self::Real(l * r)),
            _ => None,
        }
    }

    pub fn try_divide(lhs: Self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() == Some(true) {
            return None;
        }
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l / r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l / r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l / r)),
            (Self::Real(l), Self::Real(r)) => Some(Self::Real(l / r)),
            _ => None,
        }
    }

    pub fn try_mod(lhs: Self, rhs: Self) -> Option<Self> {
        if let Some(is_zero) = rhs.is_zero() {
            if is_zero {
                return None;
            }
        }
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l % r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l % r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l % r)),
            (Self::Real(l), Self::Real(r)) => Some(Self::Real(l % r)),
            _ => None,
        }
    }

    pub fn try_shift_left(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l << r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l << r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l << r)),
            _ => None,
        }
    }

    pub fn try_shift_right(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l >> r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l >> r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l >> r)),
            _ => None,
        }
    }

    pub fn try_bitwise_and(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l & r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l & r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l & r)),
            _ => None,
        }
    }

    pub fn try_bitwise_or(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l | r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l | r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l | r)),
            _ => None,
        }
    }

    pub fn try_bitwise_xor(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Max(l ^ r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Int(l ^ r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Byte(l ^ r)),
            _ => None,
        }
    }

    pub fn try_bitwise_not(val: Self) -> Option<Self> {
        match val {
            Self::Max(l) => Some(Self::Max(!l)),
            Self::Int(l) => Some(Self::Int(!l)),
            Self::Byte(l) => Some(Self::Byte(!l)),
            _ => None,
        }
    }

    pub fn try_equals(lhs: Self, rhs: Self) -> Option<Self> {
        if let Some(is_zero) = rhs.is_zero() {
            if is_zero {
                return None;
            }
        }
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Bool(l == r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Bool(l == r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Bool(l == r)),
            (Self::Real(l), Self::Real(r)) => Some(Self::Bool(l == r)),
            (Self::Bool(l), Self::Bool(r)) => Some(Self::Bool(l == r)),
            (Self::Nil, Self::Nil) => Some(Self::Bool(true)),
            (Self::HeapReference(l), Self::HeapReference(r)) => Some(Self::Bool(l == r)),
            (Self::ByteString(l), Self::ByteString(r)) => {
                let l_vec = l.get_operator();
                let r_vec = r.get_operator();
                Some(Self::Bool(l_vec.as_slice() == r_vec.as_slice()))
            }
            _ => None,
        }
    }

    pub fn try_greater_than(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Bool(l > r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Bool(l > r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Bool(l > r)),
            (Self::Real(l), Self::Real(r)) => Some(Self::Bool(l > r)),
            _ => None,
        }
    }

    pub fn try_less_than(lhs: Self, rhs: Self) -> Option<Self> {
        match (lhs, rhs) {
            (Self::Max(l), Self::Max(r)) => Some(Self::Bool(l < r)),
            (Self::Int(l), Self::Int(r)) => Some(Self::Bool(l < r)),
            (Self::Byte(l), Self::Byte(r)) => Some(Self::Bool(l < r)),
            (Self::Real(l), Self::Real(r)) => Some(Self::Bool(l < r)),
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum SharkyStackMode {
    #[default]
    Indexed,
    Operative,
    Parameter,
    Transitional,
}
