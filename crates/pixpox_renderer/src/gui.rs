use imgui::{ProgressBar, Ui};
use pixpox_utils::Stats;

use crate::{wgpu, Pixels, PixelsContext};
use std::time::Instant;

pub struct GuiParent<'a> {
    label: &'static str,
    children: Vec<&'a mut GuiChild<'a>>,
}

pub struct GuiChild<'a> {
    label: &'static str,
    cb: &'a mut dyn FnMut(&mut Ui, &mut bool, &Stats),
    state: &'a mut bool,
}

impl<'a> GuiChild<'a> {
    pub fn new(
        label: &'static str,
        cb: &'a mut dyn FnMut(&mut Ui, &mut bool, &Stats),
        state: &'a mut bool,
    ) -> Self {
        Self { label, cb, state }
    }
}

/// Manages all state required for rendering Dear ImGui over `Pixels`.
pub struct Gui<'a> {
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
    last_cursor: Option<imgui::MouseCursor>,
    gui_entries: Vec<GuiParent<'a>>,
}

impl<'a> Gui<'a> {
    /// Create Dear ImGui.
    pub fn new(window: &winit::window::Window, pixels: &Pixels) -> Self {
        // Create Dear ImGui context
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        // Initialize winit platform support
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );

        // Configure Dear ImGui fonts
        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        // Create Dear ImGui WGPU renderer
        let device = pixels.device();
        let queue = pixels.queue();
        let config = imgui_wgpu::RendererConfig {
            texture_format: pixels.render_texture_format(),
            ..Default::default()
        };
        let renderer = imgui_wgpu::Renderer::new(&mut imgui, device, queue, config);

        // Return GUI context
        Self {
            imgui,
            platform,
            renderer,
            last_frame: Instant::now(),
            last_cursor: None,
            gui_entries: Vec::new(),
        }
    }

    /// Prepare Dear ImGui.
    pub fn prepare(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), winit::error::ExternalError> {
        // Prepare Dear ImGui
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;
        self.platform.prepare_frame(self.imgui.io_mut(), window)
    }

    /// Render Dear ImGui.
    pub fn render(
        &mut self,
        window: &winit::window::Window,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
        stats: &Stats,
    ) -> imgui_wgpu::RendererResult<()> {
        // Start a new Dear ImGui frame and update the cursor
        let ui = self.imgui.frame();

        let mouse_cursor = ui.mouse_cursor();
        if self.last_cursor != mouse_cursor {
            self.last_cursor = mouse_cursor;
            self.platform.prepare_render(ui, window);
        }

        // Draw windows and GUI elements here
        ui.main_menu_bar(|| {
            self.gui_entries.iter_mut().for_each(|parent| {
                ui.menu(parent.label, || {
                    parent.children.iter_mut().for_each(|child| {
                        *child.state = ui.menu_item(child.label);
                    })
                })
            });
        });

        self.gui_entries.iter_mut().for_each(|parent| {
            parent.children.iter_mut().for_each(|child| {
                if *child.state {
                    (*child.cb)(ui, &mut *child.state, stats);
                }
            });
        });

        // Render Dear ImGui with WGPU
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("imgui"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.renderer.render(
            self.imgui.render(),
            &context.queue,
            &context.device,
            &mut rpass,
        )
    }

    /// Handle any outstanding events.
    pub fn handle_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::Event<()>,
    ) -> bool {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);

        // If ImGui is capturing the mouse, we don't want to process the click in our game logic.
        if self.imgui.io().want_capture_mouse {
            return false;
        }

        true
    }

    pub fn register_parent(&mut self, label: &'static str) {
        self.gui_entries.push(GuiParent {
            label,
            children: Vec::new(),
        });
    }

    pub fn register_child(&mut self, parent_label: &'static str, child: &'a mut GuiChild<'a>) {
        for parent in self.gui_entries.iter_mut() {
            if parent.label == parent_label {
                parent.children.push(child);
                return;
            }
        }
    }
}
