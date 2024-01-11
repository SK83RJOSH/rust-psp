use alloc::boxed::Box;
use core::any::Any;
use core::ffi::c_void;
use core::panic::PanicInfo;
use unwinding::{
    abi::{UnwindContext, UnwindReasonCode, _Unwind_Backtrace, _Unwind_GetIP},
    panic::begin_panic,
};

static mut PANIC_COUNT: usize = 0;

fn stack_trace() {
    struct CallbackData {
        counter: usize,
    }
    extern "C" fn callback(unwind_ctx: &UnwindContext<'_>, arg: *mut c_void) -> UnwindReasonCode {
        let data = unsafe { &mut *(arg as *mut CallbackData) };
        data.counter += 1;
        dprintln!(
            "{:4}:{:#19x} - <unknown>",
            data.counter,
            _Unwind_GetIP(unwind_ctx)
        );
        UnwindReasonCode::NO_REASON
    }
    let mut data = CallbackData { counter: 0 };
    _Unwind_Backtrace(callback, &mut data as *mut _ as _);
}

fn do_panic(msg: Box<dyn Any + Send>) -> ! {
    fn abort() -> ! {
        unsafe {
            crate::sys::sceKernelExitDeleteThread(1);
            core::intrinsics::unreachable()
        }
    }
    unsafe {
        if PANIC_COUNT >= 1 {
            stack_trace();
            dprintln!("thread panicked while processing panic. aborting.");
            abort();
        }
        PANIC_COUNT = 1;
    }
    stack_trace();

    let code = begin_panic(Box::new(msg)).0;
    dprintln!("failed to initiate panic, error {code}");
    abort();
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    dprintln!("{}", info);

    struct NoPayload;
    do_panic(Box::new(NoPayload))
}
