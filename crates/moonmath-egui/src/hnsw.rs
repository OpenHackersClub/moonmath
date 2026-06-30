//! Interactive visualization of a Hierarchical Navigable Small World (HNSW)
//! approximate-nearest-neighbour index.
//!
//! HNSW stacks several proximity graphs. Layer 0 holds every point; each higher
//! layer keeps an exponentially thinner random sample, so the top layers give
//! coarse long-range hops and the bottom layer gives fine local resolution
//! (the same idea as a skip list, lifted onto a graph). A query greedily
//! descends the stack: at each layer it walks to the neighbour closest to the
//! query, then drops to the same node one layer down and repeats; the bottom
//! layer runs a wider beam search of width `ef`.
//!
//! We illustrate this in 2-D — each "vector" is a point in the unit square — and
//! software-project the layer stack to a 2.5-D scene drawn with egui's plain 2-D
//! `Painter`, matching the `ifs_3d` module's approach (no glow/wgpu).
//!
//! The construction here is deliberately simplified: each layer's graph is the
//! symmetric `M`-nearest-neighbour graph over that layer's members (`M0 = 2M` at
//! layer 0), rather than the paper's incremental heuristic insertion. The search
//! procedure — greedy descent plus an `ef`-width base-layer beam — is faithful.

use eframe::egui;
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, Vec2};

/// Cap the layer count so the stack stays legible; higher levels are vanishingly
/// rare for the point counts we use anyway.
const MAX_LEVEL_CAP: usize = 5;

const LAYER_PALETTE: [Color32; 6] = [
    Color32::from_rgb(96, 165, 250),  // layer 0 — base, all points
    Color32::from_rgb(52, 211, 153),
    Color32::from_rgb(251, 191, 36),
    Color32::from_rgb(244, 114, 182),
    Color32::from_rgb(167, 139, 250),
    Color32::from_rgb(248, 113, 113),
];

fn layer_color(layer: usize) -> Color32 {
    LAYER_PALETTE[layer.min(LAYER_PALETTE.len() - 1)]
}

/// Squared Euclidean distance — we only ever compare distances, so the sqrt is
/// unnecessary except for display.
fn dist2(a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    dx * dx + dy * dy
}

/// Tiny xorshift PRNG — deterministic, no `getrandom`/`rand` dependency so the
/// WASM build stays lean.
struct Rng(u64);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 as u32
    }
    /// Uniform in the open interval (0, 1).
    fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32 + 1.0) / (u32::MAX as f32 + 2.0)
    }
}

struct Node {
    pos: [f32; 2],
    level: usize,
}

/// A built HNSW index over a fixed point set.
struct Hnsw {
    nodes: Vec<Node>,
    /// `adj[layer][node]` = neighbour indices of `node` within `layer`; empty for
    /// nodes not present in that layer.
    adj: Vec<Vec<Vec<usize>>>,
    /// `members[layer]` = node indices present at `layer`.
    members: Vec<Vec<usize>>,
    entry: usize,
    max_level: usize,
}

impl Hnsw {
    fn build(n: usize, m: usize, ml: f32, seed: u64) -> Self {
        let mut rng = Rng(seed | 1);

        // Random points = the "vectors".
        let mut nodes: Vec<Node> = (0..n)
            .map(|_| {
                let pos = [rng.next_f32(), rng.next_f32()];
                // level = floor(-ln(u) * mL): geometric decay, exactly as HNSW.
                let level = (-(rng.next_f32().ln()) * ml).floor() as usize;
                Node {
                    pos,
                    level: level.min(MAX_LEVEL_CAP),
                }
            })
            .collect();

        // Guarantee at least one node sits at every level up to the max so the
        // stack is never disconnected by an empty middle layer.
        let max_level = nodes.iter().map(|nd| nd.level).max().unwrap_or(0);
        let entry = nodes
            .iter()
            .enumerate()
            .max_by_key(|(_, nd)| nd.level)
            .map(|(i, _)| i)
            .unwrap_or(0);
        // Promote the entry node to span the full stack (it always does, being
        // the argmax, but make it explicit).
        if !nodes.is_empty() {
            nodes[entry].level = max_level;
        }

        let members: Vec<Vec<usize>> = (0..=max_level)
            .map(|l| {
                (0..nodes.len())
                    .filter(|&i| nodes[i].level >= l)
                    .collect()
            })
            .collect();

        // Per-layer symmetric kNN graph.
        let mut adj: Vec<Vec<Vec<usize>>> = (0..=max_level)
            .map(|_| vec![Vec::new(); nodes.len()])
            .collect();

        for (l, layer_members) in members.iter().enumerate() {
            let degree = if l == 0 { m * 2 } else { m };
            for &i in layer_members {
                // nearest `degree` other members of this layer
                let mut cands: Vec<(f32, usize)> = layer_members
                    .iter()
                    .filter(|&&j| j != i)
                    .map(|&j| (dist2(nodes[i].pos, nodes[j].pos), j))
                    .collect();
                cands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
                for &(_, j) in cands.iter().take(degree) {
                    if !adj[l][i].contains(&j) {
                        adj[l][i].push(j);
                    }
                    // symmetrise
                    if !adj[l][j].contains(&i) {
                        adj[l][j].push(i);
                    }
                }
            }
        }

        Hnsw {
            nodes,
            adj,
            members,
            entry,
            max_level,
        }
    }

    fn nearest_brute(&self, query: [f32; 2]) -> usize {
        (0..self.nodes.len())
            .min_by(|&a, &b| {
                dist2(self.nodes[a].pos, query)
                    .partial_cmp(&dist2(self.nodes[b].pos, query))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }

    /// Greedy descent + `ef`-width base-layer beam. Records every node touched so
    /// the UI can replay the walk step by step.
    fn search(&self, query: [f32; 2], ef: usize) -> SearchTrace {
        let n = self.nodes.len();
        let mut steps: Vec<SearchStep> = Vec::new();
        let mut dc = 0usize; // distance computations
        let d = |i: usize| dist2(self.nodes[i].pos, query);

        if n == 0 {
            return SearchTrace {
                query,
                steps,
                result: 0,
                true_nn: 0,
                dist_computations: 0,
            };
        }

        let mut cur = self.entry;
        steps.push(SearchStep {
            node: cur,
            layer: self.max_level,
        });

        // Coarse layers: pure greedy (ef = 1).
        for l in (1..=self.max_level).rev() {
            if steps.last().map(|s| s.layer != l).unwrap_or(true) {
                steps.push(SearchStep { node: cur, layer: l });
            }
            loop {
                let mut best = cur;
                let mut best_d = d(cur);
                for &nb in &self.adj[l][cur] {
                    let dd = d(nb);
                    dc += 1;
                    if dd < best_d {
                        best_d = dd;
                        best = nb;
                    }
                }
                if best == cur {
                    break;
                }
                cur = best;
                steps.push(SearchStep { node: cur, layer: l });
            }
        }

        // Base layer: ef-width beam search.
        let ef = ef.max(1);
        let mut visited = vec![false; n];
        let mut candidates: Vec<usize> = vec![cur];
        let mut top: Vec<usize> = vec![cur];
        visited[cur] = true;
        if steps.last().map(|s| s.layer != 0 || s.node != cur).unwrap_or(true) {
            steps.push(SearchStep { node: cur, layer: 0 });
        }

        let farthest_d = |top: &[usize]| -> f32 {
            top.iter()
                .map(|&i| d(i))
                .fold(f32::MIN, |acc, x| if x > acc { x } else { acc })
        };

        while !candidates.is_empty() {
            // nearest unexpanded candidate
            let ci = candidates
                .iter()
                .enumerate()
                .min_by(|a, b| d(*a.1).partial_cmp(&d(*b.1)).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx)
                .unwrap();
            let c = candidates.swap_remove(ci);
            let c_d = d(c);
            dc += 1;
            if top.len() >= ef && c_d > farthest_d(&top) {
                break;
            }
            for &e in &self.adj[0][c] {
                if visited[e] {
                    continue;
                }
                visited[e] = true;
                let e_d = d(e);
                dc += 1;
                steps.push(SearchStep { node: e, layer: 0 });
                if top.len() < ef || e_d < farthest_d(&top) {
                    candidates.push(e);
                    top.push(e);
                    if top.len() > ef {
                        // drop the current farthest
                        let fi = top
                            .iter()
                            .enumerate()
                            .max_by(|a, b| {
                                d(*a.1).partial_cmp(&d(*b.1)).unwrap_or(std::cmp::Ordering::Equal)
                            })
                            .map(|(idx, _)| idx)
                            .unwrap();
                        top.swap_remove(fi);
                    }
                }
            }
        }

        let result = top
            .iter()
            .copied()
            .min_by(|&a, &b| d(a).partial_cmp(&d(b)).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(cur);
        let true_nn = self.nearest_brute(query);

        SearchTrace {
            query,
            steps,
            result,
            true_nn,
            dist_computations: dc,
        }
    }
}

#[derive(Clone, Copy)]
struct SearchStep {
    node: usize,
    layer: usize,
}

struct SearchTrace {
    query: [f32; 2],
    steps: Vec<SearchStep>,
    result: usize,
    true_nn: usize,
    dist_computations: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Stack,
    Flat,
}

/// Orbit camera for the 2.5-D stack view.
#[derive(Clone, Copy)]
struct Camera {
    azimuth: f32,
    elevation: f32,
    distance: f32,
}

impl Camera {
    fn project(&self, p: [f32; 3], screen: Rect) -> Pos2 {
        let (sa, ca) = self.azimuth.sin_cos();
        let (se, ce) = self.elevation.sin_cos();
        let (x, y, z) = (p[0], p[1], p[2]);
        let xr = ca * x + sa * y;
        let yr = -sa * x + ca * y;
        let yc = ce * yr - se * z;
        let zc = se * yr + ce * z;

        let depth = (self.distance + zc).max(0.05);
        let f = self.distance / depth;
        let cx = screen.center().x;
        let cy = screen.center().y;
        let scale = screen.size().min_elem() * 0.46;
        Pos2::new(cx + xr * scale * f, cy - yc * scale * f)
    }
}

pub struct HnswApp {
    // index parameters
    n: usize,
    m: usize,
    ml: f32,
    seed: u64,
    // query
    query: [f32; 2],
    query_seed: u64,
    ef: usize,
    // built state
    index: Hnsw,
    trace: SearchTrace,
    // view
    view: ViewMode,
    flat_layer: usize,
    layer_spacing: f32,
    camera: Camera,
    show_edges: bool,
    show_links: bool,
    show_search: bool,
    // animation
    playing: bool,
    cursor: usize,
    anim_accum: f32,
    step_interval: f32,
}

impl Default for HnswApp {
    fn default() -> Self {
        let n = 120;
        let m = 6;
        let ml = 0.62;
        let seed = 0x5EED_1234;
        let query = [0.42, 0.55];
        let ef = 8;
        let index = Hnsw::build(n, m, ml, seed);
        let trace = index.search(query, ef);
        Self {
            n,
            m,
            ml,
            seed,
            query,
            query_seed: 0xA11CE,
            ef,
            index,
            trace,
            view: ViewMode::Stack,
            flat_layer: 0,
            layer_spacing: 0.62,
            camera: Camera {
                azimuth: 0.62,
                elevation: 0.55,
                distance: 3.2,
            },
            show_edges: true,
            show_links: true,
            show_search: true,
            playing: true,
            cursor: 0,
            anim_accum: 0.0,
            step_interval: 0.18,
        }
    }
}

impl HnswApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn rebuild_index(&mut self) {
        self.index = Hnsw::build(self.n, self.m, self.ml, self.seed);
        self.flat_layer = self.flat_layer.min(self.index.max_level);
        self.recompute_trace();
    }

    fn recompute_trace(&mut self) {
        self.trace = self.index.search(self.query, self.ef);
        self.cursor = 0;
        self.anim_accum = 0.0;
        self.playing = true;
    }

    fn new_query(&mut self) {
        let mut rng = Rng(self.query_seed | 1);
        // advance the seed so each click yields a fresh point
        self.query_seed = self.query_seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.query = [rng.next_f32(), rng.next_f32()];
        self.recompute_trace();
    }
}

impl eframe::App for HnswApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut index_dirty = false;
        let mut query_dirty = false;

        egui::SidePanel::right("hnsw_controls")
            .resizable(false)
            .default_width(248.0)
            .show(ctx, |ui| {
                ui.heading("HNSW — ANN search");
                ui.label(
                    "A stack of proximity graphs. Higher layers keep an \
                     exponentially thinner sample for long hops; layer 0 holds \
                     every point for fine search.",
                );
                ui.separator();

                ui.label("Index");
                ui.horizontal(|ui| {
                    ui.label("Points N");
                    if ui.add(egui::Slider::new(&mut self.n, 20..=240)).changed() {
                        index_dirty = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Neighbours M");
                    if ui.add(egui::Slider::new(&mut self.m, 2..=16)).changed() {
                        index_dirty = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Level mult mL");
                    if ui
                        .add(egui::Slider::new(&mut self.ml, 0.2..=1.6))
                        .changed()
                    {
                        index_dirty = true;
                    }
                });
                ui.label(
                    egui::RichText::new("level = ⌊-ln(u)·mL⌋   ·   heuristic mL = 1/ln(M)")
                        .small()
                        .weak(),
                );
                if ui.button("Rebuild (reseed)").clicked() {
                    self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    index_dirty = true;
                }
                ui.label(
                    egui::RichText::new(format!(
                        "layers: {}   ·   entry node #{}",
                        self.index.max_level + 1,
                        self.index.entry
                    ))
                    .small()
                    .weak(),
                );

                ui.separator();
                ui.label("Query");
                ui.horizontal(|ui| {
                    ui.label("Beam ef");
                    if ui.add(egui::Slider::new(&mut self.ef, 1..=32)).changed() {
                        query_dirty = true;
                    }
                });
                if ui.button("New query point").clicked() {
                    self.new_query();
                }

                ui.separator();
                ui.label("Search replay");
                ui.horizontal(|ui| {
                    let label = if self.playing { "⏸ Pause" } else { "▶ Play" };
                    if ui.button(label).clicked() {
                        if self.cursor >= self.trace.steps.len() {
                            self.cursor = 0;
                        }
                        self.playing = !self.playing;
                    }
                    if ui.button("Step").clicked() {
                        self.playing = false;
                        if self.cursor < self.trace.steps.len() {
                            self.cursor += 1;
                        }
                    }
                    if ui.button("Reset").clicked() {
                        self.cursor = 0;
                        self.anim_accum = 0.0;
                        self.playing = true;
                    }
                });
                ui.add(
                    egui::Slider::new(&mut self.step_interval, 0.02..=0.6)
                        .text("sec/step")
                        .logarithmic(true),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "step {} / {}",
                        self.cursor,
                        self.trace.steps.len()
                    ))
                    .small(),
                );

                ui.separator();
                ui.label("View");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.view, ViewMode::Stack, "Stack 2.5D");
                    ui.selectable_value(&mut self.view, ViewMode::Flat, "Flat layer");
                });
                if self.view == ViewMode::Flat {
                    ui.add(
                        egui::Slider::new(&mut self.flat_layer, 0..=self.index.max_level)
                            .text("layer"),
                    );
                } else {
                    ui.add(
                        egui::Slider::new(&mut self.layer_spacing, 0.25..=1.2).text("layer gap"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.camera.distance, 1.8..=7.0).text("zoom"),
                    );
                }
                ui.checkbox(&mut self.show_edges, "show edges");
                if self.view == ViewMode::Stack {
                    ui.checkbox(&mut self.show_links, "show layer links");
                }
                ui.checkbox(&mut self.show_search, "show search path");

                ui.separator();
                // Outcome readout.
                let exact = self.trace.result == self.trace.true_nn;
                let n = self.index.nodes.len().max(1);
                ui.label(
                    egui::RichText::new(format!(
                        "distances evaluated: {} / {} (brute force)",
                        self.trace.dist_computations, n
                    ))
                    .small(),
                );
                if self.cursor >= self.trace.steps.len() {
                    if exact {
                        ui.colored_label(
                            Color32::from_rgb(52, 211, 153),
                            "✓ found the exact nearest neighbour",
                        );
                    } else {
                        ui.colored_label(
                            Color32::from_rgb(251, 191, 36),
                            "≈ approximate result (ring = found, dashed = true NN)",
                        );
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::drag());
            let rect = response.rect;
            painter.rect_filled(rect, 4.0, Color32::from_rgb(18, 20, 26));

            // Orbit / zoom (stack view only).
            if self.view == ViewMode::Stack {
                if response.dragged() {
                    let drag = response.drag_delta();
                    self.camera.azimuth -= drag.x * 0.005;
                    self.camera.elevation = (self.camera.elevation + drag.y * 0.005).clamp(0.05, 1.45);
                }
                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll.abs() > f32::EPSILON {
                    self.camera.distance = (self.camera.distance - scroll * 0.005).clamp(1.8, 7.0);
                }
            }

            match self.view {
                ViewMode::Stack => self.draw_stack(&painter, rect),
                ViewMode::Flat => self.draw_flat(&painter, rect),
            }

            let caption = match self.view {
                ViewMode::Stack => "HNSW stack · drag to orbit · scroll to zoom",
                ViewMode::Flat => "HNSW layer · greedy walk + ef-beam at layer 0",
            };
            painter.text(
                rect.left_top() + Vec2::new(10.0, 8.0),
                Align2::LEFT_TOP,
                caption,
                FontId::proportional(11.0),
                Color32::from_rgb(170, 176, 190),
            );
        });

        // Advance the replay.
        if index_dirty {
            self.rebuild_index();
        } else if query_dirty {
            self.recompute_trace();
        }
        if self.playing && !self.trace.steps.is_empty() {
            self.anim_accum += ctx.input(|i| i.stable_dt).min(0.1);
            while self.anim_accum >= self.step_interval && self.cursor < self.trace.steps.len() {
                self.anim_accum -= self.step_interval;
                self.cursor += 1;
            }
            if self.cursor >= self.trace.steps.len() {
                self.playing = false;
            }
        }

        ctx.request_repaint();
    }
}

impl HnswApp {
    fn revealed(&self) -> &[SearchStep] {
        let k = self.cursor.min(self.trace.steps.len());
        &self.trace.steps[..k]
    }

    fn draw_stack(&self, painter: &egui::Painter, rect: Rect) {
        let max_level = self.index.max_level;
        let mid_z = 0.5 * (max_level as f32) * self.layer_spacing;
        let project = |q: [f32; 2], layer: usize| -> Pos2 {
            self.camera.project(
                [q[0] - 0.5, q[1] - 0.5, layer as f32 * self.layer_spacing - mid_z],
                rect,
            )
        };

        // Draw bottom-to-top so upper layers paint over lower ones.
        for l in 0..=max_level {
            let color = layer_color(l);
            // Layer plane border.
            let corners = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
            let proj: Vec<Pos2> = corners.iter().map(|&c| project(c, l)).collect();
            let plane_stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(120, 130, 150, 60));
            for i in 0..4 {
                painter.line_segment([proj[i], proj[(i + 1) % 4]], plane_stroke);
            }
            // Layer caption.
            painter.text(
                project([1.0, 0.0], l) + Vec2::new(6.0, 0.0),
                Align2::LEFT_CENTER,
                format!("L{}", l),
                FontId::monospace(11.0),
                color,
            );

            // Edges.
            if self.show_edges {
                let edge_stroke = Stroke::new(
                    0.7,
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 55),
                );
                for &i in &self.index.members[l] {
                    let pi = project(self.index.nodes[i].pos, l);
                    for &j in &self.index.adj[l][i] {
                        if i < j {
                            let pj = project(self.index.nodes[j].pos, l);
                            painter.line_segment([pi, pj], edge_stroke);
                        }
                    }
                }
            }

            // Nodes.
            for &i in &self.index.members[l] {
                let p = project(self.index.nodes[i].pos, l);
                painter.circle_filled(p, 2.4, color);
            }
        }

        // Vertical links connecting a node across the layers it lives in.
        if self.show_links {
            let link_stroke =
                Stroke::new(0.8, Color32::from_rgba_unmultiplied(160, 170, 190, 70));
            for (i, nd) in self.index.nodes.iter().enumerate() {
                if nd.level == 0 {
                    continue;
                }
                for l in 0..nd.level {
                    let a = project(nd.pos, l);
                    let b = project(nd.pos, l + 1);
                    painter.line_segment([a, b], link_stroke);
                    let _ = i;
                }
            }
        }

        if self.show_search {
            let steps = self.revealed();
            // Walk path through the stack.
            let path_stroke = Stroke::new(2.2, Color32::from_rgb(255, 255, 255));
            for w in steps.windows(2) {
                let a = project(self.index.nodes[w[0].node].pos, w[0].layer);
                let b = project(self.index.nodes[w[1].node].pos, w[1].layer);
                painter.line_segment([a, b], path_stroke);
            }
            // Visited nodes emphasised.
            for s in steps {
                let p = project(self.index.nodes[s.node].pos, s.layer);
                painter.circle_filled(p, 3.4, Color32::from_rgb(255, 255, 255));
            }
            // Current node ring.
            if let Some(s) = steps.last() {
                let p = project(self.index.nodes[s.node].pos, s.layer);
                painter.circle_stroke(p, 6.0, Stroke::new(2.0, Color32::from_rgb(56, 189, 248)));
                // Query marker at the current search layer + a faint full-height guide.
                let q_top = project(self.trace.query, max_level);
                let q_bot = project(self.trace.query, 0);
                painter.line_segment(
                    [q_top, q_bot],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(236, 72, 153, 120)),
                );
                let q = project(self.trace.query, s.layer);
                draw_diamond(painter, q, 5.0, Color32::from_rgb(236, 72, 153));
            }
            // Result + true NN rings once the walk completes.
            if self.cursor >= self.trace.steps.len() {
                let r = project(self.index.nodes[self.trace.result].pos, 0);
                painter.circle_stroke(r, 7.5, Stroke::new(2.4, Color32::from_rgb(250, 204, 21)));
                if self.trace.true_nn != self.trace.result {
                    let t = project(self.index.nodes[self.trace.true_nn].pos, 0);
                    draw_dashed_ring(painter, t, 7.5, Color32::from_rgb(52, 211, 153));
                }
            }
        }
    }

    fn draw_flat(&self, painter: &egui::Painter, rect: Rect) {
        let l = self.flat_layer.min(self.index.max_level);
        let color = layer_color(l);
        let pad = 28.0;
        let area = Rect::from_min_max(
            rect.min + Vec2::new(pad, pad),
            rect.max - Vec2::new(pad, pad),
        );
        let side = area.size().min_elem();
        let origin = area.center() - Vec2::new(side, side) * 0.5;
        let map = |q: [f32; 2]| -> Pos2 { origin + Vec2::new(q[0] * side, (1.0 - q[1]) * side) };

        // Frame.
        painter.rect_stroke(
            Rect::from_min_size(origin, Vec2::splat(side)),
            2.0,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(120, 130, 150, 90)),
        );
        painter.text(
            origin + Vec2::new(4.0, -2.0),
            Align2::LEFT_BOTTOM,
            format!(
                "Layer {} — {} of {} points",
                l,
                self.index.members[l].len(),
                self.index.nodes.len()
            ),
            FontId::monospace(12.0),
            color,
        );

        if self.show_edges {
            let edge_stroke = Stroke::new(
                0.8,
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 70),
            );
            for &i in &self.index.members[l] {
                let pi = map(self.index.nodes[i].pos);
                for &j in &self.index.adj[l][i] {
                    if i < j {
                        painter.line_segment([pi, map(self.index.nodes[j].pos)], edge_stroke);
                    }
                }
            }
        }
        for &i in &self.index.members[l] {
            painter.circle_filled(map(self.index.nodes[i].pos), 3.0, color);
        }

        if self.show_search {
            let steps: Vec<&SearchStep> =
                self.revealed().iter().filter(|s| s.layer == l).collect();
            let path_stroke = Stroke::new(2.0, Color32::from_rgb(255, 255, 255));
            for w in steps.windows(2) {
                painter.line_segment(
                    [
                        map(self.index.nodes[w[0].node].pos),
                        map(self.index.nodes[w[1].node].pos),
                    ],
                    path_stroke,
                );
            }
            for s in &steps {
                painter.circle_filled(
                    map(self.index.nodes[s.node].pos),
                    4.2,
                    Color32::from_rgb(255, 255, 255),
                );
            }
            if let Some(s) = self.revealed().last() {
                if s.layer == l {
                    painter.circle_stroke(
                        map(self.index.nodes[s.node].pos),
                        7.0,
                        Stroke::new(2.0, Color32::from_rgb(56, 189, 248)),
                    );
                }
            }
            // Query marker.
            draw_diamond(painter, map(self.trace.query), 6.0, Color32::from_rgb(236, 72, 153));
            // Result / true NN on layer 0.
            if l == 0 && self.cursor >= self.trace.steps.len() {
                painter.circle_stroke(
                    map(self.index.nodes[self.trace.result].pos),
                    9.0,
                    Stroke::new(2.4, Color32::from_rgb(250, 204, 21)),
                );
                if self.trace.true_nn != self.trace.result {
                    draw_dashed_ring(
                        painter,
                        map(self.index.nodes[self.trace.true_nn].pos),
                        9.0,
                        Color32::from_rgb(52, 211, 153),
                    );
                }
            }
        }
    }
}

fn draw_diamond(painter: &egui::Painter, c: Pos2, r: f32, color: Color32) {
    let pts = [
        c + Vec2::new(0.0, -r),
        c + Vec2::new(r, 0.0),
        c + Vec2::new(0.0, r),
        c + Vec2::new(-r, 0.0),
    ];
    let stroke = Stroke::new(2.0, color);
    for i in 0..4 {
        painter.line_segment([pts[i], pts[(i + 1) % 4]], stroke);
    }
}

fn draw_dashed_ring(painter: &egui::Painter, c: Pos2, r: f32, color: Color32) {
    let segments = 16;
    let stroke = Stroke::new(2.0, color);
    for k in 0..segments {
        if k % 2 != 0 {
            continue;
        }
        let a0 = (k as f32) / (segments as f32) * std::f32::consts::TAU;
        let a1 = (k as f32 + 1.0) / (segments as f32) * std::f32::consts::TAU;
        painter.line_segment(
            [
                c + Vec2::new(a0.cos() * r, a0.sin() * r),
                c + Vec2::new(a1.cos() * r, a1.sin() * r),
            ],
            stroke,
        );
    }
}
