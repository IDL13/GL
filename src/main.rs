// Импорт необходимых модулей из библиотеки SDL2
use sdl2::event::Event;
use crate::winsdl::Winsdl;
use crate::objects::*;

// Импорт модуля winsdl из текущего crate
mod winsdl;
mod objects;

fn main() {
    // Инициализация и создание экземпляра Winsdl с размерами окна 800x800
    let mut winsdl = Winsdl::new(800, 800).unwrap();

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
            // Установка цвета очистки буфера цвета
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            // Очистка буфера цвета
            gl::Clear(gl::COLOR_BUFFER_BIT)
        }

        // Обмен буферов окна SDL
        winsdl.window.gl_swap_window();
    }
}
