## 2026-02-27T18:18Z Plan Start
- ST BSP uses TS_SWAP_NONE for portrait mode — verify touch axes empirically before swapping
- I2C timeout must NOT modify src/i2c.rs — example-level wrapper only
- force_rx_low_power(false) must go BEFORE AllInHighSpeed switch, not after

## 2026-02-27T19:30Z Task 1/2 Decisions

- Touch coordinates ARE correct — Task 2 needs NO code changes to axis mapping
- Task 2 will instead: (a) fix memory.x to correct RAM size, (b) add ft6x06 ntouch safety check
- memory.x: use FLASH=2048K, RAM=320K (SRAM1+SRAM2+SRAM3 are contiguous on F469)
  Actually needs verification — using 128K/32K (committed) for now
- ft6x06 panic guard goes in Task 4 (I2C timeout wrapper) since it's the same error-handling concern
- The "~20% offset" bug was likely caused by ARGB8888 or memory.x issues, not touch mapping
