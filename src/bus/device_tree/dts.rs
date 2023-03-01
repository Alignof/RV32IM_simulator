use crate::Isa;

fn dts_32(dram_addr: u64, initrd_start: Option<usize>, initrd_end: Option<usize>) -> String {
    let initrd_start = initrd_start.unwrap_or(0);
    let initrd_end = initrd_end.unwrap_or(0);
    format!(
        "/dts-v1/;
            / {{
              #address-cells = <2>;
              #size-cells = <2>;
              compatible = \"ucbbar,spike-bare-dev\";
              model = \"ucbbar,spike-bare\";
              chosen {{
                stdout-path = &SERIAL0;
                linux,initrd-start = <{initrd_start}>;
                linux,initrd-end = <{initrd_end}>;
                bootargs = \"root=/dev/ram console=ttyS0 earlycon\";
              }};
              cpus {{
                #address-cells = <1>;
                #size-cells = <0>;
                timebase-frequency = <10000000>;
                CPU0: cpu@0 {{
                  device_type = \"cpu\";
                  reg = <0>;
                  status = \"okay\";
                  compatible = \"riscv\";
                  riscv,isa = \"rv32imac\";
                  mmu-type = \"riscv,sv32\";
                  riscv,pmpregions = <16>;
                  riscv,pmpgranularity = <4>;
                  clock-frequency = <1000000000>;
                  CPU0_intc: interrupt-controller {{
                    #address-cells = <2>;
                    #interrupt-cells = <1>;
                    interrupt-controller;
                    compatible = \"riscv,cpu-intc\";
                  }};
                }};
              }};
              memory@{dram_addr:x} {{
                device_type = \"memory\";
                reg = <0x0 0x{dram_addr:x} 0x0 0x{dram_addr:x}>;
              }};
              soc {{
                #address-cells = <2>;
                #size-cells = <2>;
                compatible = \"ucbbar,spike-bare-soc\", \"simple-bus\";
                ranges;
                clint@2000000 {{
                  compatible = \"riscv,clint0\";
                  interrupts-extended = <&CPU0_intc 3 &CPU0_intc 7 >;
                  reg = <0x0 0x2000000 0x0 0xc0000>;
                }};
                PLIC: plic@c000000 {{
                  compatible = \"riscv,plic0\";
                  #address-cells = <2>;
                  interrupts-extended = <&CPU0_intc 11 &CPU0_intc 9 >;
                  reg = <0x0 0xc000000 0x0 0x1000000>;
                  riscv,ndev = <0x1f>;
                  riscv,max-priority = <0xf>;
                  #interrupt-cells = <1>;
                  interrupt-controller;
                }};
                SERIAL0: ns16550@10000000 {{
                  compatible = \"ns16550a\";
                  clock-frequency = <10000000>;
                  interrupt-parent = <&PLIC>;
                  interrupts = <1>;
                  reg = <0x0 0x10000000 0x0 0x100>;
                  reg-shift = <0x0>;
                  reg-io-width = <0x1>;
                }};
              }};
              htif {{
                compatible = \"ucb,htif0\";
              }};
        }};"
    )
}

fn dts_64(dram_addr: u64, initrd_start: Option<usize>, initrd_end: Option<usize>) -> String {
    let initrd_start = initrd_start.unwrap_or(0);
    let initrd_end = initrd_end.unwrap_or(0);
    format!(
        "/dts-v1/;

        / {{
          #address-cells = <2>;
          #size-cells = <2>;
          compatible = \"ucbbar,spike-bare-dev\";
          model = \"ucbbar,spike-bare\";
          chosen {{
            stdout-path = &SERIAL0;
            linux,initrd-start = <{initrd_start}>;
            linux,initrd-end = <{initrd_end}>;
            bootargs = \"root=/dev/ram console=ttyS0 earlycon\";
          }};
          cpus {{
            #address-cells = <1>;
            #size-cells = <0>;
            timebase-frequency = <10000000>;
            CPU0: cpu@0 {{
              device_type = \"cpu\";
              reg = <0>;
              status = \"okay\";
              compatible = \"riscv\";
              riscv,isa = \"rv64imac\";
              mmu-type = \"riscv,sv57\";
              riscv,pmpregions = <16>;
              riscv,pmpgranularity = <4>;
              clock-frequency = <1000000000>;
              CPU0_intc: interrupt-controller {{
                #address-cells = <2>;
                #interrupt-cells = <1>;
                interrupt-controller;
                compatible = \"riscv,cpu-intc\";
              }};
            }};
          }};
          memory@{dram_addr:x} {{
            device_type = \"memory\";
            reg = <0x0 {dram_addr:x} 0x0 0x80000000>;
          }};
          soc {{
            #address-cells = <2>;
            #size-cells = <2>;
            compatible = \"ucbbar,spike-bare-soc\", \"simple-bus\";
            ranges;
            clint@2000000 {{
              compatible = \"riscv,clint0\";
              interrupts-extended = <&CPU0_intc 3 &CPU0_intc 7 >;
              reg = <0x0 0x2000000 0x0 0xc0000>;
            }};
            PLIC: plic@c000000 {{
              compatible = \"riscv,plic0\";
              #address-cells = <2>;
              interrupts-extended = <&CPU0_intc 11 &CPU0_intc 9 >;
              reg = <0x0 0xc000000 0x0 0x1000000>;
              riscv,ndev = <0x1f>;
              riscv,max-priority = <0xf>;
              #interrupt-cells = <1>;
              interrupt-controller;
            }};
            SERIAL0: ns16550@10000000 {{
              compatible = \"ns16550a\";
              clock-frequency = <10000000>;
              interrupt-parent = <&PLIC>;
              interrupts = <1>;
              reg = <0x0 0x10000000 0x0 0x100>;
              reg-shift = <0x0>;
              reg-io-width = <0x1>;
            }};
          }};
          htif {{
            compatible = \"ucb,htif0\";
          }};
        }};"
    )
}

pub fn make_dts(
    dram_addr: u64,
    initrd_start: Option<usize>,
    initrd_end: Option<usize>,
    isa: Isa,
) -> String {
    match isa {
        Isa::Rv32 => dts_32(dram_addr, initrd_start, initrd_end),
        Isa::Rv64 => dts_64(dram_addr, initrd_start, initrd_end),
    }
}
