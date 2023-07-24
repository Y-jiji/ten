use inkwell::execution_engine::{JitFunction, ExecutionEngine};
use smallvec::SmallVec;

pub struct Data {
    // a piece of data with size information
    dptr: *const (),
    size: usize,
}

pub enum Form {
    CPUxA4,
    CPUxA8,
    CPUxA1,
}

pub enum Patt {
}

pub struct Node {
    id: usize,
    tysize: (DataTy, SmallVec<[usize; 4]>),
    opcode: OpCode,
    oprand: SmallVec<[usize; 4]>,
}

pub enum DataTy {
    F32,
    F64,
}

// u8 here, because I don't believe the dimension will exceed 255
pub enum OpCode {
    Sum(u8),
    // a map from the dimension to dimension
    Mul(SmallVec<[u8; 4]>, SmallVec<[u8; 4]>),
    // input of given data form
    Inp(Form),
    // output of given data form
    Ret(Form),
}

#[derive(Default)]
pub struct Graph {
    node: Vec<Node>,
    form: Vec<Form>,
    patt: Vec<Patt>,
}

impl Graph {
    fn new() -> Self { Self::default() }
    fn register(&mut self, tysize: (DataTy, SmallVec<[usize; 4]>), opcode: OpCode, oprand: SmallVec<[usize; 4]>) -> usize {
        assert!(!self.node.last().is_some_and(|x| matches!(x.opcode, OpCode::Ret(_))));
        let id = self.node.len();
        self.node.push(Node {id, tysize, opcode, oprand});
        return id;
    }
}

// Question: 
// + What to do with the computation graph when nodes are spilled over different devices?
// + What is the execution plan then?
// + Did I miss some aspects in data movement that affects the execution?
pub struct BackendRouter {
}

pub struct LLVMBackend<'ctxt> {
    // a global device id, default 0
    id: usize,
    // llvm stuff
    ctxt: &'ctxt inkwell::context::Context,
    // code module
    modu: inkwell::module::Module<'ctxt>,
    // code builder
    builder: inkwell::builder::Builder<'ctxt>,
    // execution engine
    xengine: ExecutionEngine<'ctxt>,
}

impl<'ctxt> std::fmt::Debug for LLVMBackend<'ctxt> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, modu: \n{}", self.id, self.modu.to_string())
    }
}

type VFunc = unsafe extern "C" fn (&[*const ()]) -> *const ();

// data format of a tensor decide the behaviour
// * on write
// * on batch write
// * on read
// * on batch read
// * on remote read

// generate backend code from data format trait
pub trait DataForm {
    fn on_save(&self, );
    fn on_load(&self, );
    fn on_batch_save(&self, );
    fn on_batch_load(&self, );
    fn on_remot_load(&self, );
}

// access pattern of a tensor: how they are accessed by threads
// * parallelized access
// * vectorized access
// * sequential access
pub trait DataPatt {
}

// proof-of-concept: with computation node, data format and access pattern 
// I think this can be more sensible with some intermidiate step. 
impl<'ctxt> LLVMBackend<'ctxt> {
    fn jit_compile(&self, graph: Graph) -> Option<JitFunction<'ctxt, VFunc>> {
        // traversing the graph using graph api
        None
    }
    unsafe fn jit_execute(&self, f: JitFunction<'ctxt, VFunc>, args: &[*const ()]) -> *const () {
        f.call(args)
    }
}
