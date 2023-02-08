use super::Device;
use crate::TrapCause;
use std::collections::VecDeque;

#[allow(non_camel_case_types)]
enum UartRegister {
    RX_TX,   // read: RX, write: TX
    IER,     // write: IER
    IIR_FCR, // read: IIR, write: FCR
    LCR,     // write: LCR
    MCR,     // write: MCR
    LSR,     // read: LSR
    MSR,     // read: MSR
    SCR,     // I/O: SCR
}

pub struct Uart {
    pub uart: Vec<u8>,
    rx_queue: VecDeque<u8>,
    pub base_addr: u64,
    size: usize,
}

impl Default for Uart {
    fn default() -> Self {
        Self::new()
    }
}

impl Uart {
    #[allow(arithmetic_overflow)]
    pub fn new() -> Self {
        const UART_SIZE: usize = 0x100;
        let mut uart = vec![0; UART_SIZE];
        uart[UartRegister::IIR_FCR as usize] = 0x1; // IIR_NO_INT
        uart[UartRegister::LSR as usize] = 0x60; // LSR_TEMT | LSR_THRE
        uart[UartRegister::MSR as usize] = 0xb0; // UART_MSR_DCD | UART_MSR_DSR | UART_MSR_CTS
        uart[UartRegister::MCR as usize] = 0x08; // MCR_OUT2

        Uart {
            uart,
            rx_queue: VecDeque::new(),
            base_addr: 0x1000_0000,
            size: UART_SIZE,
        }
    }
}

#[allow(clippy::identity_op)]
impl Device for Uart {
    // is addr in device address space
    fn in_range(&self, addr: u64) -> bool {
        (self.base_addr..=self.base_addr + self.size as u64).contains(&addr)
    }

    // address to raw index
    fn addr2index(&self, addr: u64) -> usize {
        (addr - self.base_addr) as usize
    }

    // store
    fn store8(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        self.uart[index] = (data & 0xFF) as u8;
        Ok(())
    }

    fn store16(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "uart only allows load/store8 but try store16".to_string(),
        ))
    }

    fn store32(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "uart only allows load/store8 but try store32".to_string(),
        ))
    }

    fn store64(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "uart only allows load/store8 but try store64".to_string(),
        ))
    }

    // load
    fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        const RX: usize = UartRegister::RX_TX as usize;
        const LSR_DR: u8 = 0x01;
        let index = self.addr2index(addr);
        match index {
            RX => {
                self.uart[UartRegister::LSR as usize] &= !LSR_DR;
                Ok(self.uart[UartRegister::RX_TX as usize] as u64)
            }
            _ => Ok(self.uart[index] as i8 as i64 as u64),
        }
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load16".to_string(),
        ))
    }

    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load32".to_string(),
        ))
    }

    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load64".to_string(),
        ))
    }

    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok(self.uart[index] as u64)
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load_u16".to_string(),
        ))
    }

    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load_u32".to_string(),
        ))
    }
}
