use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    // general registers
    pub x: [usize; 32],
    // save sstatus
    pub sstatus: Sstatus,
    // next instruction
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        // sp is x2
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let x = [0; 32];
        let mut trap_context = Self {
            x,
            sstatus,
            sepc: entry,
        };
        trap_context.set_sp(sp);
        trap_context
    }
}
