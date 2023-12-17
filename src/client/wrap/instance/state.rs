type InvokeState = Option<Result<Vec<u8>, String>>;

pub struct State {
    pub method: Vec<u8>,
    pub args: Vec<u8>,
    pub env: Vec<u8>,
    pub invoke: InvokeState,
    pub subinvoke: InvokeState,
    // pub invoker: Arc<dyn Invoker>,
    pub get_implementations_result: Option<Vec<u8>>,
    pub subinvoke_implementation: InvokeState,
    pub memory: wasmer::Memory,
}

impl State {
    pub fn new(memory: wasmer::Memory) -> Self {
        Self {
            method: Vec::new(),
            args: Vec::new(),
            env: Vec::new(),
            invoke: None,
            subinvoke: None,
            get_implementations_result: None,
            subinvoke_implementation: None,
            memory,
        }
    }

    pub fn init(&mut self, method: Vec<u8>, args: Vec<u8>) {
        self.method = method;
        self.args = args;
        self.invoke = None;
        self.subinvoke = None;
        self.get_implementations_result = None;
        self.subinvoke_implementation = None;
    }
}
