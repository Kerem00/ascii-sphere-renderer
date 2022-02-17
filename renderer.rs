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

	for angh in 0..360
	{
		let x = 0f32 - (((angh as f32) * std::f32::consts::PI / 180.0).sin() * 10.0);
		let y = ((angh as f32) * std::f32::consts::PI / 180.0).cos() * 10.0;
		
		for angv in 0..180
		{
			let xt = 0f32 - (((angv as f32) * std::f32::consts::PI / 180.0).sin() * x);
			let z = ((angv as f32) * std::f32::consts::PI / 180.0).cos() * x;
			
			matris3d[(10.0 + xt).round() as usize][(10.0 + y).round() as usize][(10.0 + z).round() as usize] = 1; 
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
					let distance = (((locx.pow(2) + locy.pow(2) + z.pow(2)) as f32).sqrt() / 2.0) as usize;
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
