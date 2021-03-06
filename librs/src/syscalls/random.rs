// Copyright (c) 2018 Colin Finck, RWTH Aachen University
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use arch;
use synch::spinlock::Spinlock;

lazy_static! {
	static ref PARK_MILLER_LEHMER_SEED: Spinlock<u32> =
		Spinlock::new(arch::processor::get_timestamp() as u32);
}


fn generate_park_miller_lehmer_random_number() -> u32 {
	let mut seed = PARK_MILLER_LEHMER_SEED.lock();
	let random = (((*seed) as u64 * 48271) % 2147483647) as u32;
	*seed = random;
	random
}

#[no_mangle]
pub extern "C" fn sys_rand() -> u32 {
	if let Some(value) = arch::processor::generate_random_number() {
		value
	} else {
		generate_park_miller_lehmer_random_number()
	}
}
