// Импорт необходимых модулей из библиотеки SDL2
use sdl2::{video::{GLContext, SwapInterval, Window}, EventPump, Sdl};

// Определение структуры Winsdl, которая будет содержать все необходимые компоненты для работы с SDL2
pub struct Winsdl {
    pub sdl: Sdl, // Экземпляр SDL
    pub window: Window, // Окно SDL
    pub gl_context: GLContext, // Контекст OpenGL
    pub gl: (), // Загрузчик OpenGL
    pub event_pump: EventPump, // Событийная очередь SDL
}

// Реализация структуры Winsdl
impl Winsdl {
    // Метод для создания нового экземпляра Winsdl
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        // Инициализация SDL
        let sdl = sdl2::init().unwrap();
        // Получение доступа к подсистеме видео
        let video_subsystem = sdl.video().unwrap();

        // Настройка атрибутов OpenGL
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // Создание окна с поддержкой OpenGL
        let window = video_subsystem
            .window("Window", width as u32, height as u32)
            .opengl()
            .build()
            .unwrap();

        // Создание контекста OpenGL
        let gl_context = window.gl_create_context().unwrap();
        // Загрузка функций OpenGL
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        // Установка режима вертикальной синхронизации
        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();

        // Получение событийной очереди SDL
        let event_pump = sdl.event_pump().unwrap();

        // Возврат нового экземпляра Winsdl в случае успеха
        Ok(Winsdl{
            sdl,
            window,
            gl_context,
            gl,
            event_pump,
        })
    }
}