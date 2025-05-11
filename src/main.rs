// src/main.rs
//use eframe::{self, App, CreationContext};
//use ui::FretMeNotApp; // ui.rs에서 정의한 FretMeNotApp 사용
//use ui::setup_fonts;  // ui.rs에서 정의한 폰트 설정 함수 사용

#![windows_subsystem = "windows"]

mod ui; // ui.rs 파일을 모듈로 선언

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 600.0]) // 창 크기 설정
            .with_min_inner_size([1200.0, 600.0]), // 최소 크기 설정
        ..Default::default()
    };

    eframe::run_native(
        "FretMeNot",
        options,
        Box::new(|cc| {
            ui::setup_fonts(cc);
            Box::new(ui::FretMeNotApp::default())
        }),
    ).unwrap();
}
