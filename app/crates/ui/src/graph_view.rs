//! Canvas do grafo: layout em camadas, pan (arrastar) e zoom (scroll), sem nenhuma dependência
//! externa de renderização — só o `egui::Painter`.

use std::collections::HashMap;

use egui::{Align2, Color32, FontId, Pos2, Sense, Stroke, StrokeKind, Vec2};

use crate::app::{GraphViewerApp, NodeStatus};

const NODE_W: f32 = 160.0;
const NODE_H: f32 = 44.0;
const LAYER_GAP: f32 = 70.0;
const NODE_GAP: f32 = 24.0;

fn compute_layers(nodes: &[String], edges: &[(String, String)]) -> HashMap<String, usize> {
    let mut layer: HashMap<String, usize> = nodes.iter().map(|n| (n.clone(), 0)).collect();

    // relaxação de ponto fixo: layer(alvo) = max(layer(alvo), layer(origem) + 1).
    // grafo pequeno (dezenas de nós), não precisa de nada mais esperto que isso.
    for _ in 0..nodes.len().max(1) {
        let mut changed = false;
        for (source, target) in edges {
            let Some(&layer_source) = layer.get(source) else { continue };
            let candidate = layer_source + 1;
            if let Some(layer_target) = layer.get_mut(target) {
                if candidate > *layer_target {
                    *layer_target = candidate;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    layer
}

fn status_colors(status: NodeStatus) -> (Color32, Color32) {
    match status {
        NodeStatus::Idle => (Color32::from_rgb(35, 42, 56), Color32::from_rgb(42, 49, 64)),
        NodeStatus::Active => (Color32::from_rgb(58, 42, 18), Color32::from_rgb(245, 165, 36)),
        NodeStatus::Done => (Color32::from_rgb(18, 48, 31), Color32::from_rgb(46, 204, 113)),
        NodeStatus::Error => (Color32::from_rgb(51, 20, 22), Color32::from_rgb(229, 72, 77)),
    }
}

pub fn show(ui: &mut egui::Ui, app: &mut GraphViewerApp) {
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 340.0), Sense::click_and_drag());

    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 8.0, Color32::from_rgb(13, 17, 25));
    painter.rect_stroke(
        rect,
        8.0,
        Stroke::new(1.0_f32, Color32::from_rgb(42, 49, 64)),
        StrokeKind::Inside,
    );

    if response.dragged() {
        app.pan += response.drag_delta();
    }

    if response.hovered() {
        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if scroll.abs() > 0.0 {
            let old_zoom = app.zoom;
            let factor = if scroll > 0.0 { 1.1 } else { 0.9 };
            let new_zoom = (app.zoom * factor).clamp(0.3, 2.5);
            if let Some(cursor) = response.hover_pos() {
                let anchor = cursor - rect.min - Vec2::new(rect.width() / 2.0, 30.0);
                app.pan = anchor - (anchor - app.pan) * (new_zoom / old_zoom);
            }
            app.zoom = new_zoom;
        }
    }

    if app.nodes.is_empty() {
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            &app.status_msg,
            FontId::monospace(13.0),
            Color32::GRAY,
        );
        return;
    }

    let origin = rect.min + Vec2::new(rect.width() / 2.0, 30.0) + app.pan;
    let to_screen = |p: Vec2| -> Pos2 { origin + p * app.zoom };

    let layers = compute_layers(&app.nodes, &app.edges);
    let max_layer = *layers.values().max().unwrap_or(&0);

    let mut by_layer: Vec<Vec<&String>> = vec![Vec::new(); max_layer + 1];
    for n in &app.nodes {
        by_layer[layers[n]].push(n);
    }

    let mut pos: HashMap<&str, Vec2> = HashMap::new();
    for (l, list) in by_layer.iter().enumerate() {
        let total_w = list.len() as f32 * NODE_W + list.len().saturating_sub(1) as f32 * NODE_GAP;
        let start_x = -total_w / 2.0;
        for (i, n) in list.iter().enumerate() {
            let x = start_x + i as f32 * (NODE_W + NODE_GAP) + NODE_W / 2.0;
            let y = l as f32 * (NODE_H + LAYER_GAP) + NODE_H / 2.0;
            pos.insert(n.as_str(), Vec2::new(x, y));
        }
    }

    // arestas primeiro, nós por cima
    for (source, target) in &app.edges {
        let (Some(&ps), Some(&pt)) = (pos.get(source.as_str()), pos.get(target.as_str())) else {
            continue;
        };
        let from = to_screen(ps + Vec2::new(0.0, NODE_H / 2.0));
        let to = to_screen(pt - Vec2::new(0.0, NODE_H / 2.0));
        painter.line_segment([from, to], Stroke::new(1.5_f32, Color32::from_rgb(90, 98, 115)));

        let dir = (to - from).normalized();
        let tip = to;
        let left = tip - dir * 9.0 + Vec2::new(-dir.y, dir.x) * 4.0;
        let right = tip - dir * 9.0 + Vec2::new(dir.y, -dir.x) * 4.0;
        painter.add(egui::Shape::convex_polygon(
            vec![tip, left, right],
            Color32::from_rgb(90, 98, 115),
            Stroke::NONE,
        ));
    }

    for n in &app.nodes {
        let Some(&p) = pos.get(n.as_str()) else { continue };
        let center = to_screen(p);
        let half = Vec2::new(NODE_W, NODE_H) * app.zoom / 2.0;
        let node_rect = egui::Rect::from_min_max(center - half, center + half);

        let status = app.status.get(n).copied().unwrap_or(NodeStatus::Idle);
        let (fill, border) = status_colors(status);

        painter.rect_filled(node_rect, 6.0, fill);
        painter.rect_stroke(node_rect, 6.0, Stroke::new(1.5_f32, border), StrokeKind::Inside);
        painter.text(
            center,
            Align2::CENTER_CENTER,
            n,
            FontId::monospace(12.0 * app.zoom.clamp(0.5, 1.5)),
            Color32::from_rgb(230, 233, 239),
        );
    }
}
