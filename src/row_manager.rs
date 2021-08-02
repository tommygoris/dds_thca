use crate::data_source::HomeData;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use std::cmp::{max, min};

static MINIMUN_ROW_SELECTION: i32 = 1;
static MINIMUN_IMAGE_SELECTION: i32 = 1;
static IMAGE_WIDTH: u32 = 150;
static IMAGE_HEIGHT: u32 = 150;
static TEXT_WIDTH: u32 = 75;
static TEXT_HEIGHT: u32 = 45;
static SCALE: f32 = 1.25;
static PADDING: i32 = 25;
static PADDING_BETWEEN_ROWS: i32 = 75;
static PADDING_BETWEEN_TEXT_IMAGE: i32 = 50;
static PADDING_BETWEEN_IMAGES: i32 = 25;
static SCALED_IMAGE_WIDTH: u32 = (IMAGE_WIDTH as f32 * SCALE) as u32;
static SCALED_IMAGE_HEIGHT: u32 = (IMAGE_HEIGHT as f32 * SCALE) as u32;
static TEXT_COLOR: Color = Color::RGBA(255, 255, 255, 0);

/// Structure that handles a row.
struct Row<'a> {
    title_and_texture: (String, ImageTexture<'a>),
    x_position: i32,
    y_position: i32,
    images: Vec<ImageTexture<'a>>,
}

/// Structure to handle the location of a texture and the texture itself.
struct ImageTexture<'a> {
    y_position: i32,
    texture: Texture<'a>,
}

/// Structure to control the rows and the currently selected row/image.
pub struct RowManager<'a> {
    rows: Vec<Row<'a>>,
    selected_row: i32,
    selected_image: i32,
}

impl RowManager<'b> {
    pub fn new<'a>() -> RowManager<'a> {
        RowManager {
            rows: vec![],
            selected_row: MINIMUN_ROW_SELECTION,
            selected_image: MINIMUN_IMAGE_SELECTION,
        }
    }

    /// Creates a new row given a title for the row.
    pub fn create_row_and_draw_text(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        title: &str,
        texture_creator: &'b TextureCreator<WindowContext>,
    ) {
        let mut starting_y_position = PADDING;

        starting_y_position += (PADDING + IMAGE_HEIGHT as i32) * self.rows.len() as i32;
        starting_y_position += PADDING_BETWEEN_ROWS * self.rows.len() as i32;
        let starting_x_position = PADDING;

        let surface = font
            .render(title)
            .blended(TEXT_COLOR)
            .map_err(|e| e.to_string())
            .unwrap();

        let font_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let target = Rect::new(
            starting_x_position,
            starting_y_position,
            TEXT_WIDTH,
            TEXT_HEIGHT,
        );
        let _ = canvas.copy(&font_texture, None, target);

        self.rows.push(Row {
            title_and_texture: (
                title.parse().unwrap(),
                ImageTexture {
                    y_position: starting_y_position,
                    texture: font_texture,
                },
            ),
            x_position: starting_x_position,
            y_position: starting_y_position,
            images: vec![],
        });
    }

    /// Sets the appropriate selected row and selected image locations
    /// whenever an arrow key keydown is detected.
    pub fn move_image_selection(&mut self, keycode: Keycode) {
        if self.rows.is_empty() {
            return;
        }

        if keycode == Keycode::Up {
            self.selected_row = max(MINIMUN_ROW_SELECTION, self.selected_row - 1);

            if self.rows.len() > 0 {
                self.selected_image = max(
                    MINIMUN_IMAGE_SELECTION,
                    min(
                        self.selected_image,
                        self.rows[self.selected_row as usize - 1].images.len() as i32,
                    ),
                );
            }
        } else if keycode == Keycode::Down {
            if self.rows.len() > 0 {
                let previous_row = self.selected_row;

                self.selected_row = max(
                    MINIMUN_ROW_SELECTION,
                    min(self.selected_row + 1, self.rows.len() as i32),
                );

                // The number of images in the selected row is empty. Don't switch to the new row
                if self.rows[self.selected_row as usize - 1].images.is_empty() {
                    dbg!("here");
                    self.selected_row = previous_row;
                }
            }

            if self.rows.len() > 0 {
                self.selected_image = max(
                    MINIMUN_IMAGE_SELECTION,
                    min(
                        self.selected_image,
                        self.rows[self.selected_row as usize - 1].images.len() as i32,
                    ),
                );
            }
        } else if keycode == Keycode::Left {
            self.selected_image = max(MINIMUN_IMAGE_SELECTION, self.selected_image - 1);
        } else if keycode == Keycode::Right {
            if self.rows.len() > 0 {
                self.selected_image = max(
                    MINIMUN_IMAGE_SELECTION,
                    min(
                        self.selected_image + 1,
                        self.rows[self.selected_row as usize - 1].images.len() as i32,
                    ),
                );
            }
        }
        dbg!(self.selected_row);
        dbg!(self.selected_image);
    }

    /// Adds an image texture to the row manager at the specified row.
    /// row_to_add_to should start at 1 for the first row and so forth
    pub fn add_image_to_row(
        &mut self,
        row_to_add_to: u32,
        home_data: &HomeData,
        texture_creator: &'b TextureCreator<WindowContext>,
    ) {
        if row_to_add_to as usize > self.rows.len() {
            return;
        }

        let starting_y_position =
            self.rows[(row_to_add_to - 1) as usize].y_position + PADDING_BETWEEN_TEXT_IMAGE;

        let image_texture = texture_creator
            .load_texture_bytes(&*home_data.image())
            .unwrap();

        self.rows[(row_to_add_to - 1) as usize]
            .images
            .push(ImageTexture {
                y_position: starting_y_position,
                texture: image_texture,
            });
    }

    /// Redraws all textures back into the screen.
    pub fn redraw_all(&self, canvas: &mut WindowCanvas) {
        for (num_row, row) in self.rows.iter().enumerate() {
            dbg!(row.x_position);
            dbg!(row.y_position);
            let target = Rect::new(row.x_position, row.y_position, TEXT_WIDTH, TEXT_HEIGHT);

            let _ = canvas.copy(&row.title_and_texture.1.texture, None, target);
            let mut is_image_in_row_scaled = false;

            for (num_image, image) in row.images.iter().enumerate() {
                let mut x_position = PADDING_BETWEEN_IMAGES * (num_image as i32 + 1);
                x_position += num_image as i32 * IMAGE_WIDTH as i32;

                if num_row as i32 == self.selected_row - 1
                    && num_image as i32 == self.selected_image - 1
                {
                    is_image_in_row_scaled = true;

                    self.scale_up_selected_image(canvas, x_position);
                    continue;
                }

                if is_image_in_row_scaled {
                    x_position += (SCALED_IMAGE_WIDTH - IMAGE_WIDTH) as i32;
                }

                let target = Rect::new(x_position, image.y_position, IMAGE_WIDTH, IMAGE_HEIGHT);
                let _ = canvas.copy(&image.texture, None, target);
            }
        }
    }

    /// Scales up the selected image
    fn scale_up_selected_image(&self, canvas: &mut WindowCanvas, x_position: i32) {
        if self.rows.is_empty() {
            dbg!("Failed to scale up an image, no rows were found");
            return;
        }

        if self.rows[self.selected_row as usize - 1].images.is_empty() {
            dbg!("Failed to scale up image, no available image in row");
            return;
        }
        if self.rows[self.selected_row as usize - 1].images.len()
            < (self.selected_image - 1) as usize
        {
            dbg!("Failed to scale up image, the selected image was not found");
            return;
        }

        dbg!("Scaling up selected image");
        let selected_image =
            &self.rows[self.selected_row as usize - 1].images[self.selected_image as usize - 1];

        let target = Rect::new(
            x_position,
            selected_image.y_position,
            SCALED_IMAGE_WIDTH,
            SCALED_IMAGE_HEIGHT,
        );

        let _ = canvas.copy(&selected_image.texture, None, target);
    }
}
