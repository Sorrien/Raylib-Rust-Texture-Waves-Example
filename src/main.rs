use raylib::prelude::*;

fn main() {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("raylib [shaders] example - texture waves")
        .build();

    // Load texture texture to apply shaders
    let texture = rl
        .load_texture(&thread, "resources/space.png")
        .expect("Could not load space texture");

    let mut shader = rl
        .load_shader(&thread, None, Some("resources/shaders/glsl330/wave.fs"))
        .expect("Could not load wave shader");

    let seconds_loc = shader.get_shader_location("seconds");
    let freq_x_loc = shader.get_shader_location("freqX");
    let freq_y_loc = shader.get_shader_location("freqY");
    let amp_x_loc = shader.get_shader_location("ampX");
    let amp_y_loc = shader.get_shader_location("ampY");
    let speed_x_loc = shader.get_shader_location("speedX");
    let speed_y_loc = shader.get_shader_location("speedY");
    let size_loc = shader.get_shader_location("size");

    // Shader uniform values that can be updated at any time
    let freq_x = 25.0;
    let freq_y = 25.0;
    let amp_x = 5.0;
    let amp_y = 5.0;
    let speed_x = 8.0;
    let speed_y = 8.0;

    let screen_size = [screen_width as f32, screen_height as f32];
    shader.set_shader_value(size_loc, screen_size);
    shader.set_shader_value(freq_x_loc, freq_x);
    shader.set_shader_value(freq_y_loc, freq_y);
    shader.set_shader_value(amp_x_loc, amp_x);
    shader.set_shader_value(amp_y_loc, amp_y);
    shader.set_shader_value(speed_x_loc, speed_x);
    shader.set_shader_value(speed_y_loc, speed_y);

    let mut seconds = 0.0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second

    // Main game loop
    while !rl.window_should_close() { // Detect window close button or ESC key
        //Update
        seconds = seconds + rl.get_frame_time();

        shader.set_shader_value(seconds_loc, seconds);

        //Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        let mut shader_mode = d.begin_shader_mode(&shader);

        shader_mode.draw_texture(&texture, 0, 0, Color::WHITE);
        shader_mode.draw_texture(&texture, texture.width(), 0, Color::WHITE);
    }
}
