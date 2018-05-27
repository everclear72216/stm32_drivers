
shell killall JLinkGDBServer
shell JLinkGDBServer -USB -device STM32F429ZI -endian little -if SWD -speed auto -ir -LocalhostOnly -quiet&

target remote :2331

# print demangled symbols by default
set print asm-demangle on

# compatibility for SEGGER JLinkGDBServer
monitor semihosting enable
monitor semihosting IOClient 2

define n
	next
	refresh
end

define s
	step
	refresh
end

# # send captured ITM to the file itm.fifo
# # (the microcontroller SWO pin must be connected to the programmer SWO pin)
# # 8000000 must match the core clock frequency
# monitor tpiu config internal itm.fifo uart off 8000000

# # OR: make the microcontroller SWO pin output compatible with UART (8N1)
# # 2000000 is the frequency of the SWO pin
# monitor tpiu config external uart off 8000000 2000000

# # enable ITM port 0
# monitor itm port 0 on

monitor reset
monitor halt

load

monitor reset
monitor halt

break system_init

refresh
