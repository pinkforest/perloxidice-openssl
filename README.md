# perloxcidise OpenSSL

I'm wanting to see if I could transform / analyze OpenSSL + cryptograms codebase to transpile bits of it to Rust

Tricky bit is the C pre-processing for the defs and the perlmutations for build tooling to spit out all the assy out.

It's a mess atm.

## Bootstrap

$ git clone https://github.com/openssl/openssl.git

$ git clone https://github.com/dot-asm/cryptogams.git

## build.infos

$ rg -e perl openssl/crypto/**/build.info  <-- has 

## libperl

This can be used to source analyze the perl etc. beyond just running it w/ build permutations.

## LLVM IR

$ clang -I../../include -c -emit-llvm sha512.c -o sha512.bc

$ cargo run openssl-llvm-ir -- openssl/crypto/sha/sha512.bc

```
     Running `target/debug/openssl-llvm-ir openssl-llvm-ir`
===== llvm_ir::Module
name             = /home/foobar/code/cursed-perl/openssl/crypto/sha/sha512.bc
source_file_name = sha512.c
target_triple    = x86_64-unknown-linux-gnu
  --> Functions
 ?<-> [%1]sha512_224_init ([0]: ptr) -> I32
  <<< preds = []
  --> succs = ["Return"]
  <== preds_of_return = ["Number(1)"]
 ?<-> [%1]sha512_256_init ([0]: ptr) -> I32
  <<< preds = []
  --> succs = ["Return"]
  <== preds_of_return = ["Number(1)"]
 ?<-> [%1]SHA384_Init ([0]: ptr) -> I32
  <<< preds = []
  --> succs = ["Return"]
  <== preds_of_return = ["Number(1)"]
 ?<-> [%1]SHA512_Init ([0]: ptr) -> I32
  <<< preds = []
  --> succs = ["Return"]
  <== preds_of_return = ["Number(1)"]
 ?<-> [%2]SHA512_Final ([0]: ptr,[1]: ptr) -> I32
  <<< preds = []
  --> succs = ["Block(Number(27))", "Block(Number(35))"]
  <== preds_of_return = ["Number(398)"]
 ?<-> [%3]sha512_block_data_order ([0]: ptr,[1]: ptr,[2]: I64) -> void
  <<< preds = []
  --> succs = ["Block(Number(566))"]
  <== preds_of_return = ["Number(3493)"]
 ?<-> [%2]SHA384_Final ([0]: ptr,[1]: ptr) -> I32
  <<< preds = []
  --> succs = ["Return"]
  <== preds_of_return = ["Number(2)"]
 ?<-> [%3]SHA512_Update ([0]: ptr,[1]: ptr,[2]: I64) -> I32
  <<< preds = []
  --> succs = ["Block(Number(18))", "Block(Number(19))"]
  <== preds_of_return = ["Number(124)"]
 ?<-> [%3]SHA384_Update ([0]: ptr,[1]: ptr,[2]: I64) -> I32
  <<< preds = []
  --> succs = ["Return"]
  <== preds_of_return = ["Number(3)"]
 ?<-> [%2]SHA512_Transform ([0]: ptr,[1]: ptr) -> void
  <<< preds = []
  --> succs = ["Return"]
  <== preds_of_return = ["Number(2)"]
```

## ELF objdump

$ gcc -I../../include -c sha512.c -o sha512.o
$ objdump -t sha512.o 

sha512 ELF
```
sha512.o:     file format elf64-x86-64

SYMBOL TABLE:
0000000000000000 l    df *ABS*	0000000000000000 sha512.c
0000000000000000 l    d  .text	0000000000000000 .text
0000000000000ccd l     F .text	0000000000003c35 sha512_block_data_order
0000000000000000 l    d  .rodata	0000000000000000 .rodata
0000000000000000 l     O .rodata	0000000000000280 K512
0000000000000000 g     F .text	00000000000000d2 sha512_224_init
00000000000000d2 g     F .text	00000000000000d2 sha512_256_init
00000000000001a4 g     F .text	00000000000000d2 SHA384_Init
0000000000000276 g     F .text	00000000000000d2 SHA512_Init
0000000000000348 g     F .text	0000000000000721 SHA512_Final
0000000000000000         *UND*	0000000000000000 memset
0000000000000a69 g     F .text	0000000000000025 SHA384_Final
0000000000000a8e g     F .text	00000000000001e7 SHA512_Update
0000000000000000         *UND*	0000000000000000 memcpy
0000000000000c75 g     F .text	000000000000002d SHA384_Update
0000000000000ca2 g     F .text	000000000000002b SHA512_Transform
0000000000000000         *UND*	0000000000000000 __stack_chk_fail
```

conf files & tpl
```
$ find . -type f -name \*.conf
./Configurations/00-base-templates.conf
./Configurations/10-main.conf
./Configurations/15-android.conf
./Configurations/15-ios.conf
./Configurations/50-cppbuilder.conf
./Configurations/50-djgpp.conf
./Configurations/50-haiku.conf
./Configurations/50-masm.conf
./Configurations/50-nonstop.conf
./Configurations/50-os390.conf
./Configurations/50-vms-x86_64.conf
./Configurations/50-win-clang-cl.conf
./Configurations/50-win-hybridcrt.conf
./Configurations/50-win-onecore.conf

$ Configurations/unix-Makefile.tmpl
$ Configurations/windows-makefile.tmpl
```

conf desc for build.info
```
.conf ->

    asm_arch        => The architecture to be used for compiling assembly
                       source.  This acts as a selector in build.info files.
    uplink_arch     => The architecture to be used for compiling uplink
                       source.  This acts as a selector in build.info files.
                       This is separate from asm_arch because it's compiled
                       even when 'no-asm' is given, even though it contains
                       assembler source.
    perlasm_scheme  => The perlasm method used to create the
                       assembler files used when compiling with
```