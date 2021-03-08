use vex_sys::*;
use super::logger;

// Re-exports:
pub use vex_sys::{
    IRType as Type,
    _IRStmt__bindgen_ty_1__bindgen_ty_2 as StmtIMark,
    IREndness, // don't remove the IR, to differentiate from VexEndness.
    IRTemp as Temp,
    _IRExpr__bindgen_ty_1__bindgen_ty_2 as ExprGet,
    _IRExpr__bindgen_ty_1__bindgen_ty_4 as ExprRdTmp,
};

#[derive(Copy, Clone)]
pub enum ConstEnum {
    U1(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F32i(u32),
    F64(f64),
    F64i(u64),
    V128(vex_sys::V128),
    V256(vex_sys::V256),
}

pub struct Const<'a>(&'a mut IRConst);

impl Const<'_> {
    pub fn as_enum(self) -> ConstEnum {
        match self.0.tag {
            IRConstTag::Ico_U1 => ConstEnum::U1(unsafe { self.0.Ico.U1 != 0 }),
            IRConstTag::Ico_U8 => ConstEnum::U8(unsafe { self.0.Ico.U8 }),
            IRConstTag::Ico_U16 => ConstEnum::U16(unsafe { self.0.Ico.U16 }),
            IRConstTag::Ico_U32 => ConstEnum::U32(unsafe { self.0.Ico.U32 }),
            IRConstTag::Ico_U64 => ConstEnum::U64(unsafe { self.0.Ico.U64 }),
            IRConstTag::Ico_F32 => ConstEnum::F32(unsafe { self.0.Ico.F32 }),
            IRConstTag::Ico_F32i => ConstEnum::F32i(unsafe { self.0.Ico.F32i }),
            IRConstTag::Ico_F64 => ConstEnum::F64(unsafe { self.0.Ico.F64 }),
            IRConstTag::Ico_F64i => ConstEnum::F64i(unsafe { self.0.Ico.F64i }),
            IRConstTag::Ico_V128 => {
                let val = unsafe { self.0.Ico.V128 };
                let vec = vex_sys::V128 {
                    w16: [val, val, val, val, val, val, val, val]
                };
                ConstEnum::V128(vec)
            }
            IRConstTag::Ico_V256 => {
                let val = unsafe { self.0.Ico.V256 };
                let vec = vex_sys::V256 {
                    w32: [val, val, val, val, val, val, val, val]
                };
                ConstEnum::V256(vec)
            }
        }
    }
}

pub struct Callee<'a>(&'a mut IRCallee);

pub struct RegArray<'a>(&'a mut IRRegArray);

pub enum ExprEnum<'a> {
    Get(ExprGet),
    GetI(ExprGetI<'a>),
    RdTmp(ExprRdTmp),
    Qop(ExprQop<'a>),
    Triop(ExprTriop<'a>),
    Binop(ExprBinop<'a>),
    Unop(ExprUnop<'a>),
    Load(ExprLoad<'a>),
    Const(Const<'a>),
    CCall(ExprCCall<'a>),
    ITE(ExprITE<'a>),
}

pub struct ExprGetI<'a>(&'a mut _IRExpr__bindgen_ty_1__bindgen_ty_3);

pub struct ExprQop<'a>(&'a mut _IRExpr__bindgen_ty_1__bindgen_ty_5);

pub struct ExprTriop<'a>(&'a mut IRTriop);

pub struct ExprBinop<'a>(&'a mut _IRExpr__bindgen_ty_1__bindgen_ty_7);

pub struct ExprUnop<'a>(&'a mut _IRExpr__bindgen_ty_1__bindgen_ty_8);

pub struct ExprLoad<'a>(&'a mut _IRExpr__bindgen_ty_1__bindgen_ty_9);

pub struct ExprCCall<'a>(&'a mut _IRExpr__bindgen_ty_1__bindgen_ty_11);

pub struct ExprITE<'a>(&'a mut _IRExpr__bindgen_ty_1__bindgen_ty_12);

pub struct Expr<'a>(&'a mut IRExpr);

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        logger::with(|| unsafe { ppIRExpr(self.0) }).1.unwrap().fmt(f)
    }
}

impl Expr<'_> {
    pub fn as_enum(&mut self) -> ExprEnum {
        match self.0.tag {
            IRExprTag::Iex_Binder => unreachable!("Binder should never be returned by VEX"),
            IRExprTag::Iex_Get => ExprEnum::Get(unsafe { self.0.Iex.Get }),
            IRExprTag::Iex_GetI => ExprEnum::GetI(ExprGetI(unsafe { &mut self.0.Iex.GetI })),
            IRExprTag::Iex_RdTmp => ExprEnum::RdTmp(unsafe { self.0.Iex.RdTmp }),
            IRExprTag::Iex_Qop => ExprEnum::Qop(ExprQop(unsafe { &mut self.0.Iex.Qop })),
            IRExprTag::Iex_Triop => ExprEnum::Triop(ExprTriop(unsafe { &mut *self.0.Iex.Triop.details })),
            IRExprTag::Iex_Binop => ExprEnum::Binop(ExprBinop(unsafe { &mut self.0.Iex.Binop })),
            IRExprTag::Iex_Unop => ExprEnum::Unop(ExprUnop(unsafe { &mut self.0.Iex.Unop })),
            IRExprTag::Iex_Load => ExprEnum::Load(ExprLoad(unsafe { &mut self.0.Iex.Load })),
            IRExprTag::Iex_Const => ExprEnum::Const(Const(unsafe { &mut *self.0.Iex.Const.con })),
            IRExprTag::Iex_ITE => ExprEnum::ITE(ExprITE(unsafe { &mut self.0.Iex.ITE })),
            IRExprTag::Iex_CCall => ExprEnum::CCall(ExprCCall(unsafe { &mut self.0.Iex.CCall })),
            IRExprTag::Iex_VECRET => unreachable!("VECRET should never be returned by VEX"),
            IRExprTag::Iex_GSPTR => unreachable!("GSPTR should never be returned by VEX"),
        }
    }
}

pub struct Stmt<'a>(pub(crate) &'a mut IRStmt);

pub enum StmtEnum<'a> {
    NoOp,
    IMark(StmtIMark),
    AbiHint(StmtAbiHint<'a>),
    Put(StmtPut<'a>),
    PutI(StmtPutI<'a>),
    WrTmp(StmtWrTmp<'a>),
    Store(StmtStore<'a>),
    LoadG(StmtLoadG<'a>),
    StoreG(StmtStoreG<'a>),
    CAS(StmtCAS<'a>),
    LLSC(StmtLLSC<'a>),
    Dirty(StmtDirty<'a>),
    MBE(StmtMBE<'a>),
    Exit(StmtExit<'a>),
}

impl std::fmt::Display for Stmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        logger::with(|| unsafe { ppIRStmt(self.0) }).1.unwrap().fmt(f)
    }
}

impl Stmt<'_> {
    pub fn kind(&self) -> IRStmtTag {
        self.0.tag
    }

    pub fn as_enum(&mut self) -> StmtEnum {
        match self.0.tag {
            IRStmtTag::Ist_NoOp => StmtEnum::NoOp,
            IRStmtTag::Ist_IMark => StmtEnum::IMark(*unsafe { self.0.Ist.IMark.as_ref() }),
            IRStmtTag::Ist_AbiHint => StmtEnum::AbiHint(StmtAbiHint(unsafe { self.0.Ist.AbiHint.as_mut() })),
            IRStmtTag::Ist_Put => StmtEnum::Put(StmtPut(unsafe { self.0.Ist.Put.as_mut() })),
            IRStmtTag::Ist_PutI => StmtEnum::PutI(StmtPutI(unsafe { self.0.Ist.PutI.as_mut() })),
            IRStmtTag::Ist_WrTmp => StmtEnum::WrTmp(StmtWrTmp(unsafe { self.0.Ist.WrTmp.as_mut() })),
            IRStmtTag::Ist_Store => StmtEnum::Store(StmtStore(unsafe { self.0.Ist.Store.as_mut() })),
            IRStmtTag::Ist_LoadG => StmtEnum::LoadG(StmtLoadG(unsafe { self.0.Ist.LoadG.as_mut() })),
            IRStmtTag::Ist_StoreG => StmtEnum::StoreG(StmtStoreG(unsafe { self.0.Ist.StoreG.as_mut() })),
            IRStmtTag::Ist_CAS => StmtEnum::CAS(StmtCAS(unsafe { self.0.Ist.CAS.as_mut() })),
            IRStmtTag::Ist_LLSC => StmtEnum::LLSC(StmtLLSC(unsafe { self.0.Ist.LLSC.as_mut() })),
            IRStmtTag::Ist_Dirty => StmtEnum::Dirty(StmtDirty(unsafe { self.0.Ist.Dirty.as_mut() })),
            IRStmtTag::Ist_MBE => StmtEnum::MBE(StmtMBE(unsafe { self.0.Ist.MBE.as_mut() })),
            IRStmtTag::Ist_Exit => StmtEnum::Exit(StmtExit(unsafe { self.0.Ist.Exit.as_mut() })),
        }
    }
}

pub struct StmtAbiHint<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_3);

pub struct StmtPut<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_4);

impl StmtPut<'_> {
    pub fn offset(&self) -> i32 {
        self.0.offset
    }

    pub fn data(&self) -> Expr {
        Expr(unsafe { &mut *self.0.data })
    }
}

pub struct StmtPutI<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_5);

pub struct StmtWrTmp<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_6);

pub struct StmtStore<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_7);

pub struct StmtStoreG<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_8);

pub struct StmtLoadG<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_9);

pub struct StmtCAS<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_10);

pub struct StmtLLSC<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_11);

pub struct StmtDirty<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_12);

pub struct StmtMBE<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_13);

pub struct StmtExit<'a>(pub(crate) &'a mut _IRStmt__bindgen_ty_1__bindgen_ty_14);

pub struct TypeEnv<'a>(&'a mut IRTypeEnv);

impl std::fmt::Display for TypeEnv<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        logger::with(|| unsafe { ppIRTypeEnv(self.0) }).1.unwrap().fmt(f)
    }
}

pub struct IRSB<'a> {
    pub(crate) inner: &'a vex_sys::IRSB,
    pub(crate) _lock: super::LiftGuard<'a>,
}

impl std::fmt::Display for IRSB<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        logger::with(|| unsafe { ppIRSB(self.inner) }).1.unwrap().fmt(f)
    }
}

impl<'a> IRSB<'a> {
    pub fn type_env(&self) -> TypeEnv {
        TypeEnv(unsafe { &mut *self.inner.tyenv })
    }

    pub fn stmts(&'a self) -> impl Iterator<Item=Stmt<'a>> + 'a {
        unsafe {
            std::slice::from_raw_parts(
                (*self.inner).stmts,
                (*self.inner).stmts_used as usize,
            )
                .iter()
            .map(|s| Stmt(&mut **s))
        }
    }

    pub fn next(&self) -> Expr {
        Expr(unsafe { &mut *self.inner.next })
    }
}
