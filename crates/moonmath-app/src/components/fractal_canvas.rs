use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum FractalType {
    CantorSet,
    SierpinskiTriangle,
    KochCurve,
    SierpinskiCarpet,
}

/// Grid of interactive fractal visualizations for the Hausdorff Dimension page.
#[component]
pub fn FractalVisualizations() -> impl IntoView {
    view! {
        <section class="fractal-viz-section">
            <h2>"Interactive Fractal Visualizations"</h2>
            <p class="fractal-viz-subtitle">
                "Explore how iteration depth affects fractal structure and Hausdorff dimension."
            </p>
            <div class="fractal-viz-grid">
                <FractalCanvas
                    canvas_id="fractal-cantor"
                    title="Cantor Set"
                    dimension="log 2 / log 3 \u{2248} 0.631"
                    fractal_type=FractalType::CantorSet
                    max_depth=8
                />
                <FractalCanvas
                    canvas_id="fractal-sierpinski"
                    title="Sierpinski Triangle"
                    dimension="log 3 / log 2 \u{2248} 1.585"
                    fractal_type=FractalType::SierpinskiTriangle
                    max_depth=8
                />
                <FractalCanvas
                    canvas_id="fractal-koch"
                    title="Koch Curve"
                    dimension="log 4 / log 3 \u{2248} 1.262"
                    fractal_type=FractalType::KochCurve
                    max_depth=6
                />
                <FractalCanvas
                    canvas_id="fractal-carpet"
                    title="Sierpinski Carpet"
                    dimension="log 8 / log 3 \u{2248} 1.893"
                    fractal_type=FractalType::SierpinskiCarpet
                    max_depth=5
                />
            </div>
        </section>
    }
}

/// A single fractal canvas card with iteration-depth controls.
#[component]
fn FractalCanvas(
    canvas_id: &'static str,
    title: &'static str,
    dimension: &'static str,
    fractal_type: FractalType,
    max_depth: u32,
) -> impl IntoView {
    let (depth, set_depth) = signal(3u32);
    let _ = fractal_type; // used only in hydrate feature

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let d = depth.get();
            draw::render(canvas_id, fractal_type, d);
        });
    }

    view! {
        <div class="fractal-card">
            <div class="fractal-card-header">
                <h3>{title}</h3>
                <span class="fractal-dimension">"d = " {dimension}</span>
            </div>
            <canvas id=canvas_id width="360" height="280" class="fractal-canvas"/>
            <div class="fractal-controls">
                <button
                    class="fractal-btn"
                    on:click=move |_| set_depth.update(|d| *d = d.saturating_sub(1))
                    disabled=move || depth.get() == 0
                >"\u{2212}"</button>
                <span class="depth-label">"Depth: " {move || depth.get()}</span>
                <button
                    class="fractal-btn"
                    on:click=move |_| set_depth.update(|d| *d = (*d + 1).min(max_depth))
                    disabled=move || depth.get() >= max_depth
                >"+"</button>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Canvas drawing (client / WASM only)
// ---------------------------------------------------------------------------

#[cfg(feature = "hydrate")]
mod draw {
    use super::FractalType;
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;

    const COLOR_BG: &str = "#0b1a2e";
    const COLOR_ACCENT: &str = "#f4e9cd";

    pub fn render(canvas_id: &str, fractal_type: FractalType, depth: u32) {
        let Some(ctx) = canvas_ctx(canvas_id) else { return };
        let canvas: web_sys::HtmlCanvasElement = ctx
            .canvas()
            .unwrap()
            .unchecked_into();
        let w = canvas.width() as f64;
        let h = canvas.height() as f64;

        // Clear
        ctx.set_fill_style_str(COLOR_BG);
        ctx.fill_rect(0.0, 0.0, w, h);

        // Draw
        ctx.set_fill_style_str(COLOR_ACCENT);
        ctx.set_stroke_style_str(COLOR_ACCENT);

        match fractal_type {
            FractalType::CantorSet => cantor_set(&ctx, w, h, depth),
            FractalType::SierpinskiTriangle => sierpinski_triangle(&ctx, w, h, depth),
            FractalType::KochCurve => koch_curve(&ctx, w, h, depth),
            FractalType::SierpinskiCarpet => sierpinski_carpet(&ctx, w, h, depth),
        }
    }

    fn canvas_ctx(id: &str) -> Option<CanvasRenderingContext2d> {
        let document = web_sys::window()?.document()?;
        let el = document.get_element_by_id(id)?;
        let canvas: web_sys::HtmlCanvasElement = el.unchecked_into();
        canvas
            .get_context("2d")
            .ok()?
            .map(|obj| obj.unchecked_into())
    }

    // ── Cantor Set ──────────────────────────────────────────────────────

    fn cantor_set(ctx: &CanvasRenderingContext2d, w: f64, h: f64, depth: u32) {
        let pad = 20.0;
        let bar_h = 6.0;
        let levels = (depth + 1) as f64;
        let gap = ((h - 2.0 * pad) / levels).min(30.0);
        cantor_rec(ctx, pad, pad, w - 2.0 * pad, bar_h, gap, 0, depth);
    }

    fn cantor_rec(
        ctx: &CanvasRenderingContext2d,
        x: f64, y: f64, len: f64, bar_h: f64, gap: f64,
        level: u32, max: u32,
    ) {
        ctx.fill_rect(x, y, len, bar_h);
        if level < max {
            let third = len / 3.0;
            cantor_rec(ctx, x, y + gap, third, bar_h, gap, level + 1, max);
            cantor_rec(ctx, x + 2.0 * third, y + gap, third, bar_h, gap, level + 1, max);
        }
    }

    // ── Sierpinski Triangle ─────────────────────────────────────────────

    fn sierpinski_triangle(ctx: &CanvasRenderingContext2d, w: f64, h: f64, depth: u32) {
        let pad = 15.0;
        let usable_w = w - 2.0 * pad;
        let usable_h = h - 2.0 * pad;
        // Equilateral triangle fitting the canvas
        let side = usable_w.min(usable_h / 0.866);
        let cx = w / 2.0;
        let top_y = pad + (usable_h - side * 0.866) / 2.0;

        let ax = cx;
        let ay = top_y;
        let bx = cx - side / 2.0;
        let by = top_y + side * 0.866;
        let cxx = cx + side / 2.0;
        let cy = by;

        sierpinski_rec(ctx, ax, ay, bx, by, cxx, cy, depth);
    }

    fn sierpinski_rec(
        ctx: &CanvasRenderingContext2d,
        ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64,
        depth: u32,
    ) {
        if depth == 0 {
            ctx.begin_path();
            ctx.move_to(ax, ay);
            ctx.line_to(bx, by);
            ctx.line_to(cx, cy);
            ctx.close_path();
            ctx.fill();
        } else {
            let abx = (ax + bx) / 2.0;
            let aby = (ay + by) / 2.0;
            let acx = (ax + cx) / 2.0;
            let acy = (ay + cy) / 2.0;
            let bcx = (bx + cx) / 2.0;
            let bcy = (by + cy) / 2.0;
            sierpinski_rec(ctx, ax, ay, abx, aby, acx, acy, depth - 1);
            sierpinski_rec(ctx, abx, aby, bx, by, bcx, bcy, depth - 1);
            sierpinski_rec(ctx, acx, acy, bcx, bcy, cx, cy, depth - 1);
        }
    }

    // ── Koch Curve ──────────────────────────────────────────────────────

    fn koch_curve(ctx: &CanvasRenderingContext2d, w: f64, h: f64, depth: u32) {
        let pad = 20.0;
        let y_pos = h * 0.65;
        ctx.set_line_width(1.5);

        let points = koch_points(pad, y_pos, w - pad, y_pos, depth);
        if points.is_empty() {
            return;
        }
        ctx.begin_path();
        ctx.move_to(points[0].0, points[0].1);
        for &(x, y) in &points[1..] {
            ctx.line_to(x, y);
        }
        ctx.stroke();
    }

    fn koch_points(x1: f64, y1: f64, x2: f64, y2: f64, depth: u32) -> Vec<(f64, f64)> {
        if depth == 0 {
            return vec![(x1, y1), (x2, y2)];
        }
        let dx = x2 - x1;
        let dy = y2 - y1;
        // 1/3 and 2/3 points
        let ax = x1 + dx / 3.0;
        let ay = y1 + dy / 3.0;
        let bx = x1 + 2.0 * dx / 3.0;
        let by = y1 + 2.0 * dy / 3.0;
        // Peak: rotate AB by -60° around A
        let dab_x = bx - ax;
        let dab_y = by - ay;
        let px = ax + 0.5 * dab_x + 0.866 * dab_y;
        let py = ay - 0.866 * dab_x + 0.5 * dab_y;

        let mut pts = koch_points(x1, y1, ax, ay, depth - 1);
        pts.pop();
        pts.extend(koch_points(ax, ay, px, py, depth - 1));
        pts.pop();
        pts.extend(koch_points(px, py, bx, by, depth - 1));
        pts.pop();
        pts.extend(koch_points(bx, by, x2, y2, depth - 1));
        pts
    }

    // ── Sierpinski Carpet ───────────────────────────────────────────────

    fn sierpinski_carpet(ctx: &CanvasRenderingContext2d, w: f64, h: f64, depth: u32) {
        let pad = 15.0;
        let side = (w - 2.0 * pad).min(h - 2.0 * pad);
        let x0 = (w - side) / 2.0;
        let y0 = (h - side) / 2.0;

        // Draw filled base square
        ctx.fill_rect(x0, y0, side, side);

        // Punch holes with the background color
        ctx.set_fill_style_str(COLOR_BG);
        carpet_rec(ctx, x0, y0, side, depth);
    }

    fn carpet_rec(ctx: &CanvasRenderingContext2d, x: f64, y: f64, side: f64, depth: u32) {
        if depth == 0 {
            return;
        }
        let s = side / 3.0;
        // Remove center square
        ctx.fill_rect(x + s, y + s, s, s);
        // Recurse into the 8 surrounding sub-squares
        for row in 0..3u32 {
            for col in 0..3u32 {
                if row == 1 && col == 1 {
                    continue;
                }
                carpet_rec(ctx, x + col as f64 * s, y + row as f64 * s, s, depth - 1);
            }
        }
    }
}
