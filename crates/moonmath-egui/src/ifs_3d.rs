//! 3D visualization of an iterated function system, after Falconer Fig 9.3.
//!
//! Three planar affine contractions `S_1, S_2, S_3` map the unit square `E`
//! onto rectangles. We render iteration depth on the z-axis so each generation
//! of the deterministic algorithm (Falconer's Method (a)) sits in its own
//! layer — the stack telescopes toward the attractor `F`. Method (b) (the
//! chaos game point cloud) is overlaid at z = 0.
//!
//! The "3D" is software-projected: we project points/lines from R^3 to the
//! egui screen ourselves and draw via the standard 2D `Painter`. No glow/wgpu.

use eframe::egui;
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2};

/// A 2D affine map `S(x) = A x + b` with `A` a 2×2 matrix and `b ∈ R^2`.
#[derive(Clone, Copy, Debug)]
pub struct Affine2 {
    pub a: [[f32; 2]; 2],
    pub b: [f32; 2],
}

impl Affine2 {
    fn apply(&self, p: [f32; 2]) -> [f32; 2] {
        [
            self.a[0][0] * p[0] + self.a[0][1] * p[1] + self.b[0],
            self.a[1][0] * p[0] + self.a[1][1] * p[1] + self.b[1],
        ]
    }
}

/// Falconer Fig 9.3 contractions: square `E = [0,1]^2` mapped to three
/// rectangles, one tall-left and two wide arranged top-right / bottom-right.
///
/// Numbers chosen to roughly match the figure rather than any canonical
/// attractor — the goal is to reproduce the textbook illustration.
fn falconer_ifs() -> [Affine2; 3] {
    // S_1: tall-left rectangle, scale (0.45, 0.85), translated to bottom-left.
    let s1 = Affine2 {
        a: [[0.45, 0.0], [0.0, 0.85]],
        b: [0.02, 0.05],
    };
    // S_2: wide top-right rectangle with a small negative shear so the
    // generated parallelogram tilts the way the figure shows.
    let s2 = Affine2 {
        a: [[0.55, 0.08], [-0.04, 0.45]],
        b: [0.43, 0.50],
    };
    // S_3: wide bottom-right rectangle, mostly axis-aligned.
    let s3 = Affine2 {
        a: [[0.55, 0.0], [0.0, 0.40]],
        b: [0.43, 0.05],
    };
    [s1, s2, s3]
}

/// All 3^k composed parallelograms at iteration depth k, expressed as the four
/// corners of `S_{i_1} ∘ S_{i_2} ∘ ... ∘ S_{i_k}` applied to the unit square.
fn parallelograms_at_depth(maps: &[Affine2; 3], k: u32) -> Vec<[[f32; 2]; 4]> {
    let unit_square = [[0.0_f32, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let mut out: Vec<[[f32; 2]; 4]> = vec![unit_square];
    for _ in 0..k {
        let mut next = Vec::with_capacity(out.len() * 3);
        for poly in &out {
            for s in maps {
                let mapped = [
                    s.apply(poly[0]),
                    s.apply(poly[1]),
                    s.apply(poly[2]),
                    s.apply(poly[3]),
                ];
                next.push(mapped);
            }
        }
        out = next;
    }
    out
}

/// Chaos game (Method (b)): random orbit `x_{n+1} = S_{i_n}(x_n)`.
fn chaos_game(maps: &[Affine2; 3], n_points: usize, seed: u64) -> Vec<[f32; 2]> {
    // tiny xorshift; we don't need cryptographic quality
    let mut state = seed | 1;
    let mut next_u32 = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state as u32
    };

    let mut p = [0.5_f32, 0.5];
    let mut pts = Vec::with_capacity(n_points);
    // Burn-in so we land on the attractor before recording.
    for _ in 0..30 {
        let i = (next_u32() as usize) % maps.len();
        p = maps[i].apply(p);
    }
    for _ in 0..n_points {
        let i = (next_u32() as usize) % maps.len();
        p = maps[i].apply(p);
        pts.push(p);
    }
    pts
}

/// Orbit camera, parameterised by spherical angles around the origin.
#[derive(Clone, Copy)]
struct Camera {
    azimuth: f32,   // radians, around z-axis (the iteration axis)
    elevation: f32, // radians, lift above the xy-plane
    distance: f32,  // distance from look-at point
}

impl Camera {
    /// Project a point in IFS-world (x, y, z) where (x,y) ∈ [0,1]^2 and z is
    /// iteration depth, onto a unit-cube-centered eye view.
    fn project(&self, p: [f32; 3], screen: Rect) -> Pos2 {
        // Centre the IFS world: x,y around 0.5, z around mid-depth.
        // Caller passes in pre-centred coords; we just apply rotation +
        // perspective.
        let (sa, ca) = self.azimuth.sin_cos();
        let (se, ce) = self.elevation.sin_cos();

        // World → camera: rotate around z by -azimuth, then around x by -elevation.
        let (x, y, z) = (p[0], p[1], p[2]);
        let xr = ca * x + sa * y;
        let yr = -sa * x + ca * y;
        let zr = z;
        let yc = ce * yr - se * zr;
        let zc = se * yr + ce * zr;

        // Perspective with near plane at distance.
        let depth = self.distance + zc;
        let safe_depth = depth.max(0.05);
        let f = self.distance / safe_depth;

        let cx = screen.center().x;
        let cy = screen.center().y;
        let scale = screen.size().min_elem() * 0.42;
        Pos2::new(cx + xr * scale * f, cy - yc * scale * f)
    }
}

pub struct IfsApp {
    maps: [Affine2; 3],
    /// Iteration depth k for the deterministic Method (a).
    depth: u32,
    /// Chaos-game point count (Method (b)).
    chaos_points: usize,
    /// Spacing between iteration layers along z.
    layer_spacing: f32,
    /// Show the deterministic wireframe layers.
    show_method_a: bool,
    /// Show the chaos-game point cloud.
    show_method_b: bool,
    camera: Camera,
    /// Cached chaos-game points, recomputed when count or seed changes.
    cached_chaos: Vec<[f32; 2]>,
    chaos_seed: u64,
}

impl Default for IfsApp {
    fn default() -> Self {
        let maps = falconer_ifs();
        let mut app = Self {
            maps,
            depth: 4,
            chaos_points: 4000,
            layer_spacing: 0.55,
            show_method_a: true,
            show_method_b: true,
            camera: Camera {
                azimuth: 0.55,
                elevation: 0.32,
                distance: 3.2,
            },
            cached_chaos: Vec::new(),
            chaos_seed: 0xC0FFEE,
        };
        app.recompute_chaos();
        app
    }
}

impl IfsApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn recompute_chaos(&mut self) {
        self.cached_chaos = chaos_game(&self.maps, self.chaos_points, self.chaos_seed);
    }
}

const LAYER_PALETTE: [Color32; 7] = [
    Color32::from_rgb(244, 162, 97),  // gen 0: square
    Color32::from_rgb(231, 111, 81),
    Color32::from_rgb(233, 196, 106),
    Color32::from_rgb(42, 157, 143),
    Color32::from_rgb(38, 70, 83),
    Color32::from_rgb(125, 135, 199),
    Color32::from_rgb(180, 180, 180),
];

impl eframe::App for IfsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("ifs_controls")
            .resizable(false)
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("IFS — Falconer 9.3");
                ui.label(
                    "Three affine contractions S₁, S₂, S₃ mapping the unit square \
                     onto rectangles. Iteration depth k is the z-axis.",
                );
                ui.separator();

                let mut chaos_dirty = false;
                ui.horizontal(|ui| {
                    ui.label("Depth k");
                    if ui
                        .add(egui::Slider::new(&mut self.depth, 0..=6))
                        .changed()
                    {
                        // depth doesn't affect chaos points
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Layer dz");
                    ui.add(egui::Slider::new(&mut self.layer_spacing, 0.05..=1.5));
                });
                ui.checkbox(&mut self.show_method_a, "Method (a) — wireframe");
                ui.checkbox(&mut self.show_method_b, "Method (b) — chaos game");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Points");
                    if ui
                        .add(egui::Slider::new(&mut self.chaos_points, 200..=20_000))
                        .changed()
                    {
                        chaos_dirty = true;
                    }
                });
                if ui.button("Reseed").clicked() {
                    self.chaos_seed = self.chaos_seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    chaos_dirty = true;
                }
                ui.separator();
                ui.label("Camera");
                ui.add(egui::Slider::new(&mut self.camera.azimuth, -3.14..=3.14).text("azimuth"));
                ui.add(egui::Slider::new(&mut self.camera.elevation, -1.4..=1.4).text("elevation"));
                ui.add(egui::Slider::new(&mut self.camera.distance, 1.5..=8.0).text("zoom"));

                if chaos_dirty {
                    self.recompute_chaos();
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), Sense::drag());

            // Drag to orbit the camera.
            if response.dragged() {
                let drag = response.drag_delta();
                self.camera.azimuth -= drag.x * 0.005;
                self.camera.elevation = (self.camera.elevation + drag.y * 0.005)
                    .clamp(-1.4, 1.4);
            }

            // Scroll to zoom.
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll.abs() > f32::EPSILON {
                self.camera.distance = (self.camera.distance - scroll * 0.005).clamp(1.5, 8.0);
            }

            let rect = response.rect;
            painter.rect_filled(rect, 4.0, Color32::from_rgb(20, 22, 28));

            // World offset: centre the IFS attractor (x,y around 0.5, z around mid).
            let mid_z = -0.5 * (self.depth as f32) * self.layer_spacing;
            let to_world = |q: [f32; 2], k: u32| -> [f32; 3] {
                [
                    q[0] - 0.5,
                    q[1] - 0.5,
                    -(k as f32) * self.layer_spacing - mid_z,
                ]
            };

            // Method (a): draw all generations 0..=depth as wireframe parallelograms,
            // back-to-front so depth layering reads correctly.
            if self.show_method_a {
                let mut layers: Vec<(u32, Vec<[[f32; 2]; 4]>)> = (0..=self.depth)
                    .map(|k| (k, parallelograms_at_depth(&self.maps, k)))
                    .collect();
                // Sort by projected z so far-away layers paint first.
                layers.sort_by(|(ka, _), (kb, _)| {
                    let za = -(*ka as f32) * self.layer_spacing - mid_z;
                    let zb = -(*kb as f32) * self.layer_spacing - mid_z;
                    // Painter's algorithm: larger z (closer to camera in our convention) last.
                    za.partial_cmp(&zb).unwrap_or(std::cmp::Ordering::Equal)
                });

                for (k, polys) in layers {
                    let color = LAYER_PALETTE[(k as usize).min(LAYER_PALETTE.len() - 1)];
                    let stroke = Stroke::new(if k == 0 { 1.8 } else { 1.0 }, color);
                    for poly in polys {
                        let proj: [Pos2; 4] = [
                            self.camera.project(to_world(poly[0], k), rect),
                            self.camera.project(to_world(poly[1], k), rect),
                            self.camera.project(to_world(poly[2], k), rect),
                            self.camera.project(to_world(poly[3], k), rect),
                        ];
                        for i in 0..4 {
                            painter.line_segment([proj[i], proj[(i + 1) % 4]], stroke);
                        }
                    }
                }
            }

            // Method (b): chaos game cloud at z = -depth (in front of the stack).
            if self.show_method_b {
                let z_attractor = -(self.depth as f32 + 0.5) * self.layer_spacing - mid_z;
                let cloud_color = Color32::from_rgb(255, 248, 220);
                for q in &self.cached_chaos {
                    let p = self.camera.project([q[0] - 0.5, q[1] - 0.5, z_attractor], rect);
                    painter.circle_filled(p, 0.9, cloud_color);
                }
            }

            // Caption inside the canvas.
            painter.text(
                rect.left_top() + Vec2::new(10.0, 8.0),
                egui::Align2::LEFT_TOP,
                "Falconer 9.3 · drag to orbit · scroll to zoom",
                egui::FontId::proportional(11.0),
                Color32::from_rgb(180, 184, 196),
            );
        });

        ctx.request_repaint();
    }
}
