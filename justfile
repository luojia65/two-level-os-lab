target := "riscv64imac-unknown-none-elf"
mode := "debug"
m_kernel_file := "target/" + target + "/" + mode + "/machine-app-qemu"
m_bin_file := "target/" + target + "/" + mode + "/machine-kernel.bin"

objdump := "riscv64-unknown-elf-objdump"
objcopy := "rust-objcopy --binary-architecture=riscv64"
gdb := "riscv64-unknown-elf-gdb"
size := "rust-size"

build: kernel
    @{{objcopy}} {{m_kernel_file}} --strip-all -O binary {{m_bin_file}}

kernel:
    @cargo build --target={{target}}
    
qemu: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios none \
            -device loader,file={{m_bin_file}},addr=0x80000000 \

run: build qemu

asm: build
    @{{objdump}} -D {{m_kernel_file}} | less

size: build
    @{{size}} -A -x {{m_kernel_file}}

debug: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios none \
            -device loader,file={{m_bin_file}},addr=0x80000000 \
            -gdb tcp::1234 -S
            
gdb: 
    @gdb --eval-command="file {{m_kernel_file}}" --eval-command="target remote localhost:1234"
