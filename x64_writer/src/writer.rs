use super::{
    args::{Arg, ArgSize},
    label::Label,
};
use std::io::{self, Write};

pub struct AsmWriter<O> {
    out: O,
}
impl<O: Write> AsmWriter<O> {
    pub fn new(out: O) -> Self {
        Self { out }
    }

    pub fn write_filename(&mut self, name: &str) -> io::Result<()> {
        writeln!(self.out, "\t.file \"{name}\"")
    }
    pub fn emit_label<'a>(&mut self, label: impl Into<Label<'a>>) -> io::Result<()> {
        let label = label.into();
        writeln!(self.out, "{}:", label)
    }
    pub fn declare_global<'a>(&mut self, label: impl Into<Label<'a>>) -> io::Result<()> {
        let label = label.into().label;
        writeln!(self.out, "\t.global {label}")
    }
    pub fn begin_text(&mut self) -> io::Result<()> {
        writeln!(self.out, "\t.text")
    }
    pub fn empty_line(&mut self) -> io::Result<()> {
        writeln!(self.out)
    }
    pub fn comment(&mut self, comment: impl AsRef<str>) -> io::Result<()> {
        let comment = comment.as_ref();
        writeln!(self.out, "\t# {comment}")
    }

    pub fn build_mov<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        let dst = dst.into();
        let src = src.into();
        let suffix = get_size(&dst, &src).suffix();
        writeln!(self.out, "\tmov{suffix} {src}, {dst}")
    }
    pub fn build_cmov<'a>(
        &mut self,
        c: Condition,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        let dst = dst.into();
        let src = src.into();
        let suffix = c.suffix();
        writeln!(self.out, "\tcmov{suffix} {src}, {dst}")
    }
    pub fn build_push<'a>(&mut self, src: impl Into<Arg<'a>>) -> io::Result<()> {
        let src: Arg = src.into();
        let suffix = src.size().unwrap().suffix();
        writeln!(self.out, "\tpush{suffix} {src}")
    }
    pub fn build_pop<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        let dst: Arg = dst.into();
        let suffix = dst.size().unwrap().suffix();
        writeln!(self.out, "\tpop{suffix} {dst}")
    }

    pub fn build_binary_op<'a>(
        &mut self,
        op: impl Into<BinaryOp>,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        let op = op.into();
        match op {
            BinaryOp::Single(s) => self.build_binary_op_single(s, dst, src),
        }
    }
    fn build_binary_op_single<'a>(
        &mut self,
        op: BinaryOpSingle,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        let dst = dst.into();
        let src = src.into();
        let suffix = get_size(&dst, &src).suffix();
        let mnemonic = op.mnemonic();
        writeln!(self.out, "\t{mnemonic}{suffix} {src}, {dst}")
    }

    pub fn build_add<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Add, dst, src)
    }
    pub fn build_sub<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Sub, dst, src)
    }
    pub fn build_imul<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::IMul, dst, src)
    }
    pub fn build_and<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::And, dst, src)
    }
    pub fn build_or<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Or, dst, src)
    }
    pub fn build_xor<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Xor, dst, src)
    }
    pub fn build_lea<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Lea, dst, src)
    }
    pub fn build_cmp<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Cmp, dst, src)
    }
    pub fn build_test<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Test, dst, src)
    }
    pub fn build_shl<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Shl, dst, src)
    }
    pub fn build_shr<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Shr, dst, src)
    }
    pub fn build_sar<'a>(
        &mut self,
        dst: impl Into<Arg<'a>>,
        src: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        self.build_binary_op(BinaryOpSingle::Sar, dst, src)
    }

    pub fn build_unary_op<'a>(
        &mut self,
        op: impl Into<UnaryOp>,
        dst: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        let op = op.into();
        match op {
            UnaryOp::Single(s) => self.build_unary_op_single(s, dst),
        }
    }
    fn build_unary_op_single<'a>(
        &mut self,
        op: UnaryOpSingle,
        dst: impl Into<Arg<'a>>,
    ) -> io::Result<()> {
        let dst: Arg = dst.into();
        let suffix = dst.size().unwrap().suffix();
        let mnemonic = op.mnemonic();
        writeln!(self.out, "\t{mnemonic}{suffix} {dst}")
    }

    pub fn build_inc<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        self.build_unary_op(UnaryOpSingle::Inc, dst)
    }
    pub fn build_dec<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        self.build_unary_op(UnaryOpSingle::Dec, dst)
    }
    pub fn build_neg<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        self.build_unary_op(UnaryOpSingle::Neg, dst)
    }
    pub fn build_not<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        self.build_unary_op(UnaryOpSingle::Not, dst)
    }
    pub fn build_mul<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        self.build_unary_op(UnaryOpSingle::Mul, dst)
    }
    pub fn build_unary_imul<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        self.build_unary_op(UnaryOpSingle::IMul, dst)
    }
    pub fn build_div<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        self.build_unary_op(UnaryOpSingle::Div, dst)
    }
    pub fn build_idiv<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        self.build_unary_op(UnaryOpSingle::IDiv, dst)
    }

    pub fn build_call<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        let dst = dst.into();
        let needs_star = dst.is_memory() || dst.is_register();
        let star = if needs_star { "*" } else { "" };
        writeln!(self.out, "\tcall {star}{dst}")
    }
    pub fn build_jmp<'a>(&mut self, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        let dst = dst.into();
        let needs_star = dst.is_memory() || dst.is_register();
        let star = if needs_star { "*" } else { "" };
        writeln!(self.out, "\tjmp {star}{dst}")
    }
    pub fn build_cjmp<'a>(&mut self, c: Condition, dst: impl Into<Arg<'a>>) -> io::Result<()> {
        let dst = dst.into();
        let suffix = c.suffix();
        writeln!(self.out, "\tj{suffix} {dst}")
    }

    pub fn build_nonary_op(&mut self, op: NonaryOp) -> io::Result<()> {
        let mnemonic = op.mnemonic();
        writeln!(self.out, "\t{mnemonic}")
    }
    pub fn build_ret(&mut self) -> io::Result<()> {
        self.build_nonary_op(NonaryOp::Ret)
    }
}

pub enum BinaryOp {
    Single(BinaryOpSingle),
}
impl From<BinaryOpSingle> for BinaryOp {
    fn from(value: BinaryOpSingle) -> Self {
        Self::Single(value)
    }
}

/// Binary operations that have a single size suffix
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOpSingle {
    Add,
    Sub,
    IMul,
    And,
    Or,
    Xor,
    Lea,
    Cmp,
    Test,
    Shl,
    Shr,
    Sar,
}
impl BinaryOpSingle {
    pub fn mnemonic(self) -> &'static str {
        use BinaryOpSingle::*;
        match self {
            Add => "add",
            Sub => "sub",
            IMul => "imul",
            And => "and",
            Or => "or",
            Xor => "xor",
            Lea => "lea",
            Cmp => "cmp",
            Test => "test",
            Shl => "shl",
            Shr => "shr",
            Sar => "sar",
        }
    }
}

pub enum UnaryOp {
    Single(UnaryOpSingle),
}
impl From<UnaryOpSingle> for UnaryOp {
    fn from(value: UnaryOpSingle) -> Self {
        Self::Single(value)
    }
}

/// Unary operations that have a single size suffix
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOpSingle {
    Inc,
    Dec,
    Neg,
    Not,
    Mul,
    IMul,
    Div,
    IDiv,
}
impl UnaryOpSingle {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Inc => "inc",
            Self::Dec => "dec",
            Self::Neg => "neg",
            Self::Not => "not",
            Self::Mul => "mul",
            Self::IMul => "imul",
            Self::Div => "div",
            Self::IDiv => "idiv",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum NonaryOp {
    Ret,
}
impl NonaryOp {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Ret => "ret",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    Zero,
    NotZero,
    Equal,
    NotEqual,
    Negative,
    NonNegative,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Above,
    Below,
    AboveEqual,
    BelowEqual,
}
impl Condition {
    pub fn suffix(self) -> &'static str {
        use Condition::*;
        match self {
            Zero => "z",
            NotZero => "nz",
            Equal => "e",
            NotEqual => "ne",
            Negative => "s",
            NonNegative => "ns",
            GreaterThan => "g",
            LessThan => "l",
            GreaterEqual => "ge",
            LessEqual => "le",
            Above => "a",
            Below => "b",
            AboveEqual => "ae",
            BelowEqual => "be",
        }
    }
}

fn get_size(a: &Arg, b: &Arg) -> ArgSize {
    match (a.size(), b.size()) {
        (None, None) => panic!(),
        (Some(a), None) | (None, Some(a)) => a,
        (Some(a), Some(b)) => {
            assert_eq!(a, b);
            a
        }
    }
}
