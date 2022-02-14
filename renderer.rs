//   ASCII Sphere Renderer - Draws sphere on terminal. 
//   Copyright (C) 2022  Kerem Biçen

//   This program is free software: you can redistribute it and/or modify
//   it under the terms of the GNU General Public License as published by
//   the Free Software Foundation, either version 3 of the License, or
//   (at your option) any later version.

//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License for more details.

//   You should have received a copy of the GNU General Public License
//   along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io;
use std::io::Write;

fn main()
{
	let mut matris = [[' '; 21]; 21];
	let mut matris3d = [[[0u8; 21]; 21]; 21];

	for z in 0..matris3d[0][0].len()
	{
		let rng = if z <= 10 { z } else { 20 - z };
		for ang in 0..360
		{
			for distance in 1..(rng + 1)
			{
				let x = 0 - (((ang as f32) * std::f32::consts::PI / 180.0).sin() * (distance as f32)) as i8;
				let y = (((ang as f32) * std::f32::consts::PI / 180.0).cos() * (distance as f32)) as i8;
				matris3d[(10 + x) as usize][(10 + y) as usize][z] = 1;
			}
		}
	}

	let shaders = ['@', '$', '#', '*', '!', '=', ';', ':', '~', '-', ',', '.'];

	for z in 0..matris3d[0][0].len()
	{
		for x in 0..matris3d.len()
		{
			for y in 0..matris3d[0].len()
			{
				if matris3d[x][y][z] == 1 && matris[x][y] == ' '
				{
					let locx = if x >= 10 { x - 10 } else { 10 - x };
					let locy = if y >= 10 { y - 10 } else { 10 - y };
					let distance = (((locx.pow(2) + locy.pow(2) + z.pow(2)) as f32).sqrt() / 3.0) as usize;
					matris[x][y] = if distance < shaders.len() { shaders[distance] } else { ' ' };
				} 
			}
		}
	}

	for x in 0..matris.len()
	{
		for y in 0..matris[0].len()
		{
			print!("{} ", matris[x][y]);
			io::stdout().flush().unwrap();
		}
		println!("");
	}
}
