// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

mod export;

export! {

	@Java_com_reactnativerust_RustBridge_helloWorld
	fn hello_world() -> String {
		let res = fib(42);
		res.to_string()
	}

}
fn fib(n: u64) -> u64 {
	if n <= 1 {
		return n;
	}
	fib(n - 1) + fib(n - 2)
}
ffi_support::define_string_destructor!(signer_destroy_string);
