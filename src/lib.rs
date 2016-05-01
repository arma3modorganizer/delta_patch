// librsync(rust) -- library for network deltas
// Copyright 2015, 2016 Martin Pool.

// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.

extern crate libc;
extern crate librdiff;

#[no_mangle]
pub extern fn rs_version() -> *const libc::c_char {
    // Version from environment has nul termination (I think we can count on this?)
    return librdiff::VERSION.as_ptr() as *const libc::c_char;
}
