pub const CSR_RDCYCLE: u32 = 0xc00;
pub const CSR_RDTIME: u32 = 0xc01;
pub const CSR_RDINSTRET: u32 = 0xc02;

pub const CSR_RDCYCLEH: u32 = 0xc80;
pub const CSR_RDTIMEH: u32 = 0xc81;
pub const CSR_RDINSTRETH: u32 = 0xc82;

pub const CSR_SATP: u32 = 0x180;

pub const CSR_MSTATUS: u32 = 0x300;
pub const CSR_MISA: u32 = 0x301;
pub const CSR_MIE: u32 = 0x304;
pub const CSR_MTVEC: u32 = 0x305;
pub const CSR_MSCRATCH: u32 = 0x340;
pub const CSR_MEPC: u32 = 0x341;
pub const CSR_MCAUSE: u32 = 0x342;
pub const CSR_MTVAL: u32 = 0x343;
pub const CSR_MIP: u32 = 0x344;

pub const CSR_VENDORID: u32 = 0xf11;
