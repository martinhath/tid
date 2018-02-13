//! Maybe the simplest time taking crate there is.
//!
//! The `timed!` macro prints the labels as `{:<26}` (left aligned, 26 char length). The only
//! reason `26` is chosen is because my longest label happend to be around 26 chars long.
//! The timings are printed as floating points in microseconds, for much of the same reasons.
//! The exact print format is hardcoded as:
//!
//! ```
//! # let t1 = 1; let t0 = 0; let label = "hei";
//! println!("[timed] {:<26} {:9.4}ms", label, (t1 - t0) as f64 / 1_000_000.0);
//! ```
//!
//! # Examples
//!
//! The crate has a macro `timed!` which is used for timing a block:
//!
//! ```
//! # #[macro_use] extern crate tid;
//! # fn main() {
//! timed!("pushing some stuff",
//!     let mut v = Vec::new();
//!     for i in 0..100 {
//!         v.push(i);
//!     };     // note the `;` here
//! );
//! let q = v; // `v` is still reachable out here.
//! # }
//! ```
//!
//! If you have multiple consecutive blocks, you can use `Timer` instead.
//!
//! ```
//! # use tid::Timer;
//! # fn f() {  }
//! # fn g() {  }
//! # fn h() {  }
//! let mut t = Timer::new();
//! f();
//! t.mark("Doing f");
//! g();
//! t.mark("G is executed");
//! h();
//! t.mark("Done with H");
//! t.present();
//! ```
//!
extern crate time;

#[macro_export]
/// Time a block of code. For macro reasons, all blocks must be terminated by `;`.
macro_rules! timed {
    ($name:expr, $($block:stmt);+;) => (
        let t0 = $crate::_time();
        $($block);+;
        let t1 = $crate::_time();
        println!("[timed] {:<26} {:9.4}ms", $name, (t1 - t0) as f64 / 1_000_000.0);
    )
}

#[doc(hidden)]
// Wrap `time::precise_time_ns` so the crate using `tid` doesn't have to depend on `time`.
// Maybe there is a better way to do this?
pub fn _time() -> u64 {
    time::precise_time_ns()
}

/// A `Timer` is used for timing multiple consecutive sections of your code. The first timing is
/// done when the object is constructed. The second timing is done at the first call to `mark`.
/// This time difference will be the one reported with the label you pass to `mark`.
///
/// # Examples
///
/// ```
/// # use tid::Timer;
/// # fn f() {  }
/// # fn g() {  }
/// # fn h() {  }
/// let mut t = Timer::new();
/// f();
/// t.mark("Doing f");
/// g();
/// t.mark("G is executed");
/// h();
/// t.mark("Done with H");
/// t.present();
/// ```
///
/// When `present` is called, we print all timings:
///
/// ```text
/// [timer] Doing f          0.12004ms
/// [timer] G is executed   21.98122ms
/// [timer] Done with H      7.00124ms
/// ```
pub struct Timer {
    times: Vec<u64>,
    strs: Vec<&'static str>,
}

impl Timer {
    /// Create a new `Timer`. The first time sample is taken here.
    pub fn new() -> Self {
        let mut s = Self {
            times: Vec::with_capacity(100),
            strs: Vec::with_capacity(100),
        };
        s.times.push(time::precise_time_ns());
        s
    }

    /// Mark off a secion with the given label.
    pub fn mark(&mut self, label: &'static str) {
        self.times.push(time::precise_time_ns());
        self.strs.push(label);
    }

    /// Print out the timings to `stdout`.
    pub fn present(self) {
        let diffs = self.times.iter().zip(self.times.iter().skip(1)).map(
            |(a, b)| {
                b - a
            },
        );
        for (time, s) in diffs.zip(self.strs.iter()) {
            println!("\t[timer] {:<26} {:9.4}ms", s, time as f64 / 1_000_000.0);
        }
    }
}
