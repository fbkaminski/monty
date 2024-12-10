use std::mem;
use std::ptr;
use libc;

use ast::ast::{BinaryExpression, BinaryOp};

pub type JITFunction = extern "C" fn(i32, i32) -> i32;

pub struct Jit {

}

impl Jit {

    pub fn new() -> Jit {
        Jit {}
    }

    pub fn compile_operation(&self, expr: BinaryOp) -> JITFunction {
        let stencil = self.create_stencil(expr);
        unsafe {
            let operation = self.jit_compile(&stencil);
            return operation;
        }
    }

    // FIXME: we should have an Assembler -> X86_64 and use emit_iadd(), emit_sub(), emit_mul(), emit_div() etc..
    // Generate a stencil for basic math operations
    fn create_stencil(&self, operation: BinaryOp) -> Vec<u8> {

        let add_stencil = vec![
            0x89, 0xF8,       // mov eax, edi
            0x01, 0xF0,       // iadd eax, esi
            0xC3              // ret
        ];

        let sub_stencil = vec![
            0x89, 0xF8,       // mov eax, edi
            0x29, 0xF0,       // sub eax, esi
            0xC3              // ret
        ];

        let mul_stencil = vec![
            0x89, 0xF8,       // mov eax, edi
            0x0F, 0xAF, 0xC6, // imul eax, esi
            0xC3              // ret
        ];

        let div_stencil = vec![
            0x31, 0xD2,       // xor edx, edx
            0x89, 0xF8,       // mov eax, edi
            0x89, 0xF1,       // mov ecx, esi
            0xF7, 0xF9,       // idiv ecx
            0xC3              // ret
        ];

        match operation {
            BinaryOp::Addition => {
                return add_stencil;
            },
            BinaryOp::Subtraction => {
                return sub_stencil;
            }
            BinaryOp::Multiplication => {
                return mul_stencil;
            },
            BinaryOp::Division => {
                return div_stencil;
            },
        }
    }

    // Allocate executable memory using `mmap`
    unsafe fn map_buffer(&self, size: usize) -> *mut u8 {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        if ptr == libc::MAP_FAILED {
            panic!("Failed to allocate executable memory");
        }
        ptr as *mut u8
    }

    unsafe fn jit_compile(&self, stencil: &[u8]) -> JITFunction {
        let size = stencil.len();
        let mem = self.map_buffer(size);
        // Copy stencil into executable memory
        ptr::copy_nonoverlapping(stencil.as_ptr(), mem, size);
        // Cast memory pointer to a callable function
        mem::transmute(mem)
    }

}