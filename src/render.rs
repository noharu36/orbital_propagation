use full_palette::{BLUE, GREY};
use sgp4::{Constants, Elements};
use plotters::{chart::ChartState, coord::{ranged3d::Cartesian3d, types::RangedCoordf64}, prelude::*};
use minifb::{Key, Window, WindowOptions};
use std::borrow::{Borrow, BorrowMut};
use plotters_bitmap::bitmap_pixel::BGRXPixel;

const WIDTH: usize = 1200;
const HEIGHT: usize = 900;

struct BufferWrapper(Vec<u32>);
impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe {
            std::slice::from_raw_parts(
                self.0.as_ptr() as *const u8,
                self.0.len() * 4
            )
        }
    }
}
impl BorrowMut<[u8]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.as_mut_ptr() as *mut u8,
                self.0.len() * 4
            )
        }
    }
}
impl Borrow<[u32]> for BufferWrapper {
    fn borrow(&self) -> &[u32] {
        self.0.as_slice()
    }
}
impl BorrowMut<[u32]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u32] {
        self.0.as_mut_slice()
    }
}

pub fn render(elements: Vec<Elements>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = BufferWrapper(vec![0u32; WIDTH * HEIGHT]);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;

    let cs = {
        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (WIDTH as u32, HEIGHT as u32))?.into_drawing_area();
        root.fill(&GREY)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Orbital_Calc", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_3d(-7000.0..7000.0, -7000.0..7000.0, -7000.0..7000.0)?;

        chart.configure_axes()
            .x_formatter(&|x| format!("x={x}"))
            .y_formatter(&|y| format!("y={y}"))
            .z_formatter(&|z| format!("z={z}"))
            .draw()?;

        let cs = chart.into_chart_state();
        root.present()?;
        cs
    };
    window.set_target_fps(60);

    render_plot(&mut window, elements, &mut buf, &cs)?;

    Ok(())

}

fn array_to_tuple(array: [f64; 3]) -> (f64, f64, f64) {
    (array[0], array[1], array[2])
}

fn render_plot(window: &mut Window, elements: Vec<Elements>, buf: &mut BufferWrapper, cs: &ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>) -> Result<(), Box<dyn std::error::Error>>{
    let mut min: f64 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        {
            let mut data_vec: Vec<Vec<(f64, f64, f64)>> = Vec::new();
            elements.iter().for_each(|e| {
                let constants = Constants::from_elements(&e).unwrap();
                let mut data: Vec<(f64, f64, f64)> = Vec::new();
                for i in 0..10 {
                    let position = constants.propagate(sgp4::MinutesSinceEpoch(min + i as f64 * 0.1)).unwrap().position;
                    data.push(array_to_tuple(position));
                }
                data_vec.push(data);
            });

            let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(),(WIDTH as u32, HEIGHT as u32),)?.into_drawing_area();

            let mut chart = cs.clone().restore(&root);
            chart.plotting_area().fill(&GREY)?;
            chart.configure_axes()
                .x_formatter(&|x| format!("x={x}"))
                .y_formatter(&|y| format!("y={y}"))
                .z_formatter(&|z| format!("z={z}"))
                .draw()?;
            chart.plotting_area().draw(&Circle::new((0_f64, 0_f64, 0_f64), 290, BLUE.filled()))?;

            data_vec.iter().try_for_each(|d| -> Result<(), Box<dyn std::error::Error>>{
                chart.plotting_area().draw(&Circle::new(d[d.len()-1], 1, RED.filled()))?;
                Ok(())
            })?;
            root.present()?;

            min += 0.5;
        }

        window.update_with_buffer(buf.borrow_mut(), WIDTH, HEIGHT)?;

    }
    Ok(())
}
