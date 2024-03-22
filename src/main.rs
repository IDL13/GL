// Импорт необходимых модулей из библиотеки SDL2
use sdl2::event::Event;
use crate::winsdl::Winsdl;
use crate::objects::*;
use std::time::Instant;

// Импорт модуля winsdl из текущего crate
mod winsdl;
mod objects;

fn main() {
    // Инициализация и создание экземпляра Winsdl с размерами окна 800x800
    let mut winsdl = Winsdl::new(800, 800).unwrap();

    unsafe {gl::Viewport(0, 0, 1000, 1000)}

    let program = create_program().unwrap();
    program.set();

    let vertices: Vec<f32> = vec![
        -1.0, -1.0,
        1.0, -1.0,
        1.0, 1.0,

        1.0, 1.0,
        -1.0, 1.0,
        -1.0, -1.0,
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2,
    ];

    let vbo = VBO::gen();
    vbo.set(&vertices);

    let vao = VAO::gen();
    vao.set();

    let ibo = IBO::gen();
    ibo.set(&indices);

    // Бесконечный цикл основного приложения
    'main_loop: loop {
        // Обработка событий из очереди событий SDL
        for event in winsdl.event_pump.poll_iter() {
            match event {
                // Если событие - это закрытие окна, то выходим из цикла
                Event::Quit {..} => break 'main_loop,
                // Для остальных событий ничего не делаем
                _ => {}
            }
        }
        
        // Безопасный блок для вызова небезопасных функций OpenGL
        unsafe {
            // Очистка буфера цвета
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Установка цвета очистки буфера цвета
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);

            gl::Uniform1f(program.time(), Instant::now().elapsed().as_secs_f32());

            gl::DrawArrays(gl::TRIANGLES, 0, 6)

            // gl::DrawElements(
            //     gl::TRIANGLES,
            //     indices.len() as i32,
            //     gl::UNSIGNED_INT,
            //     0 as *const _
            // );
        }

        // Обмен буферов окна SDL
        winsdl.window.gl_swap_window();
    }
}
