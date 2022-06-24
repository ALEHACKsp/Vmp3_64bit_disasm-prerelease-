# Vmp3_64bit_disasm-prerelease-

Currently does not disassemble any kind of branching, due to it not emulating or symbolically executing anything.
Lifting to llvm IR will come in a next release which will solve this issue.
Next releases will also allow specifying the vmcontext to allow disassembling from a branch location instead of only vmentry.

## Example

### Call into vmp3 with pushed value
![example1](https://user-images.githubusercontent.com/102005914/175548145-8cb85a51-fef4-4a4c-b11b-f8049636b590.png)

### Converting the address to decimal (I need to add hex parsing :) )
![example2](https://user-images.githubusercontent.com/102005914/175548162-5d352eda-c66c-481b-ac7a-1697faa23e09.png)

### Invoking the disassembler
![example3](https://user-images.githubusercontent.com/102005914/175548166-ccc3bde9-fd20-44b7-850e-5b2c07119874.png)

## Example Output
```
VmContext {
    register_allocation: VmRegisterAllocation {
        vip: R8,
        vsp: Rsi,
        key: Rbp,
        handler_address: Rbx,
    },
    vm_entry_address: 5375745529,
    pushed_val: 18446744072235839311,
    vip_direction_forwards: true,
    push_order: [
        Rsi,
        Rbp,
        Rdi,
        Rdx,
        Rcx,
        R10,
        R13,
        R14,
        Flags,
        R9,
        Rax,
        R11,
        R8,
        Rbx,
        R15,
        R12,
    ],
    rolling_key: 7512180048,
    vip_value: 5372497739,
    handler_address: 5376204146,
}
0x140725d72 -> Pop(8, 98)
0x14069504f -> Pop(8, 50)
0x1406d0453 -> Pop(8, 48)
0x14069a8da -> Pop(8, 90)
0x1406de987 -> Pop(8, 58)
0x140740966 -> Pop(8, b0)
0x14076a534 -> Pop(8, a8)
0x14073cc29 -> Pop(8, 40)
0x14066b565 -> Pop(8, 88)
0x1406c5fd7 -> Pop(8, 38)
0x1406a0a37 -> Pop(8, 28)
0x140725d72 -> Pop(8, 0)
0x14069504f -> Pop(8, 80)
0x1406d0453 -> Pop(8, 20)
0x14069a8da -> Pop(8, 68)
0x1406de987 -> Pop(8, 18)
0x140740966 -> Pop(8, 10)
0x14076a534 -> Pop(8, 70)
0x14073cc29 -> Pop(8, 78)
0x1406b58aa -> PushImm64(1400148a2)
0x1407233bb -> Push(8, 98)
0x14065c9e4 -> Add(8)
0x14066b565 -> Pop(8, b8)
0x1406c5fd7 -> Pop(8, c0)
0x1406dd647 -> PushVsp(8)
0x1406a0a37 -> Pop(8, c8)
0x1407504ba -> Push(8, 18)
0x140725d72 -> Pop(8, d0)
0x1406b0c5d -> Push(8, 10)
0x14069504f -> Pop(8, d8)
0x14073ce7e -> Push(8, 68)
0x1406d0453 -> Pop(8, e0)
0x140776f28 -> Push(8, 90)
0x14069a8da -> Pop(8, e8)
0x14066ecc1 -> PushImm64(1407962a0)
0x14068f1ea -> PushImm32(3)
0x14067b681 -> Pop(4, 80)
0x140756802 -> PushImm32(0)
0x14068d4fe -> Pop(4, 84)
0x140720d16 -> PushImm64(140074b10)
0x140652822 -> Push(8, 98)
0x1406f2971 -> Add(8)
0x140720c6d -> Push(8, 90)
0x1406de987 -> Pop(8, b8)
0x140740966 -> Pop(8, a0)
0x14076a534 -> Pop(8, 60)
0x1407612e8 -> Push(8, 98)
0x140723579 -> Add(8)
0x14073cc29 -> Pop(8, 78)
0x140735929 -> PushImm64(140069068)
0x1406953ee -> Push(8, 98)
0x14071e780 -> Add(8)
0x14066b565 -> Pop(8, 58)
0x14071bcc3 -> Fetch(8)
0x140676a3e -> Push(8, 10)
0x140740fae -> Push(8, 18)
0x1407233bb -> Push(8, 68)
0x1407504ba -> Push(8, b8)
0x1406b0c5d -> Push(8, 80)
0x14073ce7e -> Push(8, 0)
0x140776f28 -> Push(8, 28)
0x140652822 -> Push(8, 38)
0x140720c6d -> Push(8, 88)
0x1407612e8 -> Push(8, 40)
0x1406953ee -> Push(8, a8)
0x140676a3e -> Push(8, b0)
0x140740fae -> Push(8, 60)
0x1407233bb -> Push(8, 90)
0x1407504ba -> Push(8, 48)
0x1406b0c5d -> Push(8, 50)
Disassembled no vip change
[Stopping]
0x1406f0494 -> VmExit
```
