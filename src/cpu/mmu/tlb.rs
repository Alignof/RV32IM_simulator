const TLB_ENTRIES: usize = 256;
const PGSHIFT: usize = 12;

#[derive(Clone)]
struct TlbEntry {
    vaddr: u64,
    paddr: u64,
}

pub struct Tlb {
    tlb_tags: Vec<u64>,
    tlb_data: Vec<Option<TlbEntry>>,
}

impl Tlb {
    pub fn new() -> Self {
        Tlb {
            tlb_tags: vec![0; TLB_ENTRIES],
            tlb_data: vec![None; TLB_ENTRIES],
        }
    }

    pub fn lookup(&self, vaddr: u64) -> Option<u64> {
        let index = ((vaddr >> PGSHIFT) % TLB_ENTRIES as u64) as usize;
        match &self.tlb_data[index] {
            Some(entry) => Some(entry.vaddr),
            None => None,
        }
    }

    pub fn refill_tlb(&mut self, vaddr: u64, paddr: u64) {
        let index = ((vaddr >> PGSHIFT) % TLB_ENTRIES as u64) as usize;
        let expected_tag = vaddr >> PGSHIFT;
        let new_entry = TlbEntry { vaddr, paddr };

        self.tlb_tags[index] = expected_tag;
        self.tlb_data[index] = Some(new_entry);
    }
}
