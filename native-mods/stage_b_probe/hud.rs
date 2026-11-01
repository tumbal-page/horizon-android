// Stage B probe: a box in the top-left corner shows this native bundle is part of the build.
fn stage_b_probe_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use horizon_client::extras::Corner;
    if !game.native_corner_shown(Corner::TopLeft) {
        return;
    }
    let mut lines = vec![String::from("Stage B: native mod active")];
    if let Some(p) = game.native_player() {
        lines.push(format!("XYZ {:.0} / {:.0} / {:.0}", p.x, p.y, p.z));
    }
    let w = lines.iter().map(|l| game.native_text_width(l)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::TopLeft, w, h);
    game.native_fill(batch, [x, y, w, h], 0x7800_0000);
    for (i, line) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], 0xFF_FFFF, true);
    }
}
